# Corrosion Sound Direction Brief

This file is a practical stand-in for direct user-supplied listening references. It defines the intended sound character in words, by behavior, so algorithm design can proceed immediately.

The goal is **not** "pretty modal synthesis". The goal is **industrial physical tension**: corroded metal, unstable resonance, violent impacts, friction, mechanical chatter, weight, and hostile energy.

## Global Identity

Corrosion should feel like:
- struck, scraped, flexed, dragged, or stressed metal
- heavy machinery, structural steel, tanks, plates, chains, brake assemblies, ducts, cables, and industrial debris
- more physical and material-specific than synth-like
- more dangerous and damaged than polished or bell-like
- aggressive when pushed, but still controllable and playable

Corrosion should **not** feel like:
- a glassy bell synth
- a clean modal demo plugin
- a soft ambient resonator by default
- generic noise through reverb
- a subtractive synth disguised as physical modeling

## Primary Tonal Axes

Every exciter/object/transform/post-process algorithm should be judged against these axes:

1. **Weight**
   - how massive the object feels
   - low-mid authority matters more than sub rumble

2. **Damage**
   - instability, rattle, split modes, chatter, looseness
   - not just detune; should feel mechanically compromised

3. **Corrosion**
   - darkness, damping, abrasion, worn surfaces, reduced shine
   - more friction and material loss than EQ dullness

4. **Violence**
   - attack severity, transient force, saturation, collision energy
   - should feel like impact stress, not just gain

5. **Tension**
   - feeling of stored force, friction buildup, strain, squeal, pressure
   - especially important for scrape-based exciters

## Exciter Targets

### 1. Hit Exciter

Role:
- the default impact driver
- covers strikes, drops, collisions, hammer hits, shell hits, slammed metal

Desired character:
- fast, forceful transient
- more body and grit than click
- velocity should change **force, brightness, and violence**, not just volume
- hard hits should feel like the object is being stressed closer to failure

Should support these sub-feels:
- **tight strike**: short, controlled, sharp
- **heavy slam**: broader transient, more low-mid push
- **damaged impact**: excites unstable modes and rattle behavior

Avoid:
- piano-like hammer softness
- generic sine-ping onset
- overly pure, bell-like attacks

### 2. Scrape Exciter

Role:
- continuous friction driver
- the "stress and tension" exciter

Desired character:
- contact friction, not broadband hiss
- instability from stick-slip behavior
- sustained energy injection into resonances
- can move between metallic bowing, brake squeal, dragging steel, cable tension

Core scrape feels:

#### Bowed Steel
- smoother sustained friction
- strong tonal lock with unstable high harmonics
- expressive pressure-dependent scream

#### Brake Squeal
- sharper, more hostile, narrower but piercing energy
- unstable mode locking and release
- ugly, high-friction, alarming character

#### Tension Rise
- audible increase in pressure/energy over time
- should feel like metal is being stressed toward snapping or slipping

Avoid:
- white noise wash
- fake "air" instead of friction
- static filtered noise pretending to scrape

### 3. Optional Mallet Exciter

If added later, it should cover:
- padded but still metallic impact
- less violent than Hit, more body-focused
- useful for drones, ceremonial hits, lower-aggression percussion

## Object Targets

### 1. Pipe

Identity:
- tubular, hollow, pitched, ringing
- the clearest object in the set

Desired behavior:
- identifiable center pitch
- supportive upper metallic ring
- moderate sustain
- can become eerie or ceremonial when large and clean

Avoid:
- sounding like a vibraphone or chime
- sounding too polished or tuned

### 2. Plate

Identity:
- flatter metal sheet/panel
- more clang and spread than Pipe

Desired behavior:
- more inharmonic splash
- sharper attack glare
- strong clang identity
- good for hits, impacts, industrial percussion

Avoid:
- turning into a cymbal model
- overly bright hi-fi shimmer

### 3. Tank

Identity:
- large cavity, low body, heavy shell response
- the most "weighty" base object

Desired behavior:
- stronger low-mid cavity feel
- slower energy bloom
- longer tail than Plate
- good for booms, impacts, giant hollow metal body

Avoid:
- muddy undefined bass blur
- purely subby, non-metallic low end

### 4. Chain

Identity:
- dense transient complexity
- unstable pitch perception
- linked metal elements, chatter, collisions

Desired behavior:
- rougher than all other objects
- less stable pitch than Pipe/Plate/Tank
- shorter, denser, more chaotic texture
- good for impacts, drags, debris, industrial percussion swarms

Avoid:
- replacing it with noise bursts
- sounding like a shaker or tambourine

## Transformation Targets

### Size

Perceptual goal:
- a true mass/scale transform

As Size increases:
- pitch lowers
- modal spacing feels bigger
- decay lengthens
- low-mid dominance increases
- transient sharpness softens slightly

As Size decreases:
- pitch rises
- transients tighten
- behavior becomes more rigid, smaller, more nervous

Avoid:
- a plain pitch shifter feel
- changing pitch without changing perceived mass

### Rust

Perceptual goal:
- material wear and oxidation

As Rust increases:
- brightness drops
- decay shortens
- friction/noisy abrasion may increase slightly
- the object sounds less efficient at ringing cleanly
- attack may feel drier and duller

Avoid:
- simple low-pass filtering as the whole effect
- losing the metallic identity completely

### Damage

Perceptual goal:
- compromised structure and mechanical instability

As Damage increases:
- roughness increases
- pitch stability decreases
- split/rattling/secondary motion increases
- attacks become less clean and more chaotic
- object may exhibit chatter, flutter, loose-part behavior

Damage should sound like:
- cracks
- loose welds
- bent metal
- unstable joints
- sympathetic debris vibration

Avoid:
- only static detune
- only noise layering

## Post Processing Targets

### Drive

Perceptual goal:
- impact stress, overload, mechanical aggression

As Drive increases:
- transients become denser and more forceful
- harmonics increase
- grit and asymmetry increase
- but the object should still feel physical

High Drive should feel like:
- metal being pushed into violent resonance
- contact overload
- harshness with retained body

Avoid:
- flattened, lifeless saturation
- turning every sound into generic distortion fuzz

### Body

Perceptual goal:
- reinforce the sense of enclosure or structural mass

As Body increases:
- low-mid support increases
- object feels more installed in a larger structure
- resonance gains a shell/frame/cabinet quality

Avoid:
- obvious reverb tail
- smeared attack

### Width

Perceptual goal:
- physical spread of energy, not fake stereo chorus

As Width increases:
- mode distribution should widen naturally
- object should feel spatially larger or more distributed

Avoid:
- modulation-chorus effect
- wide but phasey blur

### Limiter / Safety

Perceptual goal:
- never obvious as an effect
- should only prevent destructive overs

Avoid:
- pumping
- audibly shaving normal dynamics

## Macro Intent

### Mass
- should make the instrument feel larger, heavier, slower, more structural
- not just lower in pitch

### Corrosion
- should make it feel older, rougher, darker, less structurally sound
- not just muffled

### Violence
- should increase force, impact, and overload character
- not just loudness

### Damage
- should increase instability, chatter, loose-part behavior, and unsafe feeling
- not just randomization

## Preset Intent by Family

### Bass
- heavy hollow metal low-end
- useful as playable bass, not only cinematic FX

### Clang / Impact
- hard metallic strikes with pronounced attack identity

### Boom / Low-body
- large shell/body emphasis with strong low-mid bloom

### Short-hit
- compact industrial percussion
- punchy, dry, sharp, mechanical

### Long-tail
- lingering resonant structures
- eerie, structural, suspended metal energy

### Scrape
- friction-first sounds
- tension, stress, squeal, drag

### Chain
- chaotic linked-metal transients
- debris, chatter, rough cascades

### Drone
- sustained structural resonance
- ominous industrial sustain without going purely ambient

### Transition
- rises, stress sweeps, scraped tension, decaying structural events

### Cinematic-impact
- large-scale industrial impact moments with body, width, and violence

## Failure Modes to Avoid Everywhere

- too pretty
- too bell-like
- too polished
- too generic-synth
- too noisy without object identity
- too distorted without physical clarity
- too dark without material information
- too wide without structural meaning
- too random without repeatable character

## Short Algorithm Brief Per Domain

### Exciters
Design for contact physics and force delivery, not generic transient synthesis.

### Objects
Design for distinct material identities with unique modal logic, not cosmetic EQ differences.

### Transforms
Design for perceptual/material state changes, not abstract parameter motion.

### Post
Design for structural reinforcement and overload behavior, not effect-processor personality first.

## Final North Star

If a sound is:
- beautiful,
- polished,
- stable,
- harmonic,
- and pleasant,

then it is probably **not yet Corrosion enough**.

It should still be musical, but it should feel like metal under stress, age, force, and failure.
