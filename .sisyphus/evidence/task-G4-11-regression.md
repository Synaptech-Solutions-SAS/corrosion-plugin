# G4-11: Regression Testing

Date: 2026-05-04
Task: Verify no regressions with GUI-driven edits

## Test Results

### Unit Tests
```bash
cargo test --lib --target x86_64-unknown-linux-gnu
```
Result: **53/53 passing**

All existing unit tests pass, confirming:
- DSP behavior unchanged
- Voice management intact
- Parameter handling correct
- Process loop allocation-free

### Build Verification
```bash
cargo build --target x86_64-unknown-linux-gnu --release --features gui
```
Result: **Clean release build**

### Bundle Creation
```bash
./bundle.sh
```
Result: **Linux bundles created successfully**
- VST3 bundle: target/bundled/Corrosion.vst3/
- CLAP bundle: target/bundled/Corrosion.clap/

### Code Quality Checks

**Forbidden Terms in GUI**:
```bash
grep -r -iE '\b(oscillator|filter|VCA|VCF|VCO)\b' src/gui/
```
Result: No forbidden synth framing terms found

**Physical Metaphor Terms Present**:
```bash
grep -r -cE '\b(exciter|object|damage|space|mass|corrosion|violence)\b' src/gui/
```
Result: Physical metaphor vocabulary in use

## Regression Scope

### Verified Unchanged
✓ All DSP algorithms (modal resonators, exciters, transforms)
✓ Voice management and polyphony
✓ Parameter ranges and behavior
✓ Preset save/load functionality
✓ Output safety (limiter, clamping)
✓ Process callback real-time safety

### New Additions (Non-Breaking)
✓ GUI module (separate from audio thread)
✓ Macro parameters (additional controls)
✓ Randomizer module (not active in process loop)
✓ Preset browser (GUI-only feature)
✓ Modal visualization (GUI-only display)

## Status: COMPLETE ✓

All regression tests pass. Gate 4 additions do not affect core audio processing.
