# Macro Mapping Documentation

This document describes the mapping between macro controls and underlying parameters.

## Overview

Macros are high-level meta-controls that simultaneously adjust multiple underlying
parameters to achieve coherent sonic changes. They preserve the physical metaphor
of the instrument (industrial metal objects).

## Macro Definitions

### Mass Macro

**Physical metaphor**: Controls how heavy and large the object feels.

**Parameter**: `mass` (0.0 - 1.0, default: 0.5)

**Mapping**:
| Mass Range | Object | Size Value |
|------------|--------|------------|
| 0.0 - 0.25 | Pipe | 0.25 - 0.69 |
| 0.25 - 0.50 | Plate | 0.69 - 1.12 |
| 0.50 - 0.75 | Tank | 1.12 - 1.56 |
| 0.75 - 1.00 | Chain | 1.56 - 2.00 |

**Sonic result**: Low mass = small, tight, high-pitched. High mass = large, boomy, low-pitched.

---

### Corrosion Macro

**Physical metaphor**: Controls how worn and deteriorated the metal surface is.

**Parameter**: `corrosion` (0.0 - 1.0, default: 0.0)

**Mapping**:
- Rust: `corrosion × 1.0` (range: 0.0 - 1.0)
- Body: `corrosion × 0.8` (range: 0.0 - 0.8)

**Sonic result**: Low corrosion = bright, clean. High corrosion = dark, damped, resonant body.

---

### Violence Macro

**Physical metaphor**: Controls how aggressively the object is excited.

**Parameter**: `violence` (0.0 - 1.0, default: 0.2)

**Mapping**:
- Drive: `violence × 1.0` (range: 0.0 - 1.0)

**Sonic result**: Low violence = gentle, clean. High violence = saturated, aggressive, distorted.

---

### Damage Macro

**Physical metaphor**: Controls how structurally compromised the object is.

**Parameter**: `damage_macro` (0.0 - 1.0, default: 0.0)

**Mapping**:
- Damage: `damage_macro × 1.0` (range: 0.0 - 1.0)

**Sonic result**: Low damage = stable pitch. High damage = detuned, rough, rattling.

## Implementation Notes

### Conversion Functions

All mapping logic is in `src/macros/mod.rs`:

```rust
pub fn mass_to_size(mass: f32) -> f32;
pub fn mass_to_object(mass: f32) -> i32;
pub fn corrosion_to_rust(corrosion: f32) -> f32;
pub fn corrosion_to_body(corrosion: f32) -> f32;
pub fn violence_to_drive(violence: f32) -> f32;
pub fn damage_to_damage(damage: f32) -> f32;
```

### Parameter Persistence

When a preset is saved, both the macro values and the underlying parameter values
are stored. When loaded, the macro values are restored, and the underlying
parameters reflect the last state (which may have been set by the macro or
manually adjusted).

This design allows:
1. Quick sound shaping via macros
2. Fine-tuning individual parameters after macro adjustment
3. Presets to capture the final state regardless of how it was achieved

## Future Extensions

Potential future macro mappings (not implemented in Gate 4):
- Velocity sensitivity mapping
- Exciter character adjustment
- Stereo width correlation

## References

- `docs/full-feature-surface.md` Section 7: GUI/Macro Layer
- `docs/sound-direction-brief.md`: Primary tonal axes (Weight, Damage, Corrosion, Violence)
