# Corrosion Sisyphus Plan

## Purpose
This Sisyphus plan is the execution tracker for Corrosion. Each roadmap milestone is treated as a hard gate, and a gate only closes when its implementation checklist, pass criteria, evidence summary, and review steps are complete.

## Plan Rules
- Top-level gate checkboxes are approval boundaries.
- Nested checkboxes are implementation and verification tasks.
- Do not mark a gate complete until its pass criteria have been satisfied.
- If a gate review fails, add follow-up tasks under that gate instead of advancing.

## Gate Sequence
- Gate 0 — Research Prototype
- Gate 1 — Minimal Plugin
- Gate 2 — MVP / Version 0.1.0
- Gate 3 — Industrial Character / Version 0.2.0 Core
- Gate 4 — Product UX / Version 0.3.0-Style Update
- Gate 5 — Sequenced Instrument
- Gate 6 — Version 1.0 Release

---

## TODOs

### Gate 0 — Research Prototype
- [ ] Gate 0 complete
  - [x] Build an offline DSP renderer in Rust or Python
  - [x] Implement the second-order modal resonator prototype
  - [x] Create deterministic excitation input for repeatable testing
  - [x] Create initial pipe modal profile
  - [x] Create initial plate modal profile
  - [x] Create initial tank modal profile
  - [x] Tune object families so they are clearly distinct by ear
  - [x] Test size scaling against pitch and resonance behavior
  - [x] Prototype rust-based decay and brightness loss
  - [x] Prototype damage-driven detuning and roughness
  - [x] Estimate safe mode counts for real-time use
  - [x] Identify likely CPU hot spots
  - [x] Render comparison WAVs for pipe, plate, and tank
  - [x] Render low/high rust examples
  - [x] Render low/high damage examples
  - [ ] Record chosen initial parameter ranges
  - [ ] Write Gate 0 evidence summary
  - [ ] Review Gate 0 against pass criteria

#### Gate 0 Pass Criteria
  - [ ] Pipe, plate, and tank sound clearly distinct
  - [ ] Excitation produces audible decaying output
  - [ ] Rust audibly darkens and shortens the sound
  - [ ] Damage audibly destabilizes or roughens the sound
  - [ ] Output is not silent by default when excited
  - [ ] No NaN, infinity, runaway feedback, or uncontrolled blowup occurs

### Gate 1 — Minimal Plugin
- [ ] Gate 1 complete
  - [ ] Set up NIH-plug project scaffold
  - [ ] Define plugin shell structure
  - [ ] Define parameter ownership structure
  - [ ] Create parameter module scaffold
  - [ ] Implement note-on event handling
  - [ ] Implement MIDI note to base-frequency conversion
  - [ ] Preserve natural decay behavior on note-off for hit mode
  - [ ] Build first voice structure
  - [ ] Implement hit exciter in plugin path
  - [ ] Implement pipe object in plugin path
  - [ ] Route hit excitation into pipe resonator output
  - [ ] Add safe output clamp
  - [ ] Build VST3 target
  - [ ] Build CLAP target
  - [ ] Smoke test plugin in REAPER
  - [ ] Smoke test plugin in Bitwig
  - [ ] Capture Gate 1 host-loading evidence
  - [ ] Write Gate 1 evidence summary
  - [ ] Review Gate 1 against pass criteria

#### Gate 1 Pass Criteria
  - [ ] Plugin builds as VST3
  - [ ] Plugin builds as CLAP
  - [ ] Plugin loads in host without immediate crashes
  - [ ] MIDI note-on triggers audible sound
  - [ ] Note-off allows natural decay
  - [ ] Output remains bounded and safe
  - [ ] Code structure supports later parameter and voice expansion

### Gate 2 — MVP / Version 0.1.0
- [ ] Gate 2 complete
  - [ ] Expand to 8-voice voice manager
  - [ ] Implement inactive voice reuse
  - [ ] Implement quietest/oldest voice stealing fallback
  - [ ] Implement tail-energy tracking
  - [ ] Implement voice deactivation threshold logic
  - [ ] Add plate object profile to real-time engine
  - [ ] Add tank object profile to real-time engine
  - [ ] Verify pipe, plate, and tank differentiation
  - [ ] Expose Object parameter
  - [ ] Expose Size parameter
  - [ ] Expose Rust parameter
  - [ ] Expose Damage parameter
  - [ ] Expose Drive parameter
  - [ ] Expose Output parameter
  - [ ] Add stable range mapping for MVP parameters
  - [ ] Ensure automation compatibility for MVP parameters
  - [ ] Improve velocity response beyond pure loudness
  - [ ] Ensure no allocation occurs in the sample render path
  - [ ] Guard against denormals
  - [ ] Guard against NaN and infinity
  - [ ] Keep rendering stable under fast repeated MIDI sequences
  - [ ] Build generic or minimal editor
  - [ ] Design preset list aligned with MVP preset goals
  - [ ] Create at least 20 factory presets
  - [ ] Cover bass-style presets
  - [ ] Cover clang/impact presets
  - [ ] Cover boom/low-body presets
  - [ ] Cover short-hit presets
  - [ ] Cover long-tail presets
  - [ ] Add or confirm safety limiter / hard safety clip
  - [ ] Add unit test for MIDI note to frequency conversion
  - [ ] Add unit test for resonator coefficient generation
  - [ ] Add unit test for resonator stability
  - [ ] Add unit test for saturation output range
  - [ ] Add unit test for object profile selection
  - [ ] Add unit test for tail deactivation logic
  - [ ] Add DSP test for non-silent excitation
  - [ ] Add DSP test for output decay
  - [ ] Add DSP test for rust spectral impact
  - [ ] Add DSP test for damage waveform impact
  - [ ] Run pluginval validation
  - [ ] Manual test MIDI notes in REAPER
  - [ ] Manual test fast sequences in REAPER
  - [ ] Manual test long decays in REAPER
  - [ ] Manual test automation in REAPER
  - [ ] Manual test preset changes in REAPER
  - [ ] Manual test buffer size changes
  - [ ] Manual test sample rate changes
  - [ ] Capture Gate 2 evidence summary
  - [ ] Review Gate 2 against pass criteria

#### Gate 2 Pass Criteria
  - [ ] Plugin builds as VST3 and CLAP
  - [ ] Plugin loads in REAPER
  - [ ] MIDI note-on triggers audible sound consistently
  - [ ] Pipe, plate, and tank sound clearly different
  - [ ] Size changes pitch and resonance behavior
  - [ ] Rust audibly darkens, shortens, and roughens sound
  - [ ] Damage audibly destabilizes or saturates sound
  - [ ] Drive audibly increases nonlinear aggression
  - [ ] Output gain works safely
  - [ ] No crash occurs under rapid MIDI note triggering
  - [ ] No NaN or infinite samples occur
  - [ ] 8 voices can play simultaneously
  - [ ] At least 20 factory presets are included
  - [ ] Plugin does not allocate memory in the sample render path
  - [ ] Plugin passes basic plugin validation

### Gate 3 — Industrial Character / Version 0.2.0 Core
- [ ] Gate 3 complete
  - [ ] Implement scrape exciter core behavior
  - [ ] Tune scrape for bowed steel / brake squeal / tension-rise use cases
  - [ ] Implement chain object behavior
  - [ ] Verify chain behaves as a distinct physical-feeling source
  - [ ] Implement stereo modal spread
  - [ ] Tune stereo spread for controllable width
  - [ ] Implement lightweight body resonator
  - [ ] Tune body resonance to add context without over-smearing
  - [ ] Improve roughness and rattle character
  - [ ] Improve saturation character
  - [ ] Improve velocity mapping expressiveness
  - [ ] Expand preset library toward 40+
  - [ ] Add scrape-focused presets
  - [ ] Add chain-focused presets
  - [ ] Add drone-focused presets
  - [ ] Add transition-focused presets
  - [ ] Compare product identity against “should sound / should avoid sounding” guidance
  - [ ] Run automation stress test for stereo/body parameters
  - [ ] Run regression tests against Gate 2 stability expectations
  - [ ] Validate expanded engine in host
  - [ ] Capture Gate 3 evidence summary
  - [ ] Review Gate 3 against pass criteria

#### Gate 3 Pass Criteria
  - [ ] Scrape exciter is implemented
  - [ ] Chain object is implemented
  - [ ] Stereo modal spread is implemented
  - [ ] Body resonance is implemented
  - [ ] Velocity mapping is improved
  - [ ] Factory presets reach the 40+ target direction
  - [ ] Product identity is more distinctly industrial

### Gate 4 — Product UX / Version 0.3.0-Style Update
- [ ] Gate 4 complete
  - [ ] Replace generic editor with custom GUI
  - [ ] Organize GUI around Exciter → Object → Damage → Space
  - [ ] Ensure controls use physical metaphors instead of subtractive-synth framing
  - [ ] Implement Mass macro
  - [ ] Implement Corrosion macro
  - [ ] Implement Violence macro
  - [ ] Implement Damage macro
  - [ ] Map macros to meaningful internal parameter groups
  - [ ] Implement safe random mode
  - [ ] Implement object random mode
  - [ ] Implement damage random mode
  - [ ] Implement full random mode
  - [ ] Implement mutate behavior if included in this release scope
  - [ ] Enforce randomization safety constraints
  - [ ] Implement preset browser workflow
  - [ ] Improve preset browsing speed and clarity vs MVP flow
  - [ ] Add visual object/resonator feedback
  - [ ] Add tail/output activity feedback if chosen
  - [ ] Avoid generic oscillator/filter/amp visual framing
  - [ ] Perform GUI walkthrough review
  - [ ] Perform randomizer safety review
  - [ ] Perform macro mapping review
  - [ ] Perform preset browsing workflow review
  - [ ] Run regression tests for automation, preset changes, and output safety
  - [ ] Capture Gate 4 evidence summary
  - [ ] Review Gate 4 against pass criteria

#### Gate 4 Pass Criteria
  - [ ] Basic custom GUI is implemented
  - [ ] Randomizer is implemented
  - [ ] Macro controls are implemented
  - [ ] Preset browsing is implemented in usable form
  - [ ] Interface supports the product metaphor and quick sound-shaping goals

### Gate 5 — Sequenced Instrument
- [ ] Gate 5 complete
  - [ ] Implement sequencer step data structure
  - [ ] Implement runtime playback model
  - [ ] Support enabled step state
  - [ ] Support per-step note
  - [ ] Support per-step velocity
  - [ ] Support per-step probability
  - [ ] Support per-step microtiming
  - [ ] Read BPM from host
  - [ ] Read sample position from host
  - [ ] Read transport playing state from host
  - [ ] Read loop state where available
  - [ ] Keep sequencing stable under play/stop transitions
  - [ ] Keep sequencing stable under loop transitions
  - [ ] Implement per-step object locks
  - [ ] Implement per-step exciter locks
  - [ ] Implement per-step rust locks
  - [ ] Implement per-step damage locks
  - [ ] Implement per-step drive locks
  - [ ] Ensure locks behave correctly with preset recall
  - [ ] Ensure locks behave correctly with automation
  - [ ] Implement kit mode or equivalent kit-oriented workflow
  - [ ] Validate industrial rhythm workflow usefulness
  - [ ] Validate timing across transport scenarios
  - [ ] Demonstrate per-step lock behavior with example patterns
  - [ ] Verify probability and microtiming behavior musically
  - [ ] Run host sync tests in supported hosts
  - [ ] Run loop/restart/tempo-change timing checks
  - [ ] Run regression tests for note behavior, preset changes, and automation interactions
  - [ ] Capture Gate 5 evidence summary
  - [ ] Review Gate 5 against pass criteria

#### Gate 5 Pass Criteria
  - [ ] Sequencer is implemented and stable
  - [ ] Per-step locks are implemented and behave correctly
  - [ ] Host sync works under tempo and transport changes
  - [ ] Probability and microtiming are musically useful
  - [ ] Kit-oriented workflow is viable for industrial rhythm design

### Gate 6 — Version 1.0 Release
- [ ] Gate 6 complete
  - [ ] Finalize sequencer and per-step locks for release quality
  - [ ] Finalize preset browser reliability
  - [ ] Ensure multiple exciter types are available
  - [ ] Ensure multiple body/space types are available
  - [ ] Implement user-configurable modulation mappings
  - [ ] Expand preset library to 100+
  - [ ] Cover bass patch family
  - [ ] Cover percussion patch family
  - [ ] Cover drone patch family
  - [ ] Cover transition patch family
  - [ ] Cover cinematic impact patch family
  - [ ] Prepare demo sounds
  - [ ] Write user manual
  - [ ] Document object/exciter concept
  - [ ] Document controls
  - [ ] Document MIDI behavior
  - [ ] Document preset system
  - [ ] Document automation behavior
  - [ ] Document installation
  - [ ] Document troubleshooting
  - [ ] Write developer documentation
  - [ ] Document architecture
  - [ ] Document DSP modules
  - [ ] Document parameter mappings
  - [ ] Document real-time safety assumptions
  - [ ] Document build instructions
  - [ ] Document testing instructions
  - [ ] Document release process
  - [ ] Write sound design guide
  - [ ] Add recipe for rusted pipe bass
  - [ ] Add recipe for bent plate snare
  - [ ] Add recipe for oil tank boom
  - [ ] Add recipe for chain hi-hat
  - [ ] Add recipe for bowed metal drone
  - [ ] Add recipe for industrial loop
  - [ ] Build VST3 release bundle
  - [ ] Build CLAP release bundle
  - [ ] Prepare README
  - [ ] Prepare changelog
  - [ ] Prepare installation instructions
  - [ ] Prepare validation reports
  - [ ] Prepare installer or clearly packaged installation workflow
  - [ ] Run pluginval for release candidate
  - [ ] Run manual DAW tests in REAPER
  - [ ] Run manual DAW tests in Bitwig
  - [ ] Run manual DAW tests in Ardour
  - [ ] Run buffer-size regression checks
  - [ ] Run sample-rate regression checks
  - [ ] Run final performance and safety validation
  - [ ] Capture Gate 6 evidence summary
  - [ ] Review Gate 6 against pass criteria

#### Gate 6 Pass Criteria
  - [ ] Sequencer implemented
  - [ ] Per-step locks implemented
  - [ ] Preset browser implemented
  - [ ] Multiple exciter types implemented
  - [ ] Multiple body/space types implemented
  - [ ] User-configurable modulation mappings implemented
  - [ ] 100+ presets included
  - [ ] Documentation included
  - [ ] Installer or clear installation package provided

## Final Verification Wave
- [ ] Cross-gate quality checks completed
  - [ ] No heap allocation in the audio thread
  - [ ] No file I/O in sample rendering
  - [ ] No logging in sample rendering
  - [ ] No mutex locking in sample rendering
  - [ ] No JSON parsing in sample rendering
  - [ ] No blocking work in sample rendering
  - [ ] No dynamic vector resizing in sample rendering
  - [ ] No uncontrolled feedback blowups
  - [ ] No NaN in output samples
  - [ ] No infinity in output samples
  - [ ] Output remains safely bounded
  - [ ] Plugin recovers safely from extreme automation
  - [ ] Rapid note triggering does not crash the plugin
  - [ ] DSP modules remain testable outside the plugin shell
- [ ] Final release approval complete
  - [ ] All gates passed in order
  - [ ] All gate evidence summaries captured
  - [ ] Final release artifacts prepared
  - [ ] Final validation complete
  - [ ] Product identity remains aligned with industrial physical modeling under stress
