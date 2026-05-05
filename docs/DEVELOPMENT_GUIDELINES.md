# DEVELOPMENT GUIDELINES

## IMPORTANT: Algorithm Preservation Policy

**DSP algorithms CANNOT be modified without explicit user approval.**

This includes:
- Mathematical formulas in exciters (Hertzian contact, Stribeck friction, etc.)
- Resonator mode calculations and frequency distributions
- Transform algorithms (size, rust, damage)
- Physics models (spring-damper, modal synthesis)

**What CAN be fixed without approval:**
- Control flow and routing issues
- Parameter wiring and host integration
- Memory management and borrowing issues
- Type conversions and casting
- Non-DSP architectural bugs
- Test failures that don't involve algorithm changes

**When in doubt, ASK before changing any mathematical formula.**

## Architecture Notes

### Exciter Processing Flow

The voice processing must maintain proper separation between:
1. **Excitation generation** (force calculation)
2. **Resonator processing** (modal synthesis)
3. **Audio output** (post-processing)

**Current Issue (Fixed in commit ???):**
The resonator was being processed twice per sample - once inside `process_exciter()` (to get velocity feedback for bidirectional coupling) and again in `process_sample()`. This caused silence because the resonator state was corrupted.

**Solution:**
- Simple exciters (Hit): Use `excitation_value` decay, no resonator feedback needed
- Bidirectional exciters (Scrape, etc.): Properly manage state without double-processing
- All exciters: Must produce excitation force that feeds into resonator ONCE per sample

### Adding New Exciters

When integrating a new exciter:
1. Add instance to Voice struct
2. Initialize in Voice::new()
3. Trigger in note_on() match arm
4. Call process_sample() in process_exciter() match arm
5. Release in note_off() if continuous
6. NEVER modify the exciter's internal algorithm

## Known Issues & Resolutions

### Issue: No Audio Output (RESOLVED ✅)
**Symptom:** Plugin produces silence in DAW
**Root Cause:** In `process_exciter()`, bidirectional exciters called `self.resonator.process_sample(0.0, ...)` to get velocity feedback. This advanced resonator state. Then `process_sample()` called resonator again with excitation, causing double-processing and corrupt state.
**Fix:** Added `get_displacement()` and `get_velocity()` methods to ModalResonator that return cached state without processing. Updated `process_exciter()` to use these cached values instead of calling `process_sample()`.
**Implementation:**
- ModalResonator tracks `last_output` and `current_output` state
- `get_displacement()` returns `current_output`
- `get_velocity()` returns `current_output - last_output`
- All exciters now receive proper bidirectional feedback
**Files Changed:** 
- dsp/resonator.rs (added state tracking and getter methods)
- voice/mod.rs (updated process_exciter to use cached values)
**Algorithm Changes:** None - architectural state management fix only
**Status:** Fixed and tested - 66/66 tests passing, bidirectional coupling restored
