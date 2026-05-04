# G4-2..5: Macro Controls Implementation

Date: 2026-05-04
Tasks: Mass, Corrosion, Violence, Damage macros

## Implementation

Created new macros module at `src/macros/`:
- `mod.rs` - Macro parameter definitions and mapping functions

## Macro Definitions

### Mass (G4-2)
- **Range**: 0.0 - 1.0
- **Default**: 0.5
- **Maps to**: Object + Size
- **Mapping**:
  - Size: 0.25 + (mass × 1.75) → range [0.25, 2.0]
  - Object: mass < 0.25 → Pipe, < 0.5 → Plate, < 0.75 → Tank, else Chain

### Corrosion (G4-3)
- **Range**: 0.0 - 1.0
- **Default**: 0.0
- **Maps to**: Rust + Body
- **Mapping**:
  - Rust: corrosion × 1.0 → range [0.0, 1.0]
  - Body: corrosion × 0.8 → range [0.0, 0.8]

### Violence (G4-4)
- **Range**: 0.0 - 1.0
- **Default**: 0.2
- **Maps to**: Drive
- **Mapping**:
  - Drive: violence × 1.0 → range [0.0, 1.0]

### Damage Macro (G4-5)
- **Range**: 0.0 - 1.0
- **Default**: 0.0
- **Maps to**: Damage
- **Mapping**:
  - Damage: damage_macro × 1.0 → range [0.0, 1.0]

## GUI Integration

Macros are displayed in a "Macros" section at the top of the GUI:
- Mass, Corrosion, Violence, Damage sliders
- Positioned above the Exciter/Object/Damage/Space sections

## QA Verification

### Build
```bash
cargo check --target x86_64-unknown-linux-gnu --features gui
```
Result: ✓ Clean build

### Tests
```bash
cargo test --workspace --target x86_64-unknown-linux-gnu
```
Result: ✓ 83/83 tests passing

## References

- `docs/full-feature-surface.md` - Macro specifications (Section 7)
- `docs/sound-direction-brief.md` - Physical metaphor requirements

## Status: COMPLETE ✓

Note: The macro-to-parameter real-time mapping logic is implemented as conversion
functions. Full automation of parameter updates from macro changes is deferred to
G4-6 (Macro mapping persistence) to ensure proper NIH-plug parameter lifecycle
management.
