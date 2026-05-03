# Corrosion Full Feature Surface

This is the consolidated feature and parameter surface derived from the complete roadmap, organized as a working DSP design brief for exciters, objects, transformations, post-processing, macros, presets, sequencing, and modulation.

## 1. Exciters

### Confirmed

#### Hit
Current/default exciter.

Purpose:
- short struck excitation
- velocity-sensitive force/brightness
- works across all modal objects

Current behavior hooks:
- excitation force
- excitation brightness
- velocity mapping

### Planned

#### Scrape
Planned in Gate 3.

Core model targets:
- pressure
- speed
- roughness
- stick-slip behavior

Roadmap sonic targets:
- bowed steel
- brake squeal
- tension rise

Internal flavor layer planned:

#### `ScrapeFlavor`
- BowedSteel
- BrakeSqueal
- TensionRise

Likely algorithm inputs we should define:
- pressure
- speed
- friction / roughness
- stick-slip threshold
- stick time
- slip burst amount
- brightness tilt
- instability amount
- sustain feedback / energy injection

### Conditional / open slot

#### Third exciter (optional)
Gate 6 says:
- confirm Hit + Scrape are enough
- if not, add `Mallet`

So the plan currently implies:
- minimum final exciter count: 2
- possible third exciter: Mallet

If Mallet exists, likely parameters:
- hardness
- strike width
- contact time
- transient brightness
- damping

## 2. Objects / Resonators

### Confirmed

#### Pipe
Character:
- tubular ring
- clearer pitch
- moderate sustain

#### Plate
Character:
- flatter metallic object
- more inharmonic spread

#### Tank
Character:
- lower
- boomier
- cavity-like metal
- longer sustain

### Planned

#### Chain
Planned in Gate 3.

Character:
- dense transients
- unstable pitch
- high inharmonicity
- short individual decays
- rougher than all current objects

Important constraint:
- must be a true modal profile
- not `noise + reverb`

### Implied object family design dimensions

For every object we’ll eventually want to define:
- mode count
- base frequency map
- inharmonicity profile
- modal gain rolloff
- modal decay curve
- transient density
- roughness bias
- stereo spread response
- body coupling response

## 3. Transformations

### Confirmed

#### Size
Current/planned behavior:
- larger size -> lower pitch / weighted frequency
- larger size -> longer decay
- larger size -> more low-band emphasis

#### Rust
Current/planned behavior:
- more rust -> darker
- more rust -> shorter
- more rust -> more damping / worn response

#### Damage
Current/planned behavior:
- more damage -> more detune / instability
- more damage -> more roughness
- more damage -> more rattling industrial character
- Gate 3 specifically wants this improved beyond simple detune

### Planned refinement

#### Damage character pass
Not just detune:
- rattle behavior
- industrial chatter
- possible envelope-tied noise/rattle bursts
- roughness increase without turning into generic noise

#### Velocity expressiveness pass
Velocity will eventually affect more than brightness:
- force
- brightness
- excitation decay shape
- damage character depth

## 4. Post / Space / Output Processing

### Confirmed

#### Drive
Current:
- saturation stage
- currently simple tanh

Planned:
- upgraded to multi-stage/asymmetric soft clip
- preserve impact and crest factor at high drive

#### Output
Current:
- final output gain

#### Hard limiter
Current:
- final-stage hard knee limiter at about `-0.3 dBFS`

### Planned

#### Stereo spread
Gate 3.

Parameter:
- `Width`

Behavior:
- mode-dependent L/R distribution
- 0 = mono
- 1 = full spread
- preserve mid/side balance

#### Body resonator
Gate 3.

Parameter:
- `Body`

Behavior:
- lightweight broad modal body
- low-mid reinforcement
- not reverb
- fixed small bank of broad resonances

#### Space modes
Gate 6.

Need at least 2 selectable body/space modes by final release.

Known/planned candidates:
- body resonator
- stereo spread
- optional lightweight room mode

Important constraint:
- no heavy convolution / no full reverb as core identity
- if room exists, keep it lightweight and in scope

## 5. Primary User Parameters

### Current MVP parameters

These are real now:
1. `Object`
2. `Size`
3. `Rust`
4. `Damage`
5. `Drive`
6. `Output`

### Planned future primary parameters

Based on roadmap:
7. `Exciter` (`Hit` / `Scrape` / maybe `Mallet`)
8. `Width`
9. `Body`

Potentially later, if space modes become explicit:
10. `Space Mode`
11. `Exciter Flavor` or scrape submode, if exposed

## 6. Internal / Algorithmic Control Sets

These may stay hidden or semi-hidden, but they are part of the design space.

### Scrape internal controls
- pressure
- speed
- roughness
- stick-slip amount
- scrape flavor
- scrape sustain behavior
- instability
- friction noise mix
- tension rise amount

### Object internal controls

For each object:
- mode frequencies
- decay distribution
- gain distribution
- inharmonicity
- transient density
- unstable partial behavior
- coupling/body interaction

### Damage internal controls
- detune spread
- random drift
- rattle burst amount
- roughness injection
- modal duplication / sideband-like roughness
- unstable decay behavior

### Drive internal controls
- asymmetry
- soft knee amount
- stage count
- transient preservation
- low-frequency tightening
- high-frequency crunch amount

## 7. GUI / Macro Layer

### Planned GUI sections

Gate 4 fixed layout:
1. `Exciter`
2. `Object`
3. `Damage`
4. `Space`

No oscillator/filter/amp framing allowed.

### Planned macros

These are high-level meta-controls:

#### Mass
Maps to:
- Object
- Size

#### Corrosion
Maps to:
- Rust
- body damping

#### Violence
Maps to:
- Drive
- excitation force

#### Damage
Maps to:
- Damage
- roughness

These are not separate DSP blocks; they are mapping layers over internal params.

## 8. Randomization / Mutation System

### Planned randomizer modes
1. Safe
2. Object
3. Damage
4. Full

### Planned mutate behavior
- small gaussian jitter around current state
- constrained to safe ranges

This matters because algorithm design should avoid dead zones and pathological combinations.

## 9. Preset Taxonomy

### Gate 2 preset families
- Bass
- Clang / Impact
- Boom / Low-body
- Short-hit
- Long-tail

### Gate 3 additional families
- Scrape
- Chain
- Drone
- Transition

### Gate 6 final preset families
- Bass
- Percussion
- Drone
- Transition
- Cinematic-impact

This helps define which algorithms need distinct corners.

## 10. Sequencer / Step-Lock Surface

These aren’t core sound generators, but they do affect how parameters must behave.

### Planned sequencer step fields
- enabled
- note
- velocity
- probability
- microtiming offset
- locks

### Planned per-step locks
- Object lock
- Exciter lock
- Rust lock
- Damage lock
- Drive lock

This means all those parameters eventually need:
- instant recall
- deterministic behavior
- good behavior under rapid switching

## 11. Modulation System

### Planned final modulation
Gate 6:
- user-configurable modulation matrix

Model:
- source -> destination
- depth
- polarity

Possible sources explicitly implied:
- LFO
- envelope

Possible destinations:
- any user parameter

This means every exposed parameter should eventually be modulation-safe.

## 12. Final Feature Inventory by DSP Domain

### Exciters
- Hit
- Scrape
- optional Mallet
- scrape flavors:
  - BowedSteel
  - BrakeSqueal
  - TensionRise

### Objects
- Pipe
- Plate
- Tank
- Chain

### Transforms
- Size
- Rust
- Damage
- velocity expressiveness layer

### Post / Space
- Drive
- Output
- Hard limiter
- Width
- Body
- at least 2 space/body modes by release

### Meta-control
- Mass
- Corrosion
- Violence
- Damage macro
- Randomizer
- Mutate
- Mod matrix

### Sequencing / performance
- 32-step sequencer
- probability
- microtiming
- host sync
- per-step locks
- kit mode

## 13. Suggested Design Order

To build the algorithms cleanly, define these in order:

1. Exciter roster
   - Hit
   - Scrape
   - decide whether Mallet is real or optional

2. Object roster
   - Pipe / Plate / Tank / Chain
   - one-paragraph sonic identity for each

3. Transformation philosophy
   - what Size, Rust, Damage should do perceptually across all objects

4. Post-processing philosophy
   - what Drive should feel like
   - what Body/Space should do and not do

5. Macro philosophy
   - what `Mass`, `Corrosion`, `Violence`, `Damage` should feel like musically
