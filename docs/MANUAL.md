# Slackwater Manual

Version 0.6.0 · by Oskillator

Slackwater is a *spectral compressor*. It listens to your sound one frequency at a time and turns each one up or down toward a target you draw, so the whole tonal balance settles into place like water finding its level.

This manual is written so you can start using Slackwater in a couple of minutes, and then keep reading as far as you like. If you are new to compressors, read **Quick start** and **How Slackwater works** first. If you already know your way around, jump straight to **Controls** or **Technical notes**. You can open this manual at any time with the **Help** button on the plug-in.

## What Slackwater does

An ordinary compressor looks at the *overall* loudness of a sound and turns the whole thing down when it gets too loud. A bass note can make it squash the vocals along with it, because the compressor cannot tell them apart.

Slackwater works differently. It splits your sound into thousands of tiny frequency slices and runs a separate little compressor on every slice. It can turn down the one harsh frequency that sticks out without touching anything else, and it can lift the quiet frequencies that are missing, all at the same moment.

You tell it what the "right" balance looks like by drawing a **threshold curve** across the frequency range. Slackwater then pushes every slice toward that curve:

- Slices that are **louder** than the curve are turned **down** (the *Downwards* compressor, shown in amber).
- Slices that are **quieter** than the curve are turned **up** (the *Upwards* compressor, shown in blue).

The curve can follow pink noise (a naturally balanced spectrum), match another sound you send into the sidechain input, or be any shape you draw by hand.

**What people use it for:**

- Evening out the tonal balance of a mix bus, a vocal, a drum bus or a sample.
- Taming resonances and harshness without a dozen EQ cuts.
- Making one sound take on the tonal character of another (for example, a pad that sits exactly under a lead).
- Lifting dull, thin or very dynamic material toward a steadier balance.
- Creative effects: freezing a spectrum, turning sounds toward pink noise, and exporting the result as an impulse response.

**What you need:** a host (DAW) that loads VST3, CLAP or Audio Unit plug-ins. Slackwater works on mono and stereo tracks. It also has a sidechain input for the two sidechain modes.

## Quick start

The fastest way to hear what Slackwater does:

1. **Put Slackwater on a track.** A full mix, a drum bus or a pad is ideal. Press play so audio is running through it.
2. **Watch the analyzer.** The big panel on the right shows your sound as a grey landscape. The two coloured lines are the thresholds. Blue is for lifting quiet sound, amber is for turning down loud sound.
3. **Set a Downwards Ratio.** Slackwater starts with every Ratio at **1:1**, which does nothing at all, so it is safe to insert. Raise **Downwards Ratio** (bottom right) to about **4:1**. Amber bars appear in the analyzer wherever a frequency is being turned down.
4. **Set an Upwards Ratio if you want to lift quiet detail.** Raise **Upwards Ratio** to about **3:1**. Blue bars show where sound is being lifted.
5. **Move the Global Threshold.** Drag **Global Threshold** (top middle) up or down and watch the two lines move. Lower thresholds mean more of your sound is above the amber line, so more is turned down.
6. **Compare.** Click **Match** in the top left so the processed sound is as loud as the original, then switch **Bypass** on and off. Now you are comparing tone, not loudness.
7. **Listen to what it is doing.** Click **Delta** to hear only the difference between the original and the processed sound. Click it again to go back.

> **Tip:** If it sounds too strong, do not turn the ratios all the way down. Raise **Mix** a little less than 100%, or increase the **Knee** and **Smoothing** for a gentler, more natural result.

## Installing

Slackwater comes in three formats. Install the one your host uses; you can install all of them.

| Format | macOS | Windows | Linux |
| --- | --- | --- | --- |
| **VST3** | `~/Library/Audio/Plug-Ins/VST3` | `C:\Program Files\Common Files\VST3` | `~/.vst3` |
| **CLAP** | `~/Library/Audio/Plug-Ins/CLAP` | `C:\Program Files\Common Files\CLAP` | `~/.clap` |
| **Audio Unit** | `~/Library/Audio/Plug-Ins/Components` | not available | not available |

Copy the Slackwater file for your format into the folder, then rescan plug-ins in your host (most hosts do this on start-up). On macOS you can also use the system-wide folders under `/Library/Audio/Plug-Ins/`.

> **Note:** On macOS, plug-ins downloaded from the internet may be blocked the first time because they are not notarized by Apple. If your host says it cannot load Slackwater, open Terminal and run the following once for each file you installed, adjusting the path, then rescan:
>
> `xattr -dr com.apple.quarantine ~/Library/Audio/Plug-Ins/VST3/Slackwater.vst3`

**Your saved curve presets** live in a folder of their own, so they survive updates:

- macOS: `~/Library/Application Support/com.Oskillator.Slackwater/curve-presets`
- Windows: `%APPDATA%\Oskillator\Slackwater\config\curve-presets`
- Linux: `~/.config/slackwater/curve-presets`

To uninstall, delete the Slackwater files from the plug-in folders. You can delete the presets folder too if you do not want to keep them.

## How Slackwater works

You do not need this chapter to use the plug-in, but ten minutes with it will make every control make sense.

### Sound is made of frequencies

Any sound is a mixture of many frequencies at once: low rumble, body, warmth, presence, air. A **spectrum** is a picture of how much of each frequency is in the sound. Bass sits on the left, treble on the right, and loudness goes up and down. The analyzer in Slackwater draws this picture, and it moves as the music plays.

### The threshold curve

A normal compressor has one threshold: a single loudness level. Slackwater has a *threshold curve*: a different threshold at every frequency. You can think of it as a line drawn across the spectrum picture that says "this is how loud each frequency should be."

Where the sound is **above** the curve, the **Downwards** compressor turns it down. Where the sound is **below** the curve, the **Upwards** compressor turns it up. The **Ratio** controls how firmly each one pulls the sound toward the curve:

- **1:1** does nothing.
- **2:1** to **4:1** is gentle to moderate: half or a quarter of the distance above the line is let through.
- **10:1** and above holds each frequency close to the line.
- At the extreme (up to 500:1) a frequency is pinned almost exactly to the curve.

Because the two compressors have their own **Offset**, the amber and blue lines can sit at different heights. A gap between them is a "free zone" where the sound is left alone. That is often the most musical way to use it.

### What decides the curve?

The **Mode** control chooses what the thresholds follow:

- **Pink Noise** is the starting point. The curve is a gently sloping line that matches the balance of pink noise, which sounds natural and even to the human ear. You can then bend and tilt it.
- **Sidechain Matching** makes the curve follow the spectrum of a second sound that you send to the plug-in's sidechain input. Your sound is pulled toward the same tonal balance. This is how you make one sound "fit under" another.
- **Sidechain Compression** lets the sidechain sound *control* the compression. Where the sidechain has energy, your sound is turned down at those frequencies, which is a precise "ducking" effect.

### Time and detail

Slackwater analyses small slices of audio called **windows**. A bigger window shows more detail between neighbouring frequencies, but reacts more slowly and adds a longer delay (latency). A smaller window reacts faster but sees less detail. The **Window Size** and **Window Overlap** controls set this trade-off. The **Attack** and **Release** controls then set how fast each frequency reacts when it gets louder and how slowly it recovers when it gets quieter.

### Reading the analyzer

The analyzer on the right is a window into everything Slackwater is doing.

| What you see | What it means |
| --- | --- |
| **Grey bars and fill** | The spectrum of your sound as Slackwater is hearing it. |
| **Blue line** | The Upwards threshold. Sound below it is lifted. |
| **Amber line** | The Downwards threshold. Sound above it is turned down. |
| **Amber bars hanging down from the middle** | Where, and by how much, frequencies are being turned down. |
| **Blue bars rising from the middle** | Where, and by how much, frequencies are being lifted. |
| **White dots** | Curve points you can drag to reshape the thresholds. |
| **A faint moon** | The Threshold Center: the frequency the Global Threshold is attached to. |
| **Vertical lines labelled 30 Hz to 10 kHz** | Frequency guides. |

The bars show gain change on a scale where the middle of the panel is no change and the top and bottom edges are 40 dB of lift or reduction.

> **Note:** The analyzer is drawn with a gentle upward tilt toward the treble, so that pink noise (which naturally falls toward the treble) looks flat. Do not be surprised that the grey landscape of a real mix slopes upward or downward: it is the balance of your music, not a fault.

## The screen at a glance

The plug-in has five areas:

- **Header (top left):** the **Delta**, **Match**, **Bypass** and **Help** buttons, a status message, and the plug-in name and version.
- **Globals (left):** output level, mix, the window settings, attack and release, and Freeze.
- **Threshold (middle):** the global threshold, its centre and slope, curve matching, curve presets, the mode, sidechain link and smoothing.
- **Upwards and Downwards (bottom):** the two compressors, with four controls each.
- **Analyzer (right):** the live display, where you can also draw curve points.

Hover over any control for a short explanation.

**Working the sliders:**

- **Click or drag** to set a value. The mouse wheel or a trackpad scroll nudges it in steps, and **Shift** makes the steps finer.
- Hold **Shift** while dragging for fine control.
- **Alt-click** (Option-click on a Mac) to type an exact value, then press Enter.
- **Double-click**, **right-click** or **Ctrl/Cmd-click** to reset a slider to its default.

## Controls

Every control is listed here, with its range, its default and when you might change it.

### Header buttons

| Control | What it does |
| --- | --- |
| **Delta** | Plays the *difference* between your original sound and the processed sound, so you can hear exactly what Slackwater is removing and adding. Click again to return to normal. Output Gain and Mix are not applied while Delta is on. |
| **Match** | Matches loudness. Play audio and click: after about five seconds Slackwater measures how much louder or quieter the processed sound is and sets Output Gain so they are equally loud. If the signal is too quiet to measure, the status line says so. |
| **Bypass** | Switches the processing off. The built-in delay stays in place so the track remains in time with the rest of your project. Output Gain is not applied while bypassed. |
| **Help** | Opens this manual in your web browser. |

> **Tip:** Always compare the processed and original sound at the same loudness. A louder version almost always sounds "better", which can fool you. **Match** exists to prevent that.

### Globals

| Control | Range | Default | What it does |
| --- | --- | --- | --- |
| **Output Gain** | −50 to +50 dB | 0 dB | How loud the processed sound is. It is applied to the processed sound only, *before* Mix blends in the original. |
| **Mix** | 0% to 100% | 100% | Blends between your original (0%) and the processed sound (100%). The original is delayed to line up exactly, so partial mixes do not smear or phase. |
| **FFT Window** (Window Size) | 64 to 32768 samples | 2048 | How much audio Slackwater analyses at once. See the table below. |
| **FFT Window** (Window Overlap) | 4x to 32x | 16x | How many windows overlap. More overlap is smoother and cleaner but uses more CPU. |
| **Attack** | 0 to 10000 ms | 150 ms | How quickly each frequency reacts when it gets *louder*. |
| **Release** | 0 to 10000 ms | 300 ms | How slowly each frequency lets go when it gets *quieter*. |
| **Freeze** | on / off | off | Locks the current gain shape and keeps applying it. |
| **Export IR…** | button | — | Saves the frozen gain shape as a WAV impulse response. Available when Freeze is on. |

The **FFT Window** row holds two sliders: the left one is the window size, the right one is the overlap.

**Choosing a window size**

| Window size | Latency (delay) at 48 kHz | Best for |
| --- | --- | --- |
| 64 – 256 | about 1 – 5 ms | Very fast, tight effects and live monitoring. Coarse frequency detail, so it works more like a multi-band compressor. |
| 512 – 1024 | about 11 – 21 ms | Drums and percussive material that needs quick reactions. |
| **2048** (default) | about 43 ms | A balanced starting point for most material. |
| 4096 – 8192 | about 85 – 171 ms | Mixes, pads, vocals and sustained sounds that want fine detail. |
| 16384 – 32768 | about 340 – 680 ms | Mastering-style tonal shaping and very precise work. Slow to react, and it adds a long delay. |

Slackwater tells your host about its delay (the *latency*), and most hosts compensate for it automatically. The latency equals the window size in samples.

**Attack and Release in practice**

Attack and Release act on every frequency separately. Short settings follow the music closely and can sound busy or "grabby". Long settings give a smooth, steady result that leaves the natural movement of the music alone.

- For gentle tonal shaping, use a long attack and release (hundreds of milliseconds).
- To catch sudden bright or boomy moments, shorten the attack.
- There is no point setting a time shorter than one *hop* (the window size divided by the overlap). At the defaults that is about 2.7 ms at 48 kHz.

> **Note:** Changing Window Size restarts the analysis, so you may hear a brief fade-in on the processed sound. Choose the size before you fine-tune the rest.

### Threshold

These controls shape the curve that every frequency is pulled toward.

| Control | Range | Default | What it does |
| --- | --- | --- | --- |
| **Global Threshold** | −100 to +20 dB | −12 dB | Moves the whole curve up or down. |
| **Threshold** (centre frequency) | 20 Hz to 20 kHz | 1 kHz | The frequency the Global Threshold is attached to. The slope pivots around it. |
| **Threshold** (slope) | about −33 to +39 | 3.00 | Tilts the curve. 3.00 is neutral. |
| **Curve Matching: Match** | button | — | Listens for five seconds and shapes the curve to fit your audio. |
| **Curve Matching: Clear** | button | — | Resets the curve to a flat starting point. |
| **Curve Presets** | list | — | Ready-made curve shapes, plus your own. |
| **Save** and **Del** | buttons | — | Save the current curve as a preset, or delete one of yours. |
| **Mode** | three choices | Pink Noise | What the thresholds follow. |
| **SC Channel Link** | 0% to 100% | 80% | Sidechain modes only. How much the two channels share their level. |
| **Smoothing** | 0% to 100% | 0% | Blurs the gain changes across neighbouring frequencies. |

The **Threshold** row holds two sliders: the left one is the centre frequency, the right one is the slope.

**Slope.** The slope control tilts the curve. The neutral value, 3.00, makes the curve follow pink noise in Pink Noise mode, and leaves the sidechain's own shape untouched in the sidechain modes. A *lower* number tilts the curve up toward the treble (the high frequencies are allowed to be louder). A *higher* number tilts it down (the high frequencies are held quieter).

**SC Channel Link.** In the sidechain modes the left and right channels of the sidechain each have their own spectrum. At 0% each channel only listens to its own side. At 100% both channels listen to their combined level, which keeps the stereo image steady. The default of 80% is a good balance.

**Smoothing.** Real spectra are spiky, and a compressor that reacts to every spike can sound edgy or "watery". Smoothing blends each frequency's gain change with its neighbours. At 0% each frequency is treated on its own. As you raise it, neighbouring frequencies are averaged over a wider and wider range, up to about an octave on each side at 100%. Smoothing makes broad, natural moves and is excellent for mastering-style work. Keep it low if you want surgical precision.

### Upwards and Downwards

Slackwater has two compressors with identical controls. **Upwards** lifts sound that is *below* its threshold; **Downwards** turns down sound that is *above* its threshold.

| Control | Range | Upwards default | Downwards default | What it does |
| --- | --- | --- | --- | --- |
| **Offset** | −50 to +50 dB | 0 dB | 0 dB | Moves this compressor's threshold relative to the main curve. |
| **Ratio** | 1:1 to 500:1 | 1:1 | 1:1 | How firmly sound is pulled toward the threshold. 1:1 does nothing. |
| **Hi-Freq Rolloff** | 0% to 100% | 75% | 0% | Makes the ratio gentler toward the treble. |
| **Knee** | 0 to 36 dB | 6 dB | 6 dB | How gradually the effect starts around the threshold. |

**Offset.** Both compressors start on the main curve. Raising the Upwards Offset lifts its threshold, so more of the quiet detail is brought up. Lowering the Downwards Offset lowers its threshold, so more of the loud sound is turned down. Keep the Upwards line below the Downwards line to leave a "free zone" in between where sound is untouched.

**Ratio.** Think of the ratio as a rubber band: 1:1 is no band at all, and higher ratios are a stiffer and stiffer band pulling each frequency to the line.

**Hi-Freq Rolloff.** High frequencies are often delicate, and a strong ratio on them can sound harsh, hissy or lispy. Hi-Freq Rolloff lets you ease the ratio off as frequency rises. The reduction grows with the *logarithm* of frequency, so it is already noticeable in the midrange. Here is what a **4:1** setting becomes at different Hi-Freq Rolloff amounts:

| Rolloff | 100 Hz | 1 kHz | 10 kHz | 20 kHz |
| --- | --- | --- | --- | --- |
| 0% | 4.0 : 1 | 4.0 : 1 | 4.0 : 1 | 4.0 : 1 |
| 50% | 2.4 : 1 | 2.0 : 1 | 1.7 : 1 | 1.6 : 1 |
| 100% | 1.7 : 1 | 1.3 : 1 | 1.1 : 1 | 1.0 : 1 |

The Upwards compressor defaults to 75%, a gentle rolloff, so quiet treble is lifted more softly. The Downwards compressor defaults to 0%, so loud sound is turned down by the same ratio at every frequency.

**Knee.** A *hard knee* (0 dB) switches the compressor on abruptly the moment sound crosses the threshold. A *soft knee* eases in over a range of decibels centred on the threshold, which sounds smoother and more natural. The default of 6 dB is a good all-round value.

> **Note:** Frequencies below 20 Hz and sounds that are completely silent are never lifted by the Upwards compressor, so it cannot boost rumble or noise floor that carries no music.

## The curve and its points

The threshold curve has three layers that add together:

1. **The base curve:** the **Global Threshold** (its height), the **Threshold Center**, and the **Slope**.
2. **A bend:** a gentle bowl or arch shape around the centre, kept as a parameter called **Threshold Curve** (it has no on-screen slider, because the points are friendlier, but you can automate it from your host).
3. **Curve points:** up to eight draggable dots that add bumps and dips anywhere along the curve.

### Drawing curve points

All of this happens in the analyzer:

- **Click empty space** to add a point. A new point appears where you click.
- **Drag a point** to move it. Dragging up raises the threshold at that frequency, dragging down lowers it.
- **Right-click a point** to remove it.
- You can have up to **eight** points. The curve passes smoothly through them, and outside the first and last point it simply carries on at the same height.

A point that raises the curve makes that frequency *less* likely to be turned down and *more* likely to be lifted. A point that lowers the curve does the opposite. That is the key to using them: a dip in the threshold at a harsh frequency means it gets turned down more.

### Curve Matching

Instead of drawing the curve yourself, you can let Slackwater draw it from your audio.

1. Play a typical stretch of the sound you want to match.
2. Click **Match** under *Curve Matching*.
3. After about five seconds Slackwater sets the **Global Threshold**, the bend and, where the fit needs them, up to eight curve points, so the curve follows the average tonal balance of what it heard.

The **Threshold Center** and **Slope** stay exactly as you set them. The curve is placed so that the average level of your sound sits right at the Downwards threshold, which gives you a sensible place to start adding ratio. Click **Clear** to throw the curve away and start again from flat.

> **Tip:** Curve Matching is a wonderful way to capture the tonal balance of a reference track: match against the reference, then switch to your own mix and let the compressors pull it toward the same shape.

### Curve Presets

The **Curve Presets** list holds ready-made curves:

| Preset | What its shape does |
| --- | --- |
| **Equal Loudness** | Lets the deep bass and the top end sit louder, and holds the upper mids (around 1 to 8 kHz, where the ear is most sensitive) a little lower. |
| **Master Balanced** | Very gentle: slightly more room for the lowest bass, slightly less for the upper mids. Meant for a finished mix. |
| **Master Warm** | Lets the lows and the very top sit louder, and holds the low-mids (around 350 Hz) down a little, for a clearer, more open warmth. |
| **Bass Bus** | Holds the area from about 60 to 220 Hz down firmly, and lets the sub-bass and the top end through. |
| **Drum Bus** | Holds the low end around 80 to 180 Hz and the 2 to 3 kHz "honk" down, and lets the sub and the air through. |
| **Pads** | Holds the muddy area around 300 Hz down, and lets the deep lows and the shimmer on top through. |
| **Lead** | Holds the presence area from about 700 Hz to 4 kHz down a little, and lets the lows and the air through. |

In every preset, a *lower* threshold at a frequency means that frequency is turned down more (and lifted less), and a *higher* one means it is left louder. What that sounds like depends on your ratios, so treat the presets as shapes, then add ratio.

The built-in presets set the Global Threshold, the bend and the curve points. They leave your **Threshold Center** and **Slope** alone, so you can apply one and still tilt it your way. Presets shape the *threshold curve* only; they do not change the ratios, offsets, attack or release.

**Saving your own.** Click **Save**, type a name, and Slackwater stores the current curve (including the centre and slope) as a small file in the presets folder (see *Installing*). Your preset then appears in the list in every project. **Del** removes the selected preset of your own; the built-in ones cannot be deleted.

## Modes in depth

### Pink Noise

The default. The thresholds are a fixed curve that you set. With the neutral slope, the curve follows pink noise, so material with a natural balance sits comfortably around it.

Use it when you know the shape you want (flat and natural, or one you have drawn), and when there is no other sound to follow. It is the right mode for tonal balancing a mix bus, a vocal or a single instrument.

### Sidechain Matching

The threshold at each frequency becomes the loudness of the *sidechain* sound at that frequency, plus your curve and offsets. In plain terms: **your sound is pulled toward the tonal balance of the sidechain sound.**

Examples:

- Send a reference track to the sidechain and use Slackwater on your mix to nudge it toward the reference's balance.
- Put Slackwater on a pad and send the lead to the sidechain, so the pad automatically takes the same tonal shape and stays out of the way.
- Send one microphone to another channel's sidechain to make two sources blend.

Set the **Downwards Ratio** high to hold your sound down to the sidechain's balance, and the **Upwards Ratio** above 1:1 if you want gaps to be filled in as well. The **Global Threshold** now means "how far above or below the sidechain's level", and the Offsets move each compressor from there.

> **Careful:** In Sidechain Matching mode, if nothing is coming into the sidechain input, the thresholds fall to the floor and anything above them is turned down hard whenever the Downwards Ratio is above 1:1. If your sound disappears, check the sidechain routing first, or switch back to Pink Noise.

### Sidechain Compression

The sidechain sound *controls* the compression. Wherever the sidechain has energy at a frequency, your sound is compressed at that frequency, using the thresholds you have drawn. Where the sidechain is quiet, your sound can pass (or be lifted by the Upwards compressor).

This makes a very precise ducking effect. A classic use is turning down the part of a bass or pad that collides with a vocal, only at the frequencies where the vocal has energy.

### Using the sidechain input

Slackwater has a second audio input called the sidechain. How you reach it depends on your host, but the idea is always the same: choose another track (or bus) as the sidechain source for the plug-in. Look for a menu or button labelled *sidechain*, *side chain*, *key* or *external input* in the plug-in's header or its routing options. Your host's manual will tell you exactly where.

The sidechain has the same number of channels as the track (mono track, mono sidechain; stereo track, stereo sidechain). The sidechain sound is only *listened to*; it is never heard in the output.

## Freeze and impulse responses

**Freeze** captures the frequency-by-frequency gain shape at the moment you switch it on, and keeps applying that same shape no matter what the music does afterwards. You are effectively turning Slackwater into a fixed, very detailed EQ whose curve was drawn from the music.

Ways to use it:

- Match a mix, freeze the result when it sounds right, and keep that tonal correction while you change the rest of the song.
- Capture the balance of a reference and apply it as a steady EQ.
- Create unusual, static, spectral filters from any sound.

Turn Freeze off to return to live processing. Every time you turn Freeze on it captures a fresh shape, and it captures again if you change the Window Size or stop and restart playback.

### Exporting an impulse response

While Freeze is on, **Export IR…** saves the frozen curve as a **WAV impulse response**: a 32-bit floating-point file with one channel per input channel, twice as long as the window. You can load it into a convolution plug-in to get the same tonal shaping in a different place, or share it with someone else.

A few details worth knowing:

- The impulse response is **centred**: its main peak sits in the middle of the file, with the "pre-ringing" before it and the tail after it. The file name tells you how many samples in the peak is (for example, `slackwater-freeze-ir-peak-2048smp.wav`).
- Convolving with the file **delays your signal by that many samples**, so allow for it in your convolution plug-in if exact alignment matters.
- The file is made at your session's sample rate. If you plan to use it at a different sample rate, export again.
- The file is the sound Slackwater makes when it processes a single click. It reproduces the *frequency shape* of the frozen curve. It is not an exact copy of the live, moving processing.

If something goes wrong while saving (for example, the folder is read-only), Slackwater tells you in the status line at the top.

## Level matching, Delta and Bypass

Three small tools make Slackwater easier to judge.

**Match** (the loudness button in the header). Because compression changes loudness, it is easy to be fooled into thinking a louder version is better. Click **Match**, let five seconds of audio play, and Slackwater sets **Output Gain** so the processed sound has the same overall level as the original. If Bypass was on, Match turns it off for you and ends Delta mode.

**Delta.** Listening to the difference is the best way to understand what you are doing. If the Delta sound is mostly harsh or boomy, you are removing the right things. If it contains music you want to keep, ease the ratios or raise the thresholds.

**Bypass.** The bypassed sound is delayed by the same amount as the processed sound, so switching back and forth never changes the timing and does not leave a click.

## Recipes

These are starting points. Every piece of music is different, so use them as sketches and adjust by ear. Unless stated, leave the other controls at their defaults.

### Gentle tonal polish for a finished mix

1. Load the **Master Balanced** curve preset.
2. Set **Downwards Ratio** to about **2:1** and **Upwards Ratio** to about **1.5:1**.
3. Set **Smoothing** to **60%** and **Window Size** to **4096**.
4. Lower **Global Threshold** a little at a time until you see the amber bars moving gently on the loudest frequencies.
5. Press **Match**, then use **Bypass** to compare.

### Taming a harsh vocal

1. Raise **Downwards Ratio** to about **4:1**, **Upwards Ratio** stays at 1:1.
2. Set **Attack** to about **20 ms** and **Release** to about **200 ms**.
3. Click **Match** under *Curve Matching* while the vocal plays, so the curve sits right on the vocal's average balance.
4. Raise the **Downwards Offset** by a few decibels so that only the peaks above the average are turned down.
5. Use **Delta** to check that you are removing harshness, not the body of the voice.

### Bringing a dull sample to life

1. Raise **Upwards Ratio** to about **3:1** and keep **Hi-Freq Rolloff** at 75% so the treble does not turn to hiss.
2. Raise the **Upwards Offset** until the blue bars rise in the frequencies that feel missing.
3. Add a small **Downwards Ratio** (about 2:1) so that nothing jumps out as the quiet parts come up.

### Making a pad sit under a lead

1. Put Slackwater on the pad and set **Mode** to **Sidechain Matching**.
2. Route the lead track to the sidechain input.
3. Raise **Downwards Ratio** to about **3:1** and lower the **Global Threshold** until the pad ducks quietly under the lead's tone.
4. Use a long **Release** (about 500 ms) so the pad does not pump.

### Ducking the low-mids under a vocal

1. Put Slackwater on the bass or pad and set **Mode** to **Sidechain Compression**.
2. Route the vocal to the sidechain input.
3. Raise **Downwards Ratio** to about **4:1** and bring the thresholds down until the amber bars show up only around the vocal's weight.
4. Keep **Mix** below 100% to blend the effect in gently.

### Pink-noise flattening for creative effect

1. Choose a **Window Size** of **8192** or bigger and a long **Attack**.
2. Raise both **Ratios** to about **20:1** and set the **Upwards Offset** and **Downwards Offset** to the same value.
3. Everything, whatever its original balance, is pulled toward the pink noise shape. Slowly raise **Smoothing** for a more natural, less "robotic" feel.

> **Tip:** All six recipes become more convincing if you finish by pressing **Match**, so you are comparing balance rather than loudness.

## Getting the best performance

- Slackwater is light on a modern computer at its defaults. The settings that cost the most CPU are a **large window** and a **high overlap**.
- If your computer struggles, first lower **Window Overlap** (try 8x and listen), then reduce **Window Size**.
- The **analyzer** only draws while the plug-in window is open, so closing the window reduces the work slightly.
- Changing the window size changes the delay. If your host does not update its delay compensation immediately, stop and restart playback.

## Troubleshooting and questions

**Nothing happens when I insert it.**
That is on purpose: both ratios start at 1:1. Raise the Downwards or Upwards Ratio.

**The analyzer is empty.**
It only shows something while audio is playing through the plug-in. Press play.

**My sound is very quiet or has disappeared in Sidechain Matching.**
No sidechain signal is arriving, so the thresholds are at the floor. Route a track to the sidechain input, or switch the Mode to Pink Noise.

**The Match button says "signal too quiet".**
Slackwater could not measure enough level in five seconds. Make sure audio is actually playing (the transport must be running) and that it is not silent.

**The sound has a short fade-in when I change Window Size.**
Changing the window restarts the analysis. Set the size first, then adjust the rest.

**Switching Bypass on and off changes the loudness.**
That is the point of the processing. Use **Match** to equalise loudness, then compare.

**The track is late or early compared with the others.**
Slackwater reports its delay to your host, which should compensate. If you hear a shift, make sure your host's automatic delay compensation is on. A smaller **Window Size** reduces the delay.

**I hear a swishy, underwater sound.**
Typical of very high ratios with a very short attack and release. Increase **Attack** and **Release**, raise **Smoothing**, or increase **Window Overlap**.

**I hear a harsh or hissy top end.**
Raise **Hi-Freq Rolloff** on whichever compressor is doing the lifting or squashing, or lower the **Upwards Ratio**.

**Can I automate it?**
Yes. All of the controls above are available to your host's automation. The curve points you draw in the analyzer are saved with your project, but are not individually automatable.

**Is there a standalone app?**
Not at present. Use Slackwater inside a host.

**Does it work in mono?**
Yes. On a mono track it is a mono plug-in with a mono sidechain.

**My presets disappeared after an update.**
They live in their own folder (see *Installing*), which updates never touch. Check that the folder is still there.

## Technical notes

This chapter is for people who want to know exactly what the plug-in does.

### Signal flow

1. The input is copied into a delay line (for Mix, Delta and Bypass).
2. The signal passes through an overlap-add **short-time Fourier transform**: each window is multiplied by a Hann window, transformed to the frequency domain, processed, transformed back, multiplied by a second Hann window and added into the output. The window size sets both the frequency resolution and the latency.
3. In the frequency domain, each bin's magnitude drives a **per-bin envelope follower** with the Attack and Release times. The followers advance once per *hop*, which is the window size divided by the overlap.
4. Each bin's envelope level (in dB) is passed through the **Downwards** and **Upwards** compressor curves, each with its own threshold, ratio and soft knee. The gain applied to the bin is the sum of the two stages' changes.
5. The gain curve is optionally **smoothed** across frequency, then applied to the bin.
6. The result is transformed back, windowed and summed. The **Output Gain** is applied to the processed signal (smoothed over about 20 ms so moves and Match are click-free), and **Mix** blends in the delayed original.

The latency equals the window size in samples.

### Threshold curve

For each bin the base threshold is

`threshold(f) = Global Threshold + Slope × x + Curve × x²`, where `x = ln(f) − ln(Threshold Center)`.

The curve points are then added on top by interpolating (with a smooth S-shaped blend) between the nearest points on each side, carrying the first and last point's value outward. Each compressor adds its Offset to the result.

In **Pink Noise** mode the Slope parameter has 3 subtracted from it internally, which is why 3.00 on the display is the neutral position. In the sidechain modes the stored slope is applied as it is, and the display is the same, so 3.00 again means "no tilt".

> **Advanced:** The Slope and Curve values are measured against the *natural logarithm* of frequency, in which one octave is 0.69 of a unit. A displayed value labelled "dB/oct" therefore changes the threshold by about 0.69 times that number of decibels per octave. The "neutral pink noise" slope of 3.00 is accordingly about 2.1 dB per octave, a little gentler than the 3 dB per octave that true pink noise falls. The analyzer's display tilt uses the same convention, so the two always agree with each other.

### The compressors

For an envelope level `e` in dB, threshold `T`, ratio `R` and knee width `W`:

- **Downwards:** below `T − W/2` the level passes unchanged; above `T + W/2` the output is `T + (e − T) / R`; between the two, a quadratic curve joins them smoothly.
- **Upwards:** the mirror image: above `T + W/2` the level passes unchanged; below `T − W/2` the output is `T + (e − T) / R`; a quadratic joins them.
- The gain in dB for the bin is `down(e) + up(e) − 2e`.
- The Upwards stage is skipped for the lowest bins (up to about 20 Hz) and for bins with no signal at all, so it cannot raise DC or silence.

**Hi-Freq Rolloff** scales the reciprocal of the ratio: `1/R' = (1/R) × (1 − t) + t`, with `t = Rolloff × ln(f) / ln(22 050 Hz)`. At 100%, `R'` reaches 1 at 22 050 Hz regardless of the sample rate.

**Smoothing** averages the gain curve (in dB) over a window of `±Smoothing × 1 octave` on a logarithmic frequency scale, then blends that average with the unsmoothed value by the Smoothing amount. Bins at and below 20 Hz are never allowed to receive a boost from smoothing.

### Sidechain modes

- **Sidechain Matching:** each bin's threshold has the sidechain's level in dB at that bin added to it (with channel linking applied), and the soft-knee shape is recomputed per bin from that threshold.
- **Sidechain Compression:** the envelope followers read the sidechain's magnitudes instead of the main input's, and the main signal is compressed against your drawn thresholds.
- **SC Channel Link** mixes the channels' sidechain magnitudes: with `n` channels, each channel takes `1 − Link × (n − 1) / n` of its own magnitude and `Link / n` of every other channel's.

### Freeze and export

On the first processed hop after Freeze is turned on, the gain curve (after smoothing) is stored per channel and reapplied unchanged until Freeze is turned off, the window size changes, or the plug-in is reset. **Export IR…** renders a unit impulse through the same overlap-add pipeline with the stored gains and writes a window centred on the response, of length two windows, so the pre-ringing is kept.

### Match

**Match (level)** adds up the energy at the input and at the output of the processing (before Output Gain) for five seconds, then sets Output Gain to the square root of the ratio, limited to ±50 dB. **Curve Matching** averages the magnitude spectrum over five seconds, subtracts the Downwards Offset, fits an intercept and bend by least squares around your Threshold Center and Slope, and then, only if the fit is off by more than 1.5 dB somewhere, adds up to eight points at the largest remaining errors (each bigger than 3 dB, and at least about a third of an octave apart).

### Parameters

All parameters can be automated except the hidden curve points. Names are as the host shows them.

| Parameter | Range | Default |
| --- | --- | --- |
| Output Gain | −50 to +50 dB | 0 dB |
| Bypass | off / on | off |
| Mix | 0 to 100% | 100% |
| Window Size | 64 to 32768 samples (powers of two) | 2048 |
| Window Overlap | 4x to 32x (powers of two) | 16x |
| Attack | 0 to 10000 ms | 150 ms |
| Release | 0 to 10000 ms | 300 ms |
| Freeze | off / on | off |
| Global Threshold | −100 to +20 dB | −12 dB |
| Threshold Center | 20 Hz to 20 kHz | 1 kHz |
| Threshold Slope | stored −36 to +36 | 0 (shown as 3.00) |
| Threshold Curve | −24 to +24 | 0 |
| Mode | Pink Noise / Sidechain Matching / Sidechain Compression | Pink Noise |
| SC Channel Link | 0 to 100% | 80% |
| Smoothing | 0 to 100% | 0% |
| Upwards Offset, Downwards Offset | −50 to +50 dB | 0 dB |
| Upwards Ratio, Downwards Ratio | 1:1 to 500:1 | 1:1 |
| Upwards Hi-Freq Rolloff | 0 to 100% | 75% |
| Downwards Hi-Freq Rolloff | 0 to 100% | 0% |
| Upwards Knee, Downwards Knee | 0 to 36 dB | 6 dB |

The eight curve points each have an enabled flag, a frequency (30 Hz to 22 kHz) and an offset (−60 to +60 dB). They are saved in your project, but hidden from the host's list of parameters.

### Formats and identity

| Format | Identifier |
| --- | --- |
| VST3 | class ID `OsklSlackwater01` |
| CLAP | `com.oskillator.slackwater` |
| Audio Unit | type `aufx`, subtype `Slkw`, manufacturer `Oskl` |

Audio layouts: mono in / mono out with a mono sidechain, and stereo in / stereo out with a stereo sidechain.

### Presets file format

A user preset is a small JSON file with a `version`, a `name`, the five curve numbers and eight points. You can copy preset files between computers, and you can edit them by hand. Files that are not valid are skipped (and mentioned in the host's log) rather than loaded.

## Glossary

**Analyzer:** the live display of the spectrum, the thresholds and the gain changes.

**Attack / Release:** how fast a compressor reacts when sound gets louder / how slowly it lets go when sound gets quieter.

**Bin:** one thin slice of the spectrum, the unit that Slackwater compresses on its own.

**Compressor:** a processor that reduces the difference between loud and quiet.

**dB (decibel):** the unit of loudness. A change of about 1 dB is the smallest most people can hear; 6 dB is roughly "twice or half as loud" in signal level.

**Delta:** the difference between the original and the processed sound.

**Downwards compression:** turning down sound that is *above* a threshold.

**Envelope follower:** the part of a compressor that tracks the loudness of a sound over time.

**FFT (Fast Fourier Transform):** the maths that turns a slice of sound into its spectrum.

**Hop:** the time between one analysis window and the next. Window size divided by overlap.

**Knee:** the shape of the corner at the threshold. Hard is abrupt, soft is gradual.

**Latency:** the delay a plug-in adds. Slackwater's equals its window size in samples.

**Overlap:** how many windows are analysed at once. More overlap is smoother.

**Pink noise:** noise with equal energy in every octave. It sounds natural and is used as a model for a balanced spectrum.

**Ratio:** how strongly a compressor pulls sound toward its threshold. 1:1 does nothing.

**Sidechain:** a second audio input that is listened to but not heard.

**Spectrum:** a picture of how much of each frequency is in a sound.

**Threshold:** the level at which compression starts. In Slackwater, a whole curve of them across frequency.

**Upwards compression:** turning up sound that is *below* a threshold.

**Window:** the short slice of audio that is analysed at once.

## Credits and licence

Slackwater is made by Oskillator.

It is built on the work of two earlier projects, and the plug-in is free software under the **GNU General Public Licence, version 3 or later**:

- **Spectral Compressor** by Robbert van der Helm, the original open-source plug-in this design comes from.
- **Polarity-SC-Dark** by Robert Agthe, the port and its curve editing, matching and freeze features.

Slackwater is written in Rust using the nih-plug framework and the VIZIA interface library. The Audio Unit is made with clap-wrapper. The typefaces are **Sora** and **DM Mono**, both licensed under the SIL Open Font Licence.

The complete source code, including this manual, is available from the project's repository, and you are welcome to read, share and change it under the terms of the licence.
