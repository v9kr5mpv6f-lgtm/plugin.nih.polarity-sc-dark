use atomic_float::AtomicF32;
use nih_plug::prelude::{util, Buffer};
use std::sync::atomic::{AtomicU8, Ordering};

pub(crate) const MATCH_DURATION_SECONDS: f32 = 5.0;
pub(crate) const OUTPUT_GAIN_MIN_DB: f32 = -50.0;
pub(crate) const OUTPUT_GAIN_MAX_DB: f32 = 50.0;

const SILENCE_FLOOR: f64 = 1.0e-12;

/// State machine driving the level match handoff between the GUI and the audio thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum MatchState {
    Idle = 0,
    Requested = 1,
    Running = 2,
    Ready = 3,
    Failed = 4,
}

/// Final outcome of a match request, consumed by the GUI.
#[derive(Debug)]
pub(crate) enum MatchResult {
    Matched(f32),
    Failed,
}

/// Lock-free request/result handoff used to trigger output gain matching from the GUI
/// and publish the measured gain back from the audio thread.
pub(crate) struct MatchRuntime {
    state: AtomicU8,
    output_gain: AtomicF32,
}

impl MatchRuntime {
    pub(crate) fn new() -> Self {
        Self {
            state: AtomicU8::new(MatchState::Idle as u8),
            output_gain: AtomicF32::new(1.0),
        }
    }

    /// Requests a new match run, unless one is already in flight. The audio thread will pick it up
    /// on its next block. Returns whether a run was started, so a second click while measuring
    /// does not spawn a second poller that would later report a bogus failure.
    ///
    /// A finished result nobody collected (its editor went away) does not block a new request; it
    /// is discarded.
    pub(crate) fn try_request(&self) -> bool {
        let started = self
            .state
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |state| {
                if state == MatchState::Requested as u8 || state == MatchState::Running as u8 {
                    None
                } else {
                    Some(MatchState::Requested as u8)
                }
            })
            .is_ok();
        if started {
            self.output_gain.store(1.0, Ordering::Release);
        }

        started
    }

    /// Gives up on a run that produced no result in time (the host is stopped, say) so the next
    /// click can start a new one. A result that raced in is left for [`take_finished_result()`].
    pub(crate) fn abandon(&self) {
        for in_flight in [MatchState::Requested, MatchState::Running] {
            let _ = self.state.compare_exchange(
                in_flight as u8,
                MatchState::Idle as u8,
                Ordering::AcqRel,
                Ordering::Acquire,
            );
        }
    }

    /// Transitions `Requested` to `Running` on the audio thread. Only the caller that wins
    /// the CAS gets `true` back and should actually run the measurement.
    pub(crate) fn begin_running_if_requested(&self) -> bool {
        self.state
            .compare_exchange(
                MatchState::Requested as u8,
                MatchState::Running as u8,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
    }

    /// Publishes the final result from the audio thread. `Some` goes to `Ready` with the
    /// linear output gain, `None` goes to `Failed`.
    pub(crate) fn publish_result(&self, result: Option<f32>) {
        match result {
            Some(output_gain) => {
                self.output_gain.store(output_gain, Ordering::Release);
                self.state.store(MatchState::Ready as u8, Ordering::Release);
            }
            None => {
                self.state
                    .store(MatchState::Failed as u8, Ordering::Release);
            }
        }
    }

    /// Consumes a finished result on the GUI side, or returns `None` if nothing is ready yet.
    pub(crate) fn take_finished_result(&self) -> Option<MatchResult> {
        let state = self.state.load(Ordering::Acquire);
        match state {
            value if value == MatchState::Ready as u8 => {
                if self
                    .state
                    .compare_exchange(
                        MatchState::Ready as u8,
                        MatchState::Idle as u8,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    )
                    .is_ok()
                {
                    Some(MatchResult::Matched(
                        self.output_gain.load(Ordering::Acquire),
                    ))
                } else {
                    None
                }
            }
            value if value == MatchState::Failed as u8 => self
                .state
                .compare_exchange(
                    MatchState::Failed as u8,
                    MatchState::Idle as u8,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
                .ok()
                .map(|_| MatchResult::Failed),
            _ => None,
        }
    }
}

/// Accumulates input and output energy across a fixed-time measurement window. Lives
/// on the audio thread to avoid allocations and cross-thread mutable state while a
/// match is in progress.
pub(crate) struct MatchMeter {
    input_energy: f64,
    output_energy: f64,
    measured_frames: u64,
    target_frames: u64,
}

impl MatchMeter {
    pub(crate) fn new() -> Self {
        Self {
            input_energy: 0.0,
            output_energy: 0.0,
            measured_frames: 0,
            target_frames: 0,
        }
    }

    /// Starts a new measurement window sized to [`MATCH_DURATION_SECONDS`] at `sample_rate`.
    pub(crate) fn start(&mut self, sample_rate: f32) {
        self.input_energy = 0.0;
        self.output_energy = 0.0;
        self.measured_frames = 0;
        self.target_frames = (sample_rate * MATCH_DURATION_SECONDS).round().max(1.0) as u64;
    }

    pub(crate) fn is_active(&self) -> bool {
        self.target_frames > 0 && self.measured_frames < self.target_frames
    }

    /// Clamps `block_frames` to the number of frames still needed in the current window.
    pub(crate) fn frames_for_block(&self, block_frames: usize) -> usize {
        if !self.is_active() {
            return 0;
        }

        let remaining = self.target_frames - self.measured_frames;
        block_frames.min(remaining as usize)
    }

    /// Adds dry/input energy for up to `frames` samples from `buffer`.
    pub(crate) fn measure_input(&mut self, buffer: &Buffer, frames: usize) {
        self.input_energy += buffer_energy(buffer, frames);
    }

    /// Adds processed/output energy and, once the measurement window is full, returns the
    /// computed output gain (or `Some(None)` if the signal was too quiet to match).
    pub(crate) fn measure_output_and_finish(
        &mut self,
        buffer: &Buffer,
        frames: usize,
    ) -> Option<Option<f32>> {
        if frames == 0 || !self.is_active() {
            return None;
        }

        self.output_energy += buffer_energy(buffer, frames);
        self.measured_frames += frames as u64;

        if self.measured_frames < self.target_frames {
            return None;
        }

        self.target_frames = 0;
        let measured_values = self.measured_frames * buffer.channels() as u64;
        Some(calculate_match_output_gain(
            self.input_energy,
            self.output_energy,
            measured_values,
        ))
    }
}

/// Computes the linear output gain that equalizes measured input and output energy.
/// `value_count` is the total sample count across all channels. Silent or invalid
/// measurements return `None`, otherwise the gain is clamped to the allowed range.
pub(crate) fn calculate_match_output_gain(
    input_energy: f64,
    output_energy: f64,
    value_count: u64,
) -> Option<f32> {
    if value_count == 0 {
        return None;
    }

    let input_mean_square = input_energy / value_count as f64;
    let output_mean_square = output_energy / value_count as f64;
    if input_mean_square <= SILENCE_FLOOR || output_mean_square <= SILENCE_FLOOR {
        return None;
    }

    let min_gain = util::db_to_gain(OUTPUT_GAIN_MIN_DB);
    let max_gain = util::db_to_gain(OUTPUT_GAIN_MAX_DB);
    Some(((input_mean_square / output_mean_square).sqrt() as f32).clamp(min_gain, max_gain))
}

fn buffer_energy(buffer: &Buffer, frames: usize) -> f64 {
    let frames = frames.min(buffer.samples());
    buffer
        .as_slice_immutable()
        .iter()
        .map(|channel| {
            channel[..frames]
                .iter()
                .map(|sample| {
                    let sample = *sample as f64;
                    sample * sample
                })
                .sum::<f64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second_request_is_refused_while_a_run_is_in_flight() {
        let runtime = MatchRuntime::new();

        assert!(runtime.try_request());
        assert!(!runtime.try_request(), "refused while requested");
        assert!(runtime.begin_running_if_requested());
        assert!(!runtime.try_request(), "refused while measuring");

        runtime.publish_result(Some(2.0));
        assert!(matches!(
            runtime.take_finished_result(),
            Some(MatchResult::Matched(gain)) if gain == 2.0
        ));
        assert!(runtime.try_request(), "allowed again once collected");
    }

    #[test]
    fn abandoned_run_can_be_restarted() {
        let runtime = MatchRuntime::new();
        assert!(runtime.try_request());
        assert!(runtime.begin_running_if_requested());

        runtime.abandon();

        assert!(runtime.take_finished_result().is_none());
        assert!(runtime.try_request());
    }

    #[test]
    fn uncollected_result_does_not_lock_out_new_requests() {
        let runtime = MatchRuntime::new();
        assert!(runtime.try_request());
        assert!(runtime.begin_running_if_requested());
        runtime.publish_result(Some(2.0));

        assert!(runtime.try_request());
        assert!(
            runtime.take_finished_result().is_none(),
            "stale result discarded"
        );
    }

    #[test]
    fn unity_match_gain() {
        let gain = calculate_match_output_gain(2.0, 2.0, 2).unwrap();

        assert!((gain - 1.0).abs() < 1.0e-6);
    }

    #[test]
    fn six_db_match() {
        let gain = calculate_match_output_gain(4.0, 1.0, 4).unwrap();

        assert!((gain - 2.0).abs() < 1.0e-6);
    }

    #[test]
    fn clamps_high() {
        let gain = calculate_match_output_gain(1.0, 1.0e-5, 1).unwrap();

        assert!((gain - util::db_to_gain(OUTPUT_GAIN_MAX_DB)).abs() < 1.0e-3);
    }

    #[test]
    fn clamps_low() {
        let gain = calculate_match_output_gain(1.0, 1.0e5, 1).unwrap();

        assert!((gain - util::db_to_gain(OUTPUT_GAIN_MIN_DB)).abs() < 1.0e-6);
    }

    #[test]
    fn silent_input() {
        assert_eq!(calculate_match_output_gain(0.0, 1.0, 1), None);
    }

    #[test]
    fn silent_output() {
        assert_eq!(calculate_match_output_gain(1.0, 0.0, 1), None);
    }
}
