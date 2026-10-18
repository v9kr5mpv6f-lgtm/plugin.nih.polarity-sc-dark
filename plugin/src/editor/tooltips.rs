//! The words shown when you hover over a control. Written for people who have never used a
//! spectral compressor, and still accurate enough for people who have: each one says what the
//! control does, then what turning it up or down means. The manual (`docs/MANUAL.md`) goes deeper.
//!
//! [`parameter_tooltip()`] is keyed by the parameter's display name, and a test checks that every
//! parameter a host can see has an entry, so a new parameter cannot ship without one.

// Controls that are not parameters.

pub const DELTA: &str = "Listen to what Slackwater is changing. This plays the difference between \
    your original sound and the processed sound, so you can hear exactly what is being removed or \
    added. Click again to go back to normal.";

pub const MATCH_LEVEL: &str = "Matches the loudness of the processed sound to the original. \
    Play your audio, click, and after about 5 seconds Slackwater sets Output Gain so the two sound \
    equally loud. It needs audio to be playing.";

pub const HELP: &str = "Opens the Slackwater manual in your web browser.";

pub const MATCH_CURVE: &str = "Shapes the threshold curve to fit your audio. Play something \
    typical for about 5 seconds and Slackwater sets the Global Threshold, the curve bend and up to \
    8 curve points to follow its tonal balance. Threshold Center and Slope stay as you set them. \
    Needs audio to be playing.";

pub const CLEAR_CURVE: &str =
    "Resets the threshold curve to a plain, flat starting point: removes \
    every curve point and puts Global Threshold, Threshold Center, Slope and the curve bend back \
    to their defaults.";

pub const CURVE_PRESETS: &str = "Ready-made threshold curve shapes for common jobs such as \
    mastering, bass and drums, followed by any presets you have saved. Picking one replaces the \
    current curve.";

pub const SAVE_PRESET: &str = "Saves the current threshold curve under a name you choose, so you \
    can reuse it in any project. Saved presets appear in the Curve Presets list.";

pub const DELETE_PRESET: &str = "Deletes the selected curve preset from your computer. Only \
    presets you saved yourself can be deleted, and it cannot be undone.";

pub const EXPORT_IR: &str = "Saves the frozen frequency shape as a WAV impulse response that you \
    can load into a convolution reverb or EQ. Turn Freeze on first. The file name says how many \
    samples in the main peak sits.";

/// The tooltip for the parameter with this display name, if there is one.
pub fn parameter_tooltip(name: &str) -> Option<&'static str> {
    Some(match name {
        // Globals
        "Output Gain" => {
            "How loud the processed sound is, from -50 to +50 dB. It only changes the processed \
            sound, before the Mix control blends in the original. Use Match to set it \
            automatically."
        }
        "Bypass" => {
            "Turns the processing off but keeps the built-in delay, so this track stays in time \
            with the rest of your project. Output Gain is not applied while bypassed."
        }
        "Mix" => {
            "Blends between your original sound (0%) and the processed sound (100%). The \
            original is delayed to line up with the processed sound, so partial blends stay in \
            phase."
        }
        "Window Size" => {
            "How much audio Slackwater looks at in one go, from 64 to 32768 samples. A bigger \
            window sees finer detail between frequencies but reacts more slowly and adds more \
            delay. A smaller window reacts faster but is less precise."
        }
        "Window Overlap" => {
            "How many analysis windows overlap each other, from 4x to 32x. More overlap sounds \
            smoother and cleaner but uses more CPU. Fewer is lighter but can sound rougher."
        }
        "Attack" => {
            "How quickly each frequency reacts when it gets louder, from 0 to 10000 ms. Short \
            attack catches peaks quickly. Long attack lets the start of sounds through and \
            reacts more gently."
        }
        "Release" => {
            "How slowly each frequency lets go after it gets quieter, from 0 to 10000 ms. Short \
            release recovers quickly and can sound busy. Long release is smoother and steadier."
        }
        "Freeze" => {
            "Locks the current frequency-by-frequency gain shape and keeps applying it, even \
            when the music changes. Turn it off to go back to live processing. Freeze also \
            makes Export IR available."
        }

        // Threshold
        "Global Threshold" => {
            "Moves the whole threshold curve up or down, from -100 to +20 dB. In Pink Noise \
            mode it is a level. In the sidechain modes it sets the threshold relative to the \
            sidechain signal."
        }
        "Threshold Center" => {
            "The frequency the Global Threshold is attached to, from 20 Hz to 20 kHz. The \
            Threshold Slope pivots around it. The faint moon on the analyzer hangs at this \
            frequency."
        }
        "Threshold Slope" => {
            "Tilts the threshold curve across the spectrum. 3.00 is the neutral setting: it \
            follows pink noise in Pink Noise mode and leaves the sidechain's own shape alone in \
            the sidechain modes. Lower numbers tilt the curve up towards the highs."
        }
        "Threshold Curve" => {
            "Bends the threshold curve into a bowl or an arch around the Threshold Center. It \
            is mostly set by Curve Matching and the curve points, and it is here so you can \
            automate it from your host."
        }
        "Mode" => {
            "What the thresholds follow. Pink Noise pushes every frequency towards a pink \
            noise balance. Sidechain Matching makes your sound take on the tonal balance of the \
            sidechain input. Sidechain Compression lets the sidechain's frequencies decide how \
            much your sound is compressed."
        }
        "SC Channel Link" => {
            "Sidechain modes only. How much the left and right channels of the sidechain share \
            their level. 0% treats each channel separately, 100% uses the average of both so \
            the stereo image stays steady."
        }
        "Smoothing" => {
            "Blurs the gain changes across neighbouring frequencies so the result sounds less \
            spiky and more natural. 0% leaves every frequency independent. 100% smooths over up \
            to about an octave on each side."
        }

        // Upwards compressor
        "Upwards Offset" => {
            "Moves the upwards threshold away from the main curve, from -50 to +50 dB. \
            Anything quieter than the threshold is lifted. Raise it to lift more of the quiet \
            detail, lower it to lift less."
        }
        "Upwards Ratio" => {
            "How strongly quiet sound is lifted towards the threshold. 1:1 does nothing. Higher \
            ratios lift more, and very high ratios pull everything quiet right up to the \
            threshold."
        }
        "Upwards Hi-Freq Rolloff" => {
            "Makes the ratio gentler as frequency rises. 0% uses the same ratio everywhere. \
            Higher values soften the highs more, until at 100% the ratio reaches 1:1 at the top \
            of the spectrum. This keeps the highs from getting harsh or hissy."
        }
        "Upwards Knee" => {
            "How gradually the lifting starts around the threshold, from 0 to 36 dB. 0 is a hard \
            corner. Larger values ease in smoothly and sound more natural."
        }

        // Downwards compressor
        "Downwards Offset" => {
            "Moves the downwards threshold away from the main curve, from -50 to +50 dB. \
            Anything louder than the threshold is turned down. Lower it to turn down more, raise \
            it to turn down less."
        }
        "Downwards Ratio" => {
            "How strongly loud sound is turned down towards the threshold. 1:1 does nothing. \
            Higher ratios squash more, and very high ratios act like a limiter that holds each \
            frequency at the threshold."
        }
        "Downwards Hi-Freq Rolloff" => {
            "Makes the ratio gentler as frequency rises. 0% uses the same ratio everywhere. \
            Higher values soften the highs more, until at 100% the ratio reaches 1:1 at the top \
            of the spectrum. This keeps the highs open and airy."
        }
        "Downwards Knee" => {
            "How gradually the turning-down starts around the threshold, from 0 to 36 dB. 0 is a \
            hard corner. Larger values ease in smoothly and sound more natural."
        }

        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SpectralCompressor;
    use nih_plug::prelude::*;

    fn with_large_stack(test: impl FnOnce() + Send + 'static) {
        std::thread::Builder::new()
            .stack_size(8 * 1024 * 1024)
            .spawn(test)
            .expect("failed to spawn test thread")
            .join()
            .expect("test thread panicked");
    }

    /// Display names of every parameter a host can see (so not the hidden curve points).
    fn visible_parameter_names() -> Vec<String> {
        let plugin = SpectralCompressor::default();
        plugin
            .params()
            .param_map()
            .into_iter()
            .filter(|(_, ptr, _)| !unsafe { ptr.flags() }.contains(ParamFlags::HIDDEN))
            .map(|(_, ptr, _)| unsafe { ptr.name() }.to_owned())
            .collect()
    }

    #[test]
    fn every_visible_parameter_has_a_tooltip() {
        with_large_stack(|| {
            let names = visible_parameter_names();
            assert!(
                names.len() > 10,
                "expected the plugin's parameters, got {names:?}"
            );

            let missing: Vec<_> = names
                .iter()
                .filter(|name| parameter_tooltip(name).is_none())
                .collect();
            assert!(
                missing.is_empty(),
                "parameters without a tooltip: {missing:?}"
            );
        });
    }

    #[test]
    fn no_tooltip_is_orphaned() {
        // A tooltip for a parameter that no longer exists is a sign the list went stale.
        with_large_stack(|| {
            let names = visible_parameter_names();
            let candidates = [
                "Output Gain",
                "Bypass",
                "Mix",
                "Window Size",
                "Window Overlap",
                "Attack",
                "Release",
                "Freeze",
                "Global Threshold",
                "Threshold Center",
                "Threshold Slope",
                "Threshold Curve",
                "Mode",
                "SC Channel Link",
                "Smoothing",
                "Upwards Offset",
                "Upwards Ratio",
                "Upwards Hi-Freq Rolloff",
                "Upwards Knee",
                "Downwards Offset",
                "Downwards Ratio",
                "Downwards Hi-Freq Rolloff",
                "Downwards Knee",
            ];
            for name in candidates {
                assert!(
                    names.iter().any(|n| n == name),
                    "'{name}' is not a parameter any more"
                );
            }
        });
    }

    #[test]
    fn tooltips_are_clean_sentences_of_a_readable_length() {
        let all = [
            DELTA,
            MATCH_LEVEL,
            HELP,
            MATCH_CURVE,
            CLEAR_CURVE,
            CURVE_PRESETS,
            SAVE_PRESET,
            DELETE_PRESET,
            EXPORT_IR,
        ];
        let params = [
            "Output Gain",
            "Bypass",
            "Mix",
            "Window Size",
            "Window Overlap",
            "Attack",
            "Release",
            "Freeze",
            "Global Threshold",
            "Threshold Center",
            "Threshold Slope",
            "Threshold Curve",
            "Mode",
            "SC Channel Link",
            "Smoothing",
            "Upwards Offset",
            "Upwards Ratio",
            "Upwards Hi-Freq Rolloff",
            "Upwards Knee",
            "Downwards Offset",
            "Downwards Ratio",
            "Downwards Hi-Freq Rolloff",
            "Downwards Knee",
        ]
        .map(|name| parameter_tooltip(name).expect(name));

        for text in all.iter().chain(params.iter()) {
            assert!(text.ends_with('.'), "should end with a full stop: {text}");
            assert!(!text.contains("  "), "double space in: {text}");
            assert!(!text.contains('\n'), "tooltips wrap on their own: {text}");
            assert!(
                (30..=360).contains(&text.len()),
                "length {} is outside what a tooltip can hold: {text}",
                text.len()
            );
        }
    }
}
