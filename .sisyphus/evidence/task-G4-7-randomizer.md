# G4-7: Randomizer Modes

Date: 2026-05-04
Task: Implement Safe/Object/Damage/Full randomizer modes

## Implementation

Created new randomizer module at `src/randomizer/`:
- `mod.rs` - Randomizer modes and parameter generation

## Randomizer Modes

### Safe Mode
Small jitter on all parameters within safe ranges.
- Size: 0.5 - 1.5
- Rust: 0.0 - 0.5
- Damage: 0.0 - 0.5
- Drive: 0.0 - 0.5
- Width: 0.3 - 0.8
- Body: 0.0 - 0.5
- Output: 0.5 - 1.0

### Object Mode
Randomizes Object-related parameters only.
- Size: 0.25 - 2.0 (full range)
- Other params: fixed at neutral values

### Damage Mode
Randomizes Rust and Damage parameters.
- Rust: 0.0 - 1.0 (full range)
- Damage: 0.0 - 1.0 (full range)
- Other params: fixed at safe values

### Full Mode
Randomizes all parameters within their full ranges.
- All parameters: full min-max range
- Output constrained to 0.5 - 1.0 to prevent silence/clipping

## API

```rust
pub enum RandomizerMode {
    Safe,
    Object,
    Damage,
    Full,
}

pub fn ranges_for_mode(mode: RandomizerMode) -> ParamRanges;
pub fn randomize_params(mode: RandomizerMode, t_values: &[f32]) -> RandomizedParams;
```

## Design Constraints Met

✓ No `rand::thread_rng()` on audio thread - uses deterministic t-values
✓ Produces non-silent patches (Output >= 0.1)
✓ Produces non-clipping patches (constraints applied)
✓ Safe ranges prevent extreme/unusable combinations

## QA Verification

### Build
```bash
cargo check --target x86_64-unknown-linux-gnu --features gui
```
Result: ✓ Clean build

### Unit Tests
```bash
cargo test --lib --target x86_64-unknown-linux-gnu
```
Result: ✓ 53/53 tests passing

## References

- `docs/full-feature-surface.md` Section 8: Randomization/Mutation System
- `docs/sound-direction-brief.md`: Product metaphor constraints

## Status: COMPLETE ✓

Note: GUI trigger button and integration with preset system deferred to G4-9
(Preset browser workflow) to be implemented together.
