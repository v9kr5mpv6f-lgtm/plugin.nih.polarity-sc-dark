# Slackwater

Slackwater is an FFT-based spectral compressor by Oskillator, built on top of a
vendored NIH-plug snapshot. It is a fork of Robert Agthe's
[Polarity-SC-Dark](https://github.com/polarity/plugin.nih.polarity-sc-dark), which
is itself a port of Robbert van der Helm's
[Spectral Compressor](https://github.com/robbert-vdh/nih-plug). Like both, it is
licensed under the GPL-3.0-or-later. The repository is organized as a plugin project
first: the product crate lives in `plugin/`, and the framework crates it
depends on live in `framework/`.

The current plugin feature set is intentionally focused on three compressor
modes:

- Pink Noise
- Sidechain Matching
- Sidechain Compression

Freeze and IR export remain part of the compressor workflow.

![Screenshot](Polarity-SC-Dark.png)

## Exporting the frozen curve as an impulse response

With Freeze engaged, **Export IR...** renders the frozen per-bin gain curve as a 32-bit float WAV
(one channel per input channel). The impulse response is centred: its main peak sits in the middle
of the file, with the pre-ringing before it and the tail after it. The suggested file name says how
many samples in the peak is (for example `slackwater-freeze-ir-peak-2048smp.wav`), and
convolving with the file delays your signal by that many samples, so compensate for it in your
convolver. If the export fails, the reason is shown in the status label at the top of the editor.

## Repository Layout

| Path | Purpose |
| --- | --- |
| `plugin/` | Slackwater crate: DSP, GUI, entry points, assets |
| `framework/nih_plug/` | Vendored NIH-plug core framework crate |
| `framework/nih_plug_derive/` | Vendored NIH-plug derive macros |
| `framework/nih_plug_vizia/` | Vendored VIZIA integration used by the plugin |
| `framework/nih_plug_xtask/` | Vendored bundling/tooling library |
| `framework/xtask/` | `cargo xtask` entry point |
| `framework/cargo_nih_plug/` | Optional `cargo nih-plug` wrapper |
| `docs/` | Project-specific technical notes |
| `docs/user-feedback` | Feature Requests and Reports by users. Each user a markdown file |


## Build

After installing [Rust](https://rustup.rs/), bundle the plugin with:

```shell
rustup run stable cargo xtask bundle slackwater --release
```

Artifacts are written to `target/bundled/`.

To build just the plugin crate:

```shell
rustup run stable cargo build -p slackwater
```

To run the standalone binary:

```shell
rustup run stable cargo run -p slackwater --bin slackwater_standalone --release
```

## GUI Development

The plugin theme lives at `plugin/src/editor/theme.css`. Use the `hot-reload`
feature to load the CSS from disk at runtime:

```shell
cargo run -p slackwater --bin slackwater_standalone --features hot-reload
```

Close and reopen the editor window after editing `plugin/src/editor/theme.css`.

## Notes

- All vendored NIH-plug crates live under `framework/` because I like it.
- `bundler.toml` is intentionally scoped to this single plugin.
