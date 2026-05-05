# Corrosion UI Interface Design Brief

This document describes every user-facing input in the current simple Corrosion plugin UI, what each control does, how controls change when models are selected, and the intended visual direction for a future designed interface. It also includes a detailed prompt for a graphic-designer AI to create a full interface concept.

Corrosion is an industrial physical-modeling instrument. It should not look like a clean subtractive synth, a glossy EDM plugin, or a generic modular panel. The interface should communicate physical objects under stress: struck metal, corroded surfaces, machine pressure, friction, heat, structural damage, and controlled violence.

## Current UI Structure

The current interface is intentionally simple. It uses native egui widgets: combo boxes, sliders, collapsible sections, and three main columns.

Window behavior:
- Base size: `1440x1024` at 100% UI scale.
- UI scale changes both the size of the interface elements and the requested editor window size.
- Scale choices:
  - `50%`: `720x512`
  - `75%`: `1080x768`
  - `100%`: `1440x1024`
  - `125%`: `1800x1280`
  - `150%`: `2160x1536`

Layout:
- Top global strip: UI scale, output, stereo width, body amount.
- Left column: Exciter model, exciter model settings, envelope, envelope modulation, interaction.
- Middle column: Resonator model, resonator model settings, damage and material.
- Right column: Processing, filter, drive, body/spread, space, limiter.

## Important Interaction Model

Several controls are dynamic. Changing a model does not create a new independent set of host parameters. Instead, the UI changes labels and visible sections while reusing shared parameter slots.

Examples:
- Exciter models all share three core model-setting slots: `exciter_pressure`, `exciter_speed`, and `exciter_roughness`.
- Resonator models share four main model-setting slots: `size`, `res_damping`, `res_brightness`, and `thickness`.
- Space modes share a global amount control and then expose only the controls for the chosen space model.

This means the UI designer should make dynamic sections feel intentional: the label and context changes matter. The user should understand that each model reinterprets the same control positions as physically relevant settings for that model.

## Global Inputs

### UI Scale

Type: combo box.

Choices:
- `50%`
- `75%`
- `100%`
- `125%`
- `150%`

What it does:
- Resizes the plugin UI elements.
- Requests a matching plugin window size.
- Does not affect audio.

Design intent:
- This should be a utility control, not a sound-design control.
- It can live in a small top-right settings area.
- It should be visually quiet.

### Output

Type: slider.

Range shown in UI: `0.0` to `10.0`.

What it does:
- Controls final output gain before the hard output limiter.
- Higher values make the plugin louder and can drive the final limiter harder.

Design intent:
- Treat this as the final master level.
- It should be clearly visible but not more visually dominant than the object/exciter controls.
- Use a simple level fader or horizontal output slider.

### Width

Type: slider.

Range shown in UI: `-2.0` to `3.0`.

What it does:
- Controls stereo modal spread during voice processing.
- Low or centered values reduce stereo spread.
- Higher values widen modal radiation and spatial separation.

Design intent:
- This is a spatial control, not a chorus effect.
- Represent it with a stereo field icon, left/right rails, or subtle width meter.

### Body

Type: slider.

Range shown in UI: `0.0` to `5.0`.

What it does:
- Adds low-mid body reinforcement through the post-processing body path.
- In runtime it contributes to body amount alongside chassis volume.

Design intent:
- Should feel like adding the resonating mass of a physical chassis or cavity.
- Use imagery such as a hull, tank body, or resonance chamber, not a generic EQ bump.

## Exciter Column

The exciter is the force source. It decides how energy is introduced into the object: strike, scrape, brush, pipe impact, chain drop, jet, hum, snap, or debris.

### Exciter Model

Type: combo box.

Choices:
- `Hit`
- `Scrape`
- `Hand Strike`
- `Felt Mallet`
- `Hard Mallet`
- `Drumstick`
- `Wire Brush`
- `Metal Pipe`
- `Metal Chain`
- `Stiff Point`
- `Heavy Grinding`
- `Corrugated Drag`
- `Tension Rise`
- `Pneumatic Jet`
- `Electromagnetic Hum`
- `Tension Snap`
- `Particle Rain`

What it does:
- Selects the exciter algorithm used for new notes.
- Changes the visible meaning of the three model-setting sliders.
- Changes envelope behavior by exciter family.

Design intent:
- This should be one of the most prominent selectors.
- It defines the gesture: hit, scrape, friction, air, electricity, tension, debris.
- It should be grouped with a clear label such as `EXCITER / FORCE SOURCE`.

## Exciter Model Settings

All exciter models use three shared sliders internally. The UI relabels those sliders depending on the selected exciter.

Underlying shared controls:
- First slider: `exciter_pressure`
- Second slider: `exciter_speed`
- Third slider: `exciter_roughness`

All three use a normalized `0.0` to `1.0` range in the UI.

### Hit

Visible labels:
- `Level`
- `Speed`
- `Tone`

What it does:
- Uses a simple impulse-style hit path.
- Level, speed, and tone are broad labels for impact strength, transient speed, and brightness/grit behavior.

Design intent:
- Should feel like the most immediate default strike.
- Use impact language: strike mark, hammer blow, impulse, initial force.

### Hand Strike

Visible labels:
- `Hand Mass`
- `Palm Stiffness`
- `Skin Damping`

What it does:
- Maps shared controls to hand mass, stiffness, and damping behavior.
- Produces a fleshy, damped strike that can mute high frequencies.

Design intent:
- Soft but heavy physical contact.
- Avoid making it look delicate; it is still industrial, like a gloved palm slamming sheet metal.

### Felt Mallet

Visible labels:
- `Mallet Mass`
- `Hardness`
- `Soft Curve`

What it does:
- Shapes a softer mallet contact with a hard core response as force increases.
- Higher hardness increases attack definition.
- The curve changes how quickly the soft mallet bottoms out into a harder impact.

Design intent:
- Padded impact with controlled force.
- Visually: worn rubber, fabric-wrapped hammer, compressed felt.

### Hard Mallet

Visible labels:
- `Mallet Mass`
- `Stiffness`
- `Damping`

What it does:
- Controls a heavier rigid-body impact.
- Mass increases drive into the object.
- Stiffness increases brightness and bite.
- Damping reduces uncontrolled bounce.

Design intent:
- Heavy tool impact, hard rubber or plastic on metal.

### Drumstick

Visible labels:
- `Stick Mass`
- `Stiffness`
- `Rebound`

What it does:
- Controls a light rigid stick with micro-bounce behavior.
- Rebound affects how many fast re-strikes can happen.

Design intent:
- Quick, sharp, woody/rigid tool energy, but still on metal.

### Wire Brush

Visible labels:
- `Wires`
- `Sweep`
- `Spread`

What it does:
- Controls the density, duration/sweep, and randomness/spread of brush-like micro-impulses.

Design intent:
- Many small metal contacts over time.
- Visual metaphor: bristles, sparks, scratch traces.

### Metal Pipe

Visible labels:
- `Pipe Mass`
- `Stiffness`
- `Pitch`

What it does:
- Controls a metal-on-metal pipe impact exciter.
- Mass controls force, stiffness controls collision hardness, pitch shifts the internal pipe-like resonant behavior.

Design intent:
- A loose pipe or metal bar striking another industrial object.

### Metal Chain

Visible labels:
- `Links`
- `Speed`
- `Rattle`

What it does:
- Controls chain hit density, drop/spread speed, and high-frequency internal rattle.

Design intent:
- Linked metal, scattered impacts, unstable transient clusters.

### Scrape

Visible labels:
- `Pressure`
- `Speed`
- `Roughness`

What it does:
- Controls a stick-slip scrape/friction source.
- Pressure increases contact force.
- Speed changes friction motion.
- Roughness increases grip, rasp, and instability.

Design intent:
- This is the core sustained friction gesture.
- Visual language: drag marks, brake squeal, pressure gauge, metal dust.

### Stiff Point

Visible labels:
- `Speed`
- `Pressure`
- `Chatter`

What it does:
- Controls a sharp point scraping the resonator.
- Chatter emphasizes high-frequency micro-snaps.

Design intent:
- Nail, awl, pick, or sharp tool dragged over metal.

### Heavy Grinding

Visible labels:
- `Speed`
- `Pressure`
- `Grit`

What it does:
- Controls a heavier rough scrape/grind.
- Grit increases tearing noise and abrasive texture.

Design intent:
- Concrete, sandpaper, grinder, rough machine contact.

### Corrugated Drag

Visible labels:
- `Speed`
- `Spacing`
- `Depth`

What it does:
- Controls rhythmic dragging across ridges or grate-like bumps.
- Spacing affects bump frequency.
- Depth affects impact intensity between ridges.

Design intent:
- Ribbed metal, grates, corrugated sheets, repeated mechanical bumps.

### Tension Rise

Visible labels:
- `Pull Speed`
- `Threshold`
- `Stochasticity`

What it does:
- Controls force buildup and release behavior.
- Pull Speed accumulates tension.
- Threshold sets how much force is required before slipping.
- Stochasticity adds instability to the release timing.

Design intent:
- Creaking cable, stressed metal, avalanche slip, mechanical pressure building toward failure.

### Pneumatic Jet

Visible labels:
- `Pressure`
- `Nozzle Width`
- `Chaos`

What it does:
- Controls a turbulent air/steam jet source.
- Pressure drives intensity.
- Nozzle width changes tonal focus.
- Chaos adds turbulence and non-linear instability.

Design intent:
- Steam valve, compressed air leak, pneumatic force hitting metal.

### Electromagnetic Hum

Visible labels:
- `Mains Frequency`
- `Field`
- `Voltage Sag`

What it does:
- Controls a continuous electromagnetic drive.
- Frequency shifts the hum base.
- Field controls drive/proximity.
- Voltage Sag increases unstable harmonic distortion.

Design intent:
- Transformer, motor housing, power station hum, failing electrical field.

### Tension Snap

Visible labels:
- `Pull Distance`
- `Hook Stiffness`
- `Snap Force`

What it does:
- Controls a catch-and-release force source.
- Pull Distance changes how far force builds.
- Hook Stiffness changes how rigid the connection is.
- Snap Force changes the breaking threshold.

Design intent:
- Cable snapping, gear catch release, stressed wire breaking free.

### Particle Rain

Visible labels:
- `Flow`
- `Particle Mass`
- `Mass Variance`

What it does:
- Controls a stochastic stream of small impacts.
- Flow changes density.
- Particle Mass changes individual impact weight.
- Mass Variance randomizes particles for organic debris motion.

Design intent:
- Gravel, debris, bolts, rust flakes, industrial particles falling onto metal.

## Exciter Envelope Inputs

Envelope controls are dynamic by exciter family.

### Hit Family Envelope

Shown for:
- `Hit`
- `Hand Strike`
- `Felt Mallet`
- `Hard Mallet`
- `Drumstick`
- `Wire Brush`
- `Metal Pipe`
- `Metal Chain`

#### Attack

Range: `0.001` to `2.0` seconds.

What it does:
- Controls how quickly the strike force rises.
- Lower values make sharper impacts.
- Higher values soften the beginning of the strike.

#### Release

Range: `0.01` to `5.0` seconds.

What it does:
- Controls how quickly the strike force releases or leaves the object.
- For some hit exciters, release also influences mute/decay behavior.

### Scrape Family Envelope

Shown for:
- `Scrape`
- `Stiff Point`
- `Heavy Grinding`
- `Corrugated Drag`
- `Tension Rise`

This family uses a 6-stage MSEG for continuous friction gestures.

#### Onset

Range: `0.001` to `1.0` seconds.

What it does:
- Controls the initial contact phase before the scrape fully bites.

#### Attack

Range: `0.001` to `2.0` seconds.

What it does:
- Controls the rise into maximum pressure or friction energy.

#### Hold

Range: `0.0` to `2.0` seconds.

What it does:
- Controls how long the gesture stays at peak before settling.

#### Decay

Range: `0.01` to `5.0` seconds.

What it does:
- Controls the fall from peak into sustain.

#### Sustain

Range: `0.0` to `1.0`.

What it does:
- Controls the sustained force level while the note is held.

#### Release

Range: `0.01` to `5.0` seconds.

What it does:
- Controls the lift-off or fade-out after note release.

#### Loop

Type: combo box.

Choices:
- `Off`
- `Forward`
- `Ping-Pong`

What it does:
- Controls whether the MSEG loops while the note is held.
- Forward repeats in one direction.
- Ping-Pong alternates direction, useful for back-and-forth scraping.

#### Loop Start

Type: combo box.

Choices: `0`, `1`, `2`, `3`, `4`, `5`.

What it does:
- Selects the MSEG stage where looping begins.

#### Loop End

Type: combo box.

Choices: `0`, `1`, `2`, `3`, `4`, `5`.

What it does:
- Selects the MSEG stage where looping ends or reverses.

### Specialty Family Envelope

Shown for:
- `Pneumatic Jet`
- `Electromagnetic Hum`
- `Tension Snap`
- `Particle Rain`

#### Attack

Range: `0.001` to `2.0` seconds.

What it does:
- Controls how quickly the specialty source starts.

#### Decay

Range: `0.01` to `5.0` seconds.

What it does:
- Controls the fall from attack peak to sustain level.

#### Sustain

Range: `0.0` to `1.0`.

What it does:
- Controls the held force level.

#### Release

Range: `0.01` to `5.0` seconds.

What it does:
- Controls how quickly the specialty source ends after note release.

## Envelope Modulation Inputs

These controls shape how the envelope responds to velocity and time.

### Env Amount

Range: `0.0` to `1.0`.

What it does:
- Controls the strength of the envelope modulation.
- Most relevant to MSEG-controlled scrape behavior.

### Velocity To Peak

Range: `0.0` to `1.0`.

What it does:
- Controls how much MIDI velocity raises the peak envelope level.
- Higher values make harder notes produce stronger friction/force peaks.

### Global Time

Range: `0.1` to `10.0`.

What it does:
- Scales envelope timing globally.
- Lower values compress the gesture; higher values stretch it.

### Velocity To Level

Range: `0.0` to `1.0`.

What it does:
- Controls how much MIDI velocity changes overall envelope level.

### Velocity To Time

Range: `0.0` to `1.0`.

What it does:
- Controls how much MIDI velocity affects timing.
- Higher values make harder notes alter timing more strongly.

### Curve

Range: `-1.0` to `1.0`.

What it does:
- Shapes MSEG curve tension.
- Negative and positive values bend envelope segments in opposite directions.

## Interaction Inputs

Interaction controls determine how the exciter force is applied to the resonator.

### Strike Position

Range: `0.0` to `1.0`.

What it does:
- Moves the point where the exciter contacts the object.
- Changes modal emphasis and timbre without changing the selected object.

Design intent:
- This is a physical contact-location control.
- It should be visualized as a contact point moving across a pipe, plate, chain, cable, or object silhouette.

### Coupling

Range: `0.0` to `1.0`.

What it does:
- Controls how strongly the exciter and resonator interact.
- Lower values feel more feed-forward.
- Higher values feel more physically connected.

### Position Wander

Range: `0.0` to `1.0`.

What it does:
- Adds movement or instability to the contact point.
- Useful for human-like scraping, unstable mechanical contact, or drifting pressure.

### Position Envelope

Range: `0.0` to `1.0`.

What it does:
- Lets the envelope move the strike/contact position.

### Fundamental Anchor

Range: `0.0` to `1.0`.

What it does:
- Preserves the fundamental even when strike position would naturally reduce it.
- Higher values keep the note center stronger and more stable.

## Resonator Column

The resonator is the object being excited. It defines what is vibrating.

### Resonator Model

Type: combo box.

Choices:
- `Pipe`
- `Plate`
- `Tank`
- `Chain`
- `I-Beam`
- `Taut Cable`
- `Coil Spring`
- `Sheet Metal`
- `Industrial Cog`

What it does:
- Selects the modal object profile used for new notes.
- Changes the visible labels for model setting sliders.

Design intent:
- This should be the second most important selector after Exciter Model.
- It should feel like choosing a physical object, not choosing an oscillator waveform.

## Resonator Model Settings

All resonator models share four control slots:
- `size`
- `res_damping`
- `res_brightness`
- `thickness`

The labels change per selected object.

### Pipe

Visible labels:
- `Pipe Length`
- `Wall Loss`
- `Tube Ring`
- `Wall Thickness`

What it does:
- Length changes perceived size and pitch scale.
- Wall Loss changes damping.
- Tube Ring changes brightness/resonant emphasis.
- Wall Thickness changes stiffness/inharmonicity.

### Plate

Visible labels:
- `Plate Size`
- `Edge Loss`
- `Metal Brightness`
- `Plate Thickness`

What it does:
- Controls a flat metal object with more inharmonic clang.

### Tank

Visible labels:
- `Tank Volume`
- `Cavity Loss`
- `Shell Ring`
- `Wall Thickness`

What it does:
- Controls a heavier cavity-like object with low-mid body.

### Chain

Visible labels:
- `Link Mass`
- `Friction Decay`
- `Link Brightness`
- `Link Gauge`

What it does:
- Controls linked-metal modal behavior with dense transient complexity.

### I-Beam

Visible labels:
- `Beam Mass`
- `Rigidity Damping`
- `Shear Brightness`
- `Beam Mass`

What it does:
- Controls heavy girder-like resonance.
- Emphasizes mass, rigidity, and compressed high-frequency behavior.

### Taut Cable

Visible labels:
- `Cable Tension`
- `Tension Loss`
- `Braid Brightness`
- `Wire Gauge`

What it does:
- Controls stiff cable-like resonance.

### Coil Spring

Visible labels:
- `Coil Length`
- `Friction`
- `Spring Slosh`
- `Wire Gauge`

What it does:
- Controls spring-like resonance, dispersion, and metallic slosh.

### Sheet Metal

Visible labels:
- `Sheet Size`
- `Edge Damping`
- `Sheet Brightness`
- `Thinness`

What it does:
- Controls thin, broad, unstable sheet-metal resonance.

### Industrial Cog

Visible labels:
- `Blade Radius`
- `Friction Decay`
- `Tooth Ring`
- `Blade Thickness`

What it does:
- Controls circular/cog-like metallic ringing and mode splitting.

## Damage And Material Inputs

### Rust

Range: `0.0` to `5.0`.

What it does:
- Increases corrosion-like damping.
- Reduces brightness and shortens high-frequency decay.
- Makes the object feel more worn and less efficient at ringing.

Design intent:
- Should feel like oxidation, surface loss, dry abrasion, old metal.

### Damage

Range: `0.0` to `10.0`.

What it does:
- Adds structural instability through mode splitting and rattle behavior.
- Higher values create rougher, less stable, more compromised resonance.

Design intent:
- Should feel like cracked welds, loose bolts, bent metal, broken supports.

### Heat

Range: `0.0` to `1.0`.

What it does:
- Adds thermal deformation character to modal profiles.
- Softens and destabilizes the object response.

Design intent:
- Should feel like glowing, warped metal under heat stress.

### Sludge

Range: `0.0` to `1.0`.

What it does:
- Adds mass loading and damping.
- Makes objects darker, heavier, and more muffled.

Design intent:
- Oil, grime, wet corrosion, industrial residue.

## Processing Column

Processing shapes the final sound after the physical model.

## Filter Inputs

### Cutoff

Range: `20.0` to `20000.0` Hz.

What it does:
- Controls post-filter cutoff.
- Lower values darken the signal.

Design intent:
- Should be visually secondary to physical modeling controls.

### Resonance

Range: `0.0` to `1.0`.

What it does:
- Increases filter resonance around the cutoff.

### Tolerance

Range: `0.0` to `1.0`.

What it does:
- Adds component variation/instability character to the filter behavior.

Design intent:
- Represents aging electronics and uneven component behavior.

## Drive Inputs

### Drive Amount

Range: `0.0` to `5.0`.

What it does:
- Controls post-chain chaotic/saturating drive amount.
- Higher values add density, aggression, and harmonic violence.

### Bias Starvation

Range: `0.0` to `1.0`.

What it does:
- Simulates power starvation or bias instability in the drive stage.

### Chaos

Range: `0.0` to `1.0`.

What it does:
- Controls chaotic modulation depth in the drive path.

### Legacy Drive

Range: `0.0` to `5.0`.

What it does:
- Controls the older direct drive stage applied before the full post chain.
- Still affects runtime sound and therefore remains user-facing.

## Body And Spread Inputs

### Chassis Material

Range: `0.0` to `1.0`.

What it does:
- Morphs the post-body resonator material character.

### Chassis Volume

Range: `0.0` to `1.0`.

What it does:
- Controls the amount/size of chassis body resonance.

### Spread

Range: `0.0` to `1.0`.

What it does:
- Controls post-processing stereo spread.
- Separate from the global Width control, which affects voice/modal stereo behavior.

### Listener Proximity

Range: `0.0` to `1.0`.

What it does:
- Controls perceived listener distance/proximity in the spread stage.

## Space Inputs

### Space Mode

Type: combo box.

Choices:
- `Off`
- `Factory`
- `Spring`
- `Echo`

What it does:
- Selects the active space model.
- Controls which space parameters are visible.

### Amount

Range: `0.0` to `1.0`.

What it does:
- Controls wet amount for the selected space mode.

### Factory Space Controls

Shown when Space Mode is `Factory`.

#### Factory Size

Range: `0.0` to `1.0`.

What it does:
- Controls the perceived size of the factory-like space.

#### Machinery Clutter

Range: `0.0` to `1.0`.

What it does:
- Adds density and diffusion from machinery-like obstacles.

#### Wall Impedance

Range: `0.0` to `1.0`.

What it does:
- Changes how reflective or absorptive the virtual walls feel.

### Spring Space Controls

Shown when Space Mode is `Spring`.

#### Spring Tension

Range: `0.0` to `1.0`.

What it does:
- Controls spring wave speed/tension behavior.

#### Wire Stiffness

Range: `0.0` to `1.0`.

What it does:
- Controls stiffness and dispersion in the spring model.

#### Spring Tank Size

Range: `0.0` to `1.0`.

What it does:
- Controls the size/length of the spring tank response.

### Echo Space Controls

Shown when Space Mode is `Echo`.

#### Delay Time

Range: `0.0` to `1.0`.

What it does:
- Controls the base echo delay time.

#### Machinery Movement

Range: `0.0` to `1.0`.

What it does:
- Adds moving-machine modulation to the echo path.

#### High Frequency Damping

Range: `0.0` to `1.0`.

What it does:
- Darkens echo repeats by damping high frequencies.

## Limiter Inputs

### Analog Ceiling

Range: `0.5` to `1.0`.

What it does:
- Sets the ceiling for the analog-style clipper.
- Lower values clip earlier.

### Diode Softness

Range: `0.0` to `1.0`.

What it does:
- Controls the softness/knee of diode-style clipping.
- Higher values preserve more rounded saturation behavior.

## Host Parameter Not Currently Visible In The Simple UI

### Sync Rate

Parameter ID: `sync_rate`.

Range: `0.0` to `1.0`.

What it does now:
- Exists as a host/preset parameter.
- It is not currently displayed in the simple UI and is not actively wired to host tempo behavior in the current runtime.

Design guidance:
- Do not include this prominently until tempo sync is implemented.
- If shown later, place it in an advanced MSEG timing section.

## Visual Theme Definition

### Theme Name

Industrial Physical Metalwork.

### Core Feeling

The interface should feel like operating a controlled machine for striking, scraping, stressing, and damaging industrial objects. It should be functional and readable, but its atmosphere should suggest mass, corrosion, heat, pressure, and dangerous resonance.

The visual identity should be:
- simple enough to use quickly,
- physical enough to communicate object modeling,
- dark enough to support long studio sessions,
- textured enough to avoid generic flat UI,
- restrained enough to avoid novelty steampunk clutter.

### Keywords

- corroded steel
- blackened iron
- oxidized copper and rust
- hazard paint
- pressure gauges
- factory panels
- stamped labels
- heavy bolts
- scorched edges
- worn enamel
- heat tinting
- oil residue
- scratched sheet metal
- stamped industrial typography
- physical contact points
- modal object silhouettes

### Color Palette

Primary background:
- near-black graphite: `#1C1E22`
- dark blue-black steel: `#20242A`
- charcoal panel: `#2B2F36`

Primary text:
- warm off-white: `#DCDAD7`
- muted grey labels: `#A09E9B`
- disabled grey: `#666A70`

Accent colors:
- rust orange: `#A04628`
- hot rust highlight: `#E66E3C`
- hazard yellow: `#DCB428`
- warning amber: `#C87828`
- danger red: `#B43C32`
- cold machinery blue: `#5082AA`

Use accents by function:
- Orange/rust: damage, corrosion, friction, heat.
- Yellow/amber: warnings, space/output/limiter, dangerous gain.
- Blue: resonator/object structure, stereo/spread/filter technical controls.
- Red: high damage, overload, clipping, destructive force.

### Typography

Use strong, readable industrial typography.

Suggested direction:
- Headings: condensed industrial sans, uppercase, slightly squared forms.
- Body labels: clean technical sans, highly legible at small sizes.
- Values: monospaced or tabular numeric font for precision.

Avoid:
- rounded friendly SaaS typography,
- futuristic sci-fi fonts that hurt readability,
- fake stencil fonts everywhere.

### Materials And Texture

The UI should use subtle material language, not heavy photorealism.

Recommended textures:
- lightly scratched steel panels,
- worn painted metal,
- faint rust blooms near panel edges,
- oil smudges in corners,
- stamped metal section headers,
- subtle noise/grain overlays,
- restrained hazard stripes only for important boundaries.

Avoid:
- excessive grunge that makes labels hard to read,
- skeuomorphic knobs that look like stock hardware renders,
- decorative pipes/cogs that do not help usability,
- generic neon synth gradients.

### Control Style

The current UI is simple and should remain functionally simple. A designed version can look more custom, but it should preserve clear control behavior.

Recommended control forms:
- model selectors as large dropdown blocks or segmented selectors,
- primary sliders as horizontal industrial rails,
- values displayed in small tabular readouts,
- dynamic labels that visibly change when the model changes,
- collapsible or tabbed sections for advanced controls,
- contact-position control as a small object diagram if feasible.

Do not hide critical controls behind icons only. Every control needs a label.

### Layout Direction

Keep the current three-zone mental model:

1. Exciter / force source.
2. Resonator / physical object.
3. Processing / space / output.

Recommended arrangement:
- Top header: plugin name, preset area if added later, UI scale/settings, output meter area.
- Left module: Exciter model and force/envelope controls.
- Center module: Resonator model and damage/material controls.
- Right module: Processing, space, limiter, output.

The most important choices should be large and immediate:
- Exciter Model
- Resonator Model
- Space Mode
- Output

Advanced controls can be visually grouped but still accessible.

### Motion And Feedback

Use subtle animation only.

Good feedback ideas:
- small animated contact spark/pressure glow when notes hit,
- resonator silhouette glow based on output level,
- damage/rust controls add subtle texture intensity,
- limiter indicator flashes when clipping/ceiling is hit,
- active space mode shows a faint spatial field.

Avoid:
- bouncing controls,
- gratuitous waveform animations,
- heavy CPU visuals,
- distracting idle movement.

### Accessibility And Readability

- Maintain high contrast between labels and background.
- Do not rely on color alone for section identity.
- Keep numeric values readable.
- Preserve large click/drag targets.
- Make dynamic labels clear and stable.
- Keep the interface usable at 75%, 100%, and 125% scale.

## Graphic Designer AI Prompt

Use the following prompt to generate a complete interface design concept for Corrosion.

```text
Design a VST3/CLAP audio plugin interface for an industrial physical-modeling instrument named "Corrosion".

The plugin is not a normal subtractive synthesizer. It models physical industrial objects being struck, scraped, dragged, stressed, damaged, heated, corroded, and processed through body/space/output stages. The sound identity is corroded metal, unstable resonance, violent impacts, friction, brake squeal, structural steel, chains, tanks, plates, cables, machinery, heat, sludge, and damaged industrial tension.

Design goal:
Create a professional, production-ready plugin UI that is simple to operate but visually distinctive. It should feel like an industrial control surface for physical metal objects, not a generic synth panel. The UI must be clear, readable, and usable in a DAW. Avoid clutter. Avoid overdone steampunk. Avoid glossy EDM plugin aesthetics. Avoid generic flat SaaS UI.

Canvas and scaling:
- Base plugin window: 1440x1024 pixels.
- Must support scaled versions: 50%, 75%, 100%, 125%, 150%.
- Design at 1440x1024 but keep all controls readable when scaled down to 75%.

Overall layout:
- Use a top header and three main vertical modules.
- Top header: plugin name "Corrosion", compact utility area, global controls, output area.
- Left module: EXCITER / FORCE SOURCE.
- Center module: RESONATOR / OBJECT.
- Right module: PROCESSING / SPACE / OUTPUT.
- Use clear section cards or panels with strong headings.
- Advanced sections may be collapsed or visually grouped, but every control must remain discoverable.

Core visual theme:
- Industrial Physical Metalwork.
- Dark graphite and blackened steel background.
- Worn painted metal panels.
- Subtle scratches, rust blooms, oil residue, scorch marks, and stamped labels.
- Restrained hazard-striping for warning/output areas only.
- Heavy bolts, panel seams, and engraved dividers may be used sparingly.
- Use material texture subtly; do not make the UI dirty enough to reduce readability.

Color palette:
- Background: near-black graphite #1C1E22, dark steel #20242A, charcoal panel #2B2F36.
- Text: warm off-white #DCDAD7, muted grey #A09E9B, disabled grey #666A70.
- Rust/corrosion accent: #A04628 and hot rust #E66E3C.
- Hazard/output accent: #DCB428 and amber #C87828.
- Danger/damage accent: #B43C32.
- Technical/resonator/stereo accent: cold machinery blue #5082AA.

Typography:
- Headings: uppercase condensed industrial sans, strong but readable.
- Body labels: clean technical sans.
- Numeric values: tabular or monospaced digits.
- Do not use unreadable sci-fi fonts or decorative stencil fonts for all labels.

Control design:
- Keep controls simple and obvious: sliders, dropdowns, segmented selectors, readouts.
- Model selectors should be prominent dropdown blocks or selector strips.
- Sliders should look like industrial rails or calibrated faders, with numeric readouts.
- Use dynamic labels: when the user changes Exciter Model or Resonator Model, the parameter names change to model-specific physical terms.
- Every control needs a readable text label.
- Avoid unlabeled icon-only controls.

Global top controls:
- UI Scale dropdown: 50%, 75%, 100%, 125%, 150%.
- Output slider.
- Width slider.
- Body slider.

Left module: EXCITER / FORCE SOURCE
This chooses how energy enters the object.

Exciter Model dropdown options:
- Hit
- Scrape
- Hand Strike
- Felt Mallet
- Hard Mallet
- Drumstick
- Wire Brush
- Metal Pipe
- Metal Chain
- Stiff Point
- Heavy Grinding
- Corrugated Drag
- Tension Rise
- Pneumatic Jet
- Electromagnetic Hum
- Tension Snap
- Particle Rain

Exciter Model Settings section:
Show three sliders whose labels change depending on selected model:
- Hit: Level, Speed, Tone
- Hand Strike: Hand Mass, Palm Stiffness, Skin Damping
- Felt Mallet: Mallet Mass, Hardness, Soft Curve
- Hard Mallet: Mallet Mass, Stiffness, Damping
- Drumstick: Stick Mass, Stiffness, Rebound
- Wire Brush: Wires, Sweep, Spread
- Metal Pipe: Pipe Mass, Stiffness, Pitch
- Metal Chain: Links, Speed, Rattle
- Scrape: Pressure, Speed, Roughness
- Stiff Point: Speed, Pressure, Chatter
- Heavy Grinding: Speed, Pressure, Grit
- Corrugated Drag: Speed, Spacing, Depth
- Tension Rise: Pull Speed, Threshold, Stochasticity
- Pneumatic Jet: Pressure, Nozzle Width, Chaos
- Electromagnetic Hum: Mains Frequency, Field, Voltage Sag
- Tension Snap: Pull Distance, Hook Stiffness, Snap Force
- Particle Rain: Flow, Particle Mass, Mass Variance

Envelope section:
- For hit-family exciters: show Attack and Release only. Label as AR strike envelope.
- For scrape-family exciters: show 6-stage MSEG controls: Onset, Attack, Hold, Decay, Sustain, Release, Loop Mode, Loop Start, Loop End.
- For specialty exciters: show simple envelope controls: Attack, Decay, Sustain, Release.

Envelope Modulation section:
- Env Amount
- Velocity To Peak
- Global Time
- Velocity To Level
- Velocity To Time
- Curve

Interaction section:
- Strike Position
- Coupling
- Position Wander
- Position Envelope
- Fundamental Anchor

Interaction visual idea:
Add a small simplified object-contact diagram that shows a contact point moving across an object. The control must remain functional and readable even if the diagram is not implemented.

Center module: RESONATOR / OBJECT
This chooses what physical thing vibrates.

Resonator Model dropdown options:
- Pipe
- Plate
- Tank
- Chain
- I-Beam
- Taut Cable
- Coil Spring
- Sheet Metal
- Industrial Cog

Resonator Model Settings section:
Show four sliders whose labels change depending on selected object:
- Pipe: Pipe Length, Wall Loss, Tube Ring, Wall Thickness
- Plate: Plate Size, Edge Loss, Metal Brightness, Plate Thickness
- Tank: Tank Volume, Cavity Loss, Shell Ring, Wall Thickness
- Chain: Link Mass, Friction Decay, Link Brightness, Link Gauge
- I-Beam: Beam Mass, Rigidity Damping, Shear Brightness, Beam Mass
- Taut Cable: Cable Tension, Tension Loss, Braid Brightness, Wire Gauge
- Coil Spring: Coil Length, Friction, Spring Slosh, Wire Gauge
- Sheet Metal: Sheet Size, Edge Damping, Sheet Brightness, Thinness
- Industrial Cog: Blade Radius, Friction Decay, Tooth Ring, Blade Thickness

Damage and Material section:
- Rust: oxidation, darkening, damping.
- Damage: structural instability, rattle, split modes.
- Heat: thermal warp and softened response.
- Sludge: mass loading, muffling, grime.

Visual idea for Resonator:
Include a small abstract silhouette or icon area that changes by object type: pipe, plate, tank, chain, beam, cable, spring, sheet, cog. Keep it stylized and subtle. It should not become decorative clutter.

Right module: PROCESSING / SPACE / OUTPUT

Filter section:
- Cutoff
- Resonance
- Tolerance

Drive section:
- Drive Amount
- Bias Starvation
- Chaos
- Legacy Drive

Body and Spread section:
- Chassis Material
- Chassis Volume
- Spread
- Listener Proximity

Space section:
- Space Mode dropdown: Off, Factory, Spring, Echo.
- Amount slider.
- If Factory selected: Factory Size, Machinery Clutter, Wall Impedance.
- If Spring selected: Spring Tension, Wire Stiffness, Spring Tank Size.
- If Echo selected: Delay Time, Machinery Movement, High Frequency Damping.

Limiter section:
- Analog Ceiling
- Diode Softness
- Add a clear but small clipping/limiter indicator area.

Output/metering:
- Include a restrained output meter or peak indicator.
- Do not make the meter visually louder than the sound-design controls.
- Use amber/yellow/red only as output approaches limiting.

Visual hierarchy:
1. Exciter Model and Resonator Model are primary.
2. Model Settings and Damage/Material are secondary.
3. Envelope, Interaction, Processing, and Space are tertiary but still easy to access.
4. UI Scale is utility-level and visually quiet.

Tone and mood:
- Dangerous but controlled.
- Heavy but readable.
- Corroded but professional.
- Physical, tactile, machine-like.
- The user should feel they are manipulating force, material, and resonance, not oscillator/filter/amp synth controls.

Do not include:
- Piano keys.
- Oscillator waveform graphics.
- Generic subtractive synth ADSR module styling.
- Purple SaaS gradients.
- Neon cyberpunk clutter.
- Excessive fake rust that harms legibility.
- Tiny unreadable labels.
- Unlabeled knobs.

Deliverables requested from the design AI:
1. Full 1440x1024 plugin interface mockup.
2. Separate close-up of each main module: Exciter, Resonator, Processing.
3. Palette swatches with hex values.
4. Typography recommendations.
5. Control component sheet: dropdown, slider, collapsible section, numeric readout, meter, warning indicator.
6. Notes for how dynamic labels should change when models are selected.
```

## Implementation Notes For Future UI Work

- Preserve parameter labels and model-dependent meaning from this document unless the DSP parameter surface changes.
- Do not reintroduce macro controls unless a new spec explicitly asks for them.
- Keep model selectors obvious.
- Keep dynamic model settings readable; the user should immediately see that choosing a different model changes the physical meaning of the controls.
- Keep the UI simple enough to use without a manual, even if the visual design becomes more atmospheric.
- Any future visual redesign should maintain the `1440x1024` base size and UI scale behavior.
