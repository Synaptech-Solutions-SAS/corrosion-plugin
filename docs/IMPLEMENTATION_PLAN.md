# Corrosion Implementation Plan

## Purpose
This document turns the product spec into an execution plan where each roadmap milestone is a delivery gate. Work only advances when the current gate meets its implementation scope, verification requirements, and exit evidence, so the project stays aligned with the core identity: physically modeled industrial objects under stress.

> **Note**: As of Gate 3 (v0.2.0), this document describes both implemented features and future roadmap targets. Modules like `src/gui/`, `src/sequencer/`, `src/macros/`, and `src/randomizer/` are planned for Gates 4–6 and do not yet exist in the codebase. Refer to `docs/EXECUTION_TRACKER.md` for current gate status.

## Gate Sequence
| Gate | Roadmap Milestone | Release Mapping | Primary Outcome |
|---|---|---|---|
| Gate 0 | Research Prototype | Pre-release | Prove the sound engine concept offline |
| Gate 1 | Minimal Plugin | Pre-release | First playable VST3/CLAP instrument |
| Gate 2 | MVP Synth | 0.1.0 | First complete product concept |
| Gate 3 | Industrial Character | 0.2.0 core | Distinctive industrial sound expansion |
| Gate 4 | Product UX | 0.3.0-style update | Usable, inspiring product workflow |
| Gate 5 | Sequenced Instrument | Pre-1.0 feature complete | Industrial pattern instrument workflow |
| Gate 6 | Version 1.0 Release | 1.0.0 | Public-ready release package |

## Planning Rules
- A gate is failed unless all listed pass criteria and evidence are satisfied.
- Sound quality comes before interface polish.
- Stability and real-time safety are mandatory at every gate.
- Scope must follow the spec; no speculative features are added to “help.”
- Future expansion ideas remain deferred unless explicitly named in a gate.
- A gate review should end with a go/no-go decision, not a vague “mostly done.”

## Product Boundaries

### What the Plan Covers
- The main Corrosion instrument as a VST3/CLAP software instrument.
- The path from DSP prototype through public-ready 1.0 release.
- Sound engine, plugin integration, presets, UI, sequencing, documentation, validation, and packaging.

### What the Plan Does Not Cover
- Corrosion FX as a separate product.
- Corrosion Lab as a standalone workstation.
- Expansion preset packs as a separate business line.
- Neural synthesis, sample library browser, modular environment, or DAW-like features.

## Program-Level Priorities
1. Sound must be compelling before interface complexity grows.
2. The plugin must remain safe under real-time constraints.
3. The first complete release must stay focused and comprehensible.
4. Velocity and automation must shape physical behavior, not only output level.
5. The code architecture must allow later objects, exciters, and sequencing without rewrites.

## Cross-Gate Workstreams

### 1. DSP Core
- Exciter design
- Object model design
- Modal resonator implementation
- Damage/corrosion processing
- Body/space processing
- Output limiting and safety

### 2. Plugin Integration
- NIH-plug shell
- VST3/CLAP targets
- MIDI behavior
- Parameter exposure and automation
- Voice management and render lifecycle

### 3. Product Content
- Factory presets
- Patch families aligned to the sound identity
- Demo material and validation renders

### 4. Interface and UX
- Generic editor first
- Custom GUI later
- Macro controls
- Preset browsing
- Randomization

### 5. Testing and Validation
- Unit tests
- DSP tests
- Plugin validation
- Manual DAW tests
- Release verification

### 6. Documentation and Packaging
- User manual
- Developer documentation
- Sound design guide
- README, changelog, installation instructions
- Bundled release artifacts

## Gate Review Standard
Each gate review must answer all of the following:
- Is the gate scope complete?
- Does the implementation meet the sound and stability expectations for this stage?
- Is the evidence package sufficient to justify advancing?
- Are remaining issues only acceptable carry-forward items, or are they true blockers?

If any answer is “no” or “unclear,” the gate remains open.

---

## Gate 0 — Research Prototype

### Objective
Validate the core exciter-to-resonator approach outside the plugin environment so the team knows the product can achieve the intended industrial sound identity before plugin engineering begins.

### Release Mapping
- Pre-release research gate

### Depends On
- Product spec only

### In Scope
- Offline DSP experimentation
- Resonator proof of concept
- Pipe, plate, and tank object exploration
- Early parameter-range decisions
- Sound evaluation renders

### Out of Scope
- Plugin shell work
- Host integration
- Full UI
- Sequencing
- Preset browser

### Required Deliverables
- Modal resonator proof of concept
- Pipe, plate, and tank modal profiles
- WAV renders for evaluation
- Chosen initial parameter ranges
- Preliminary CPU and mode-count assumptions

### Implementation Workstreams

#### DSP Prototyping
- Build an offline renderer in Rust or Python
- Implement the second-order resonator structure from the spec
- Create deterministic excitation input for repeatable comparison
- Establish a baseline decay model suitable for real-time use later

#### Object Modeling
- Create initial curated mode sets for pipe, plate, and tank
- Tune each object so the families are clearly separable by ear
- Test how size scaling affects pitch and resonance behavior

#### Damage and Corrosion Exploration
- Prototype rust-based decay and brightness loss
- Prototype damage-driven detuning and roughness behavior
- Confirm both transformations remain musically useful rather than purely academic

#### Sound Evaluation
- Render comparative examples for each object
- Render low/high rust and low/high damage examples
- Compare object families against target patch families from the spec

#### Performance and Safety Framing
- Estimate safe mode counts for early real-time implementation
- Identify likely CPU hot spots before plugin integration
- Reject approaches that would require expensive per-sample recalculation

### Exit Evidence
- Offline render set covering pipe, plate, tank, size variation, rust variation, and damage variation
- Written parameter notes for chosen working ranges
- Short summary of what object profiles and transformations are worth carrying into plugin implementation

### Pass Criteria
- Pipe, plate, and tank sound clearly distinct in offline renders
- Excitation produces audible, decaying output without instability
- Rust audibly darkens and shortens the sound
- Damage audibly destabilizes or roughens the sound
- Output is not silent by default when excited
- No NaN, infinity, runaway feedback, or uncontrolled resonator blowup occurs in the prototype

### Verification Focus
- Offline deterministic renders
- Spectral and waveform inspection
- Comparative listening across object families
- Stress evaluation at extreme parameter settings

### Main Risks / Blockers
- Weak or generic sound identity
- Overly realistic but musically uninteresting behavior
- Excessive CPU from modal density
- Parameter ranges that sound good offline but will be unsafe in real time

---

## Gate 1 — Minimal Plugin

### Objective
Convert the validated core into the first real plugin instrument: playable from MIDI, audible in-host, and exportable as VST3/CLAP.

### Release Mapping
- Pre-release integration gate

### Depends On
- Gate 0 passed

### In Scope
- NIH-plug integration
- Single voice instrument behavior
- Hit exciter
- Pipe object
- MIDI note handling
- Safe audio output

### Out of Scope
- Polyphony beyond one practical voice path
- Plate and tank objects
- Preset library
- Custom UI
- Sequencer and randomizer

### Required Deliverables
- NIH-plug project scaffold
- Parameter module scaffold
- Single-voice audio render path
- MIDI note handling
- Hit exciter implementation
- Pipe object model in plugin path
- VST3 and CLAP build targets
- Safe output clamping

### Implementation Workstreams

#### Plugin Shell
- Set up plugin entry points and format exports
- Define processor structure and parameter ownership
- Prepare a layout that supports later DSP modularization

#### MIDI and Note Flow
- Implement note-on event handling
- Convert MIDI note to base frequency
- Preserve natural decay behavior on note-off for hit mode

#### Voice Rendering
- Build the first voice structure with exciter state, resonator state, and tail behavior
- Route hit excitation into pipe resonator output
- Ensure the render path stays simple and deterministic

#### Safety and Output
- Add hard output clamp or equivalent safety stage
- Confirm no dangerous failure mode at extreme input conditions

#### Build and Host Validation
- Produce first VST3/CLAP binaries
- Load the plugin in recommended hosts for smoke testing

### Exit Evidence
- Successful VST3/CLAP builds
- Host screenshots or notes confirming the plugin loads
- Audible test notes rendered from the plugin
- Short integration notes on remaining architectural gaps before MVP

### Pass Criteria
- Plugin builds as VST3 and CLAP instrument targets
- Plugin loads in host without immediate crashes
- MIDI note-on triggers audible sound
- Note-off allows natural decay rather than abrupt muting
- Output remains bounded and free from obvious failure states
- The code structure supports later parameter and voice expansion

### Verification Focus
- Host smoke testing in REAPER and Bitwig
- Basic plugin validation
- Manual note-trigger confirmation
- Simple rapid retrigger sanity check

### Main Risks / Blockers
- NIH-plug integration complexity
- Broken MIDI-to-audio path
- Plugin builds succeeding while host behavior remains unstable
- Early architectural shortcuts that would block polyphony or later objects

---

## Gate 2 — MVP / Version 0.1.0

### Objective
Deliver the first complete version that proves the product concept as a usable instrument rather than a prototype.

### Release Mapping
- 0.1.0

### Depends On
- Gate 1 passed

### In Scope
- 8-voice polyphony
- Pipe, plate, and tank object models
- MVP parameter set
- Velocity-sensitive excitation
- Stable real-time rendering
- Generic or minimal editor
- 20 factory presets
- Core validation workflow

### Out of Scope
- Scrape exciter
- Chain object
- Body resonance as a full feature
- Custom GUI
- Randomizer
- Sequencer

### Required Deliverables
- 8-voice voice manager
- Voice stealing and tail detection
- Pipe, plate, and tank objects
- Parameters: Object, Size, Rust, Damage, Drive, Output
- Stable rendering path
- Generic or minimal editor
- At least 20 factory presets
- Safe output limiter or hard safety clip
- Basic plugin validation results

### Implementation Workstreams

#### Voice Management
- Expand from one voice to 8 active voices
- Implement inactive-voice reuse, then quietest or oldest voice stealing
- Add tail-energy tracking and voice deactivation threshold logic

#### Object Expansion
- Implement plate and tank profiles in the real-time engine
- Make object selection automatable and reliable
- Ensure all three objects differ clearly in pitch behavior, mode balance, and decay feel

#### Core Parameter System
- Expose MVP parameter IDs and defaults
- Add stable range mapping for size, rust, damage, drive, and output
- Ensure automation is compatible with host expectations

#### Excitation and Expression
- Improve velocity response so it shapes force and brightness, not only amplitude
- Confirm different note velocities produce musically useful physical changes

#### Safety and Rendering Stability
- Ensure no allocation occurs in the sample render path
- Guard against denormals, NaN, infinity, and uncontrolled level spikes
- Keep rendering stable under fast repeated MIDI sequences

#### Preset Content
- Design at least 20 presets aligned with the MVP preset list from the spec
- Cover bass, clang, boom, knock, short hit, and long-tail categories
- Use presets to test whether the engine already communicates the intended identity

### Exit Evidence
- 8-voice stress notes
- Preset list and patch coverage summary
- Validation notes from REAPER and pluginval
- Manual test notes for note input, automation, and rapid triggering

### Pass Criteria
- Plugin builds as VST3 and CLAP
- Plugin loads in REAPER
- MIDI note-on triggers audible sound consistently
- Pipe, plate, and tank sound clearly different
- Size changes pitch and resonance behavior
- Rust audibly darkens, shortens, and roughens sound
- Damage audibly destabilizes or saturates sound
- Drive audibly increases nonlinear aggression
- Output gain works safely
- No crash occurs under rapid MIDI note triggering
- No NaN or infinite samples occur
- 8 voices can play simultaneously
- At least 20 factory presets are included
- Plugin does not allocate memory in the sample render path
- Plugin passes basic plugin validation

### Verification Focus
- Unit tests for note-to-frequency conversion, resonator coefficients, resonator stability, saturation output range, object selection, and tail deactivation
- DSP tests for non-silent excitation, decay behavior, rust spectral impact, and damage waveform impact
- pluginval verification
- Manual DAW tests for MIDI notes, fast sequences, long decays, automation, preset changes, buffer size changes, and sample rate changes

### Main Risks / Blockers
- CPU spikes with eight active voices
- Presets hiding weak underlying engine quality
- Damage and drive becoming harsh without being useful
- Generic editor being enough for testing but insufficient for reliable preset work if parameter layout is poor

---

## Gate 3 — Industrial Character / Version 0.2.0 Core

### Objective
Move from “functional MVP” to a more unmistakable Corrosion identity by adding friction, chain behavior, stereo spread, and body context.

### Release Mapping
- 0.2.0 core feature set

### Depends On
- Gate 2 passed

### In Scope
- Scrape exciter
- Chain object
- Stereo modal spread
- Body resonance
- Stronger roughness and saturation character
- Improved velocity mapping
- Preset expansion toward 40+

### Out of Scope
- Full sequencer workflow
- Deep preset browser UX polish beyond what Gate 4 requires
- Separate effect mode
- Standalone application

### Required Deliverables
- Scrape exciter implementation
- Chain object implementation
- Stereo modal spread
- Lightweight body resonator
- Improved saturation / rattle behavior
- Improved velocity response
- Expanded preset library meeting the 0.2 target direction

### Implementation Workstreams

#### Exciter Expansion
- Implement scrape behavior with pressure, speed, roughness, and stick-slip framing where appropriate
- Tune scrape for bowed steel, brake squeal, tension-rise, and metallic drone style behavior

#### Object Expansion
- Implement chain as a transient-dense, low-stable-pitch industrial source
- Ensure chain is not just “noise plus reverb,” but a distinct physical-feeling behavior

#### Stereo and Body Context
- Spread modal content across stereo in a musically controlled way
- Add a lightweight body resonator before considering any heavier future space model
- Tune body behavior to provide context without smearing all presets into the same sound

#### Damage Character Improvement
- Improve roughness, rattle, and nonlinear behavior so the instrument sounds more industrial and less generic
- Keep the saturation path forceful without collapsing dynamics

#### Preset and Identity Pass
- Expand preset library beyond the MVP baseline toward 40+
- Cover scrape, chain, drone, and transition categories introduced by this gate
- Re-evaluate the product identity against the “should sound / should avoid sounding” guidance from the spec

### Exit Evidence
- Comparative examples: hit vs scrape, pipe vs chain, dry vs body-enhanced, mono vs stereo spread
- Expanded preset inventory with category coverage
- Regression notes proving Gate 2 behavior remains stable

### Pass Criteria
- Scrape exciter is implemented
- Chain object is implemented
- Stereo modal spread is implemented
- Body resonance is implemented
- Velocity mapping is improved
- Factory presets expand toward and satisfy the 40+ target
- The product sounds more distinctly industrial rather than just “more featured”

### Verification Focus
- Comparative listening across new and old object/exciter families
- Automation stress testing for stereo and body parameters
- Regression testing against MVP safety and CPU expectations
- Host validation with the expanded engine enabled

### Main Risks / Blockers
- Added features weakening the core voice instead of sharpening it
- Phase, level, or CPU problems from stereo/body work
- Scrape and chain sounding derivative or gimmicky

---

## Gate 4 — Product UX / Version 0.3.0-Style Update

### Objective
Make Corrosion feel like a coherent instrument for musicians and sound designers, not just a strong engine behind a utilitarian shell.

### Release Mapping
- 0.3.0-style UX and workflow gate

### Depends On
- Gate 3 passed

### In Scope
- Custom GUI
- Visual object/resonator feedback
- Preset browser
- Randomizer
- Macro controls
- Physical-model-first layout

### Out of Scope
- Final public release packaging
- Full 1.0 sequencer completion
- Separate FX product

### Required Deliverables
- Custom GUI
- Visual object model or resonator display
- Preset browser
- Randomizer with constrained modes
- Macro controls
- Improved workflow for sound discovery and editing

### Implementation Workstreams

#### Interface Architecture
- Replace the generic editor with a custom interface organized around Exciter, Object, Damage, and Space
- Ensure controls use physical metaphors rather than traditional subtractive-synth framing

#### Macro System
- Implement macro controls such as Mass, Corrosion, Violence, and Damage
- Map macros to meaningful internal parameter groupings rather than arbitrary convenience knobs

#### Randomization
- Implement safe random, object random, damage random, full random, and mutate-style behavior if the plan chooses to expose all modes now
- Enforce constraints that avoid dangerous output, silence, DC problems, or unstable patches

#### Preset Browsing and Discovery
- Add a preset browser or equivalent workflow for rapid sound navigation
- Ensure browsing, recall, and category access are faster than the MVP preset flow

#### Visual Feedback
- Add feedback elements that reinforce the object-based model: object identity, resonant state, damage feel, or tail/output activity
- Avoid visuals that imply a generic oscillator/filter/amp architecture

### Exit Evidence
- GUI walkthrough notes or screenshots
- Randomizer safety notes and example outputs
- Macro mapping summary
- Preset browsing workflow demonstration

### Pass Criteria
- Basic custom GUI is implemented
- Randomizer is implemented
- Macro controls are implemented
- Preset browsing is implemented in usable form
- The interface supports the product metaphor and quick sound-shaping goals from the spec

### Verification Focus
- Hands-on usability testing in host
- Visual QA for layout clarity and interaction flow
- Regression testing for automation, preset changes, and output safety under GUI-driven edits

### Main Risks / Blockers
- UI scope creep delaying the product
- Randomization destabilizing patches or identity
- Macros feeling disconnected from the actual sound behavior
- The interface becoming technically dense instead of intuitive

---

## Gate 5 — Sequenced Instrument

### Objective
Add the industrial pattern-generation workflow that turns Corrosion into a stronger composition and groove-design tool.

### Release Mapping
- Pre-1.0 feature-complete sequencing gate

### Depends On
- Gate 4 passed

### In Scope
- 16/32-step sequencer
- Per-step note and velocity control
- Probability and microtiming
- Per-step locks
- Host sync
- Kit-oriented workflow

### Out of Scope
- Separate standalone environment
- Expansion product ecosystem
- Non-spec sequencing concepts

### Required Deliverables
- 16/32-step sequencer
- Per-step locks for object, exciter, rust, damage, and drive
- Host tempo and transport sync
- Probability support
- Microtiming support
- Kit mode or equivalent industrial rhythm workflow

### Implementation Workstreams

#### Sequencer Core
- Implement step data structure and runtime playback model
- Support enabled state, note, velocity, probability, and microtiming
- Ensure deterministic timing under normal host playback

#### Host Synchronization
- Read BPM, sample position, playing state, and loop state as available
- Keep sequencing stable under play/stop/loop transitions

#### Per-Step Locks
- Support locks for sound-defining properties exactly where the spec calls for them
- Ensure locked steps still behave correctly with preset recall and automation

#### Industrial Workflow Design
- Make the sequencer useful for industrial techno rhythm design rather than generic sequencing only
- Ensure kit-like workflows can map different object/preset behaviors into rhythmic programming

### Exit Evidence
- Timing validation notes across transport scenarios
- Demonstration patterns showing per-step locks and probability behavior
- Host sync verification summary

### Pass Criteria
- Sequencer is implemented and stable
- Per-step locks are implemented and behave correctly
- Host sync works under tempo and transport changes
- Probability and microtiming are musically useful
- Kit-oriented workflow is viable for industrial rhythm design

### Verification Focus
- Manual DAW sync tests in supported hosts
- Loop/restart/tempo-change timing checks
- Regression testing for note behavior, preset changes, and automation interactions

### Main Risks / Blockers
- Host sync edge cases across DAWs
- State complexity causing unstable sequencing behavior
- Timing drift or restart bugs that make the sequencer untrustworthy

---

## Gate 6 — Version 1.0 Release

### Objective
Package Corrosion as a public-ready product with complete features, content, documentation, validation, and installation clarity.

### Release Mapping
- 1.0.0

### Depends On
- Gate 5 passed

### In Scope
- Release bundles
- Full 1.0 acceptance coverage
- Documentation set
- Expanded preset library
- Validation reports
- Installation guidance

### Out of Scope
- Separate Corrosion FX release
- Corrosion Lab
- Post-1.0 ecosystem expansion

### Required Deliverables
- VST3 bundle
- CLAP bundle
- User manual
- Developer documentation
- Sound design guide
- 100+ preset library
- Demo sounds
- Validation reports
- README
- Changelog
- Installation instructions
- Installer or clearly packaged installation workflow

### Implementation Workstreams

#### Feature Completion
- Ensure sequencer and per-step locks are final-quality
- Ensure preset browser is complete and reliable
- Ensure multiple exciter types are available
- Ensure multiple body/space types are available
- Ensure user-configurable modulation mappings are implemented

#### Content Completion
- Expand preset library to 100+
- Cover the major patch families from the spec across bass, percussion, drones, transitions, and cinematic impacts
- Prepare demo material that proves the instrument range

#### Documentation Completion
- Write user-facing documentation covering concept, controls, MIDI behavior, presets, automation, installation, and troubleshooting
- Write developer documentation covering architecture, DSP modules, parameter mapping, real-time safety, build flow, testing flow, and release flow
- Write sound design guide recipes aligned with the spec examples

#### Release Engineering
- Build distributable VST3/CLAP bundles for target platforms
- Confirm release packaging is understandable for end users
- Prepare validation reports and release notes

### Exit Evidence
- Final release artifact list
- Validation report package
- Documentation package
- Preset inventory summary
- Release candidate verification notes

### Pass Criteria
- Sequencer implemented
- Per-step locks implemented
- Preset browser implemented
- Multiple exciter types implemented
- Multiple body/space types implemented
- User-configurable modulation mappings implemented
- 100+ presets included
- Documentation included
- Installer or clear installation package provided

### Verification Focus
- pluginval and release validation reports
- Manual DAW tests in REAPER, Bitwig, and Ardour
- Buffer-size and sample-rate regression checks
- Packaging review for installability and clarity
- Final performance and safety validation

### Main Risks / Blockers
- Packaging and distribution issues late in the release cycle
- Documentation lagging behind actual behavior
- Expanded content surfacing instability that earlier gates did not expose
- Final feature additions threatening the product’s clarity or identity

---

## Cross-Gate Quality Requirements
- No heap allocation in the audio thread
- No file I/O, logging, mutex locking, JSON parsing, blocking work, or dynamic vector resizing in sample rendering
- No uncontrolled feedback blowups
- No NaN or infinity in output samples
- Output must remain safely bounded
- The plugin must recover safely from extreme automation
- Rapid note triggering must not crash the plugin
- DSP modules should remain testable outside the plugin shell

## Cross-Gate Test Matrix

### Unit-Level Coverage
- MIDI note to frequency conversion
- Resonator coefficient generation
- Resonator stability
- Saturation output range
- Object profile selection
- Tail deactivation logic

### DSP-Level Coverage
- Non-silent output when excited
- Output decay after excitation
- Rust changes spectral energy
- Damage changes waveform shape
- No NaN or infinity under deterministic test buffers

### Plugin-Level Coverage
- Plugin loads
- Parameters automate
- Process callback is stable
- No crashes during random automation
- No crashes during rapid note events

### Manual Host Coverage
- REAPER
- Bitwig
- Ardour
- MIDI note entry
- Fast sequences
- Long decays
- Automation
- Preset changes
- Buffer size changes
- Sample rate changes

## Gate Artifacts Checklist
Each gate should close with an explicit artifact package proportionate to its scope.

### Minimum Gate Review Package
- Deliverable summary
- Known issues list
- Verification summary
- Audio or host evidence where applicable
- Decision: pass, fail, or carry-forward with named constraints

### Release-Oriented Gates Should Add
- Preset inventory status
- Documentation status
- Packaging status
- Compatibility notes

## Deferred Beyond This Plan
The following ideas exist in the spec but are not required for the core gate sequence unless explicitly pulled forward in a future revision:
- Corrosion FX as a separate product
- Corrosion Kits as separate expansion packs
- Corrosion Lab as a standalone workstation

## Definition of Complete Version
The complete version is the Gate 6 outcome: a public-ready Corrosion release that ships as VST3 and CLAP, includes the expanded sound engine and sequencing workflow defined by the spec, provides strong preset and documentation coverage, passes validation and host testing, preserves real-time safety, and clearly expresses the product identity of industrial physical modeling under stress.
