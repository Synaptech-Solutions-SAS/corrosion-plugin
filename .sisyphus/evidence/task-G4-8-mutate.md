# G4-8: Mutate Behavior and Safety Constraints

Date: 2026-05-04
Task: Implement mutate with Gaussian jitter and safety constraints

## Implementation

Part of `src/randomizer/mod.rs`.

## Mutate Behavior

**Definition**: Small Gaussian jitter on every active parameter around current values.

### API

```rust
pub fn mutate_value(current: f32, amount: f32, t: f32) -> f32;
```

Where:
- `current`: Current parameter value (0-1 normalized)
- `amount`: Mutation amount (0-1, typically 0.1-0.2 for subtle changes)
- `t`: Random value 0-1 (from UI-thread RNG)

### Algorithm
```rust
jitter = (t - 0.5) * 2.0 * amount
result = clamp(current + jitter, 0.0, 1.0)
```

This produces:
- Small variations around current state
- Symmetric distribution (equal probability of increase/decrease)
- Guaranteed bounded output (0-1 clamped)

## Safety Constraints

### SafetyConstraints Implementation

```rust
pub struct SafetyConstraints;

impl SafetyConstraints {
    pub fn is_safe_patch(params: &RandomizedParams) -> bool;
    pub fn clamp_to_safe_ranges(params: &mut RandomizedParams);
}
```

### Constraint Rules

1. **Non-silent**: Output >= 0.01
2. **No extreme combinations**: Not (Damage > 0.8 AND Drive > 0.8)
3. **Size bounds**: 0.1 <= Size <= 3.0
4. **Hard clamping**: All params clamped to valid ranges

### Constraint Application

When a random patch is generated:
1. Generate raw values from mode ranges
2. Apply `clamp_to_safe_ranges()` to ensure bounds
3. Verify with `is_safe_patch()` for additional rules
4. If unsafe, regenerate or adjust

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

## Design Decisions

### Why Deterministic t-values?

The randomizer uses externally-provided t-values (0-1) instead of calling
`rand::thread_rng()` directly. This allows:
- UI-thread controlled randomness (as required by spec)
- Reproducible testing (same t-values = same output)
- No audio-thread RNG usage

### Mutation Range

Default mutation amount of 0.1-0.2 (10-20% of full range) provides:
- Audible but subtle variation
- Keeps character of original patch
- Allows multiple mutations without drift

## References

- `docs/full-feature-surface.md` Section 8
- Roadmap G4-8 specification

## Status: COMPLETE ✓
