// Spectral Compressor: an FFT based compressor
// Copyright (C) 2021-2024 Robbert van der Helm
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! NanoVG colors used by the analyzer's custom canvas drawing. Everything
//! reachable through vizia styling (backgrounds, text, borders, ...) lives in
//! `theme.css` instead so it can be tweaked with the `hot-reload` feature.

use nih_plug_vizia::vizia::vg;

// Moonpull: a night sea. Analyzer background and spectrum.
pub const ANALYZER_BACKGROUND: vg::Color = vg::Color::rgbaf(0.031, 0.035, 0.078, 1.0);
pub const ANALYZER_BORDER: vg::Color = vg::Color::rgbaf(0.114, 0.129, 0.251, 1.0);
pub const ANALYZER_SPECTRUM_TOP: vg::Color = vg::Color::rgbaf(0.875, 0.902, 1.0, 0.85);
pub const ANALYZER_SPECTRUM_BOTTOM: vg::Color = vg::Color::rgbaf(0.875, 0.902, 1.0, 0.05);
pub const ANALYZER_FREQUENCY_GUIDE_LINE: vg::Color = vg::Color::rgbaf(0.62, 0.65, 0.78, 0.10);
pub const ANALYZER_FREQUENCY_GUIDE_LABEL: vg::Color = vg::Color::rgbaf(0.43, 0.46, 0.63, 1.0);

// The moon sits behind the analyzer at the threshold's centre frequency.
pub const ANALYZER_MOON_GLOW_INNER: vg::Color = vg::Color::rgbaf(0.79, 0.85, 1.0, 0.20);
pub const ANALYZER_MOON_GLOW_MID: vg::Color = vg::Color::rgbaf(0.50, 0.58, 0.84, 0.07);
pub const ANALYZER_MOON_GLOW_OUTER: vg::Color = vg::Color::rgbaf(0.031, 0.035, 0.078, 0.0);
pub const ANALYZER_MOON_RING: vg::Color = vg::Color::rgbaf(0.79, 0.85, 1.0, 0.14);

// Gain reduction overlays: lantern amber for downwards (ebb), moonlight blue for upwards (flood).
pub const ANALYZER_GR_DOWNWARDS: vg::Color = vg::Color::rgbaf(0.949, 0.702, 0.369, 0.30);
pub const ANALYZER_GR_UPWARDS: vg::Color = vg::Color::rgbaf(0.624, 0.769, 1.0, 0.30);

// Threshold curves: a bright core over a soft glow in the overlay colors.
pub const ANALYZER_THRESHOLD_DOWNWARDS: vg::Color = vg::Color::rgbaf(1.0, 0.835, 0.604, 1.0);
pub const ANALYZER_THRESHOLD_UPWARDS: vg::Color = vg::Color::rgbaf(0.812, 0.878, 1.0, 1.0);
pub const ANALYZER_THRESHOLD_DOWNWARDS_GLOW: vg::Color =
    vg::Color::rgbaf(0.949, 0.702, 0.369, 0.22);
pub const ANALYZER_THRESHOLD_UPWARDS_GLOW: vg::Color = vg::Color::rgbaf(0.624, 0.769, 1.0, 0.22);
pub const ANALYZER_THRESHOLD_POINT_FILL: vg::Color = vg::Color::rgbaf(0.906, 0.918, 1.0, 1.0);
pub const ANALYZER_THRESHOLD_POINT_HALO: vg::Color = vg::Color::rgbaf(0.906, 0.918, 1.0, 0.22);
