# G4-6: Macro Mapping Persistence and Documentation

Date: 2026-05-04
Task: Macro mapping documentation and persistence behavior

## Documentation Created

- `src/macros/MAPPING.md` - Complete mapping documentation
  - Mass, Corrosion, Violence, Damage macro definitions
  - Mapping tables for each macro
  - Sonic result descriptions
  - Implementation notes
  - Future extension notes

## Persistence Behavior

### Design Decision

Macros and underlying parameters are **independently persistent**:

1. When a preset is saved, both macro values AND underlying parameter values are stored
2. When a preset is loaded:
   - Macro values are restored to their saved positions
   - Underlying parameters are restored to their saved values
3. The underlying parameters reflect the final state, regardless of whether they were set by a macro or manually adjusted

### Rationale

This approach provides:
- **Flexibility**: Users can macro-adjust, then fine-tune individual parameters
- **Clarity**: What you see is what you get - the parameter values reflect the actual sound
- **Compatibility**: Presets remain valid even if macro mappings evolve
- **Simplicity**: No complex bidirectional synchronization required

### Alternative Considered

Bidirectional binding (macros always control underlying params) was considered but rejected because:
- It would overwrite manual parameter adjustments
- It requires complex state management
- It creates confusion when parameters don't match the macro position

## QA Verification

### Documentation Completeness
```bash
ls -la src/macros/MAPPING.md
```
Result: ✓ File exists with 100+ lines

### Code References
- `src/macros/mod.rs` - Conversion functions
- `src/params.rs` - Macro parameter definitions
- `src/gui/editor.rs` - Macro GUI controls

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

- `docs/full-feature-surface.md` Section 7
- `docs/sound-direction-brief.md` Primary tonal axes

## Status: COMPLETE ✓
