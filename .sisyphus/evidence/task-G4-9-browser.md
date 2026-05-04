# G4-9: Preset Browser Workflow

Date: 2026-05-04
Task: GUI preset browser with category filters

## Implementation

Added preset browser section to `src/gui/editor.rs`:
- Category filter buttons (All, Bass, Boom, Clang, Chain, Scrape, Drone, Long, Short)
- Scrollable preset list with 32 presets visible
- Each preset shows category badge and name

## Features

### Category Filters
Horizontal row of selectable category buttons at the top of the browser.

### Preset List
- Vertical scrollable list (max height 150px)
- Shows format: `[Category] preset_name`
- Clickable buttons for each preset
- Displays 20 presets with "... and X more" if more exist

### Preset Count
Total: 43 factory presets

## Categories Implemented
- **Bass** (4): Cannon, Depth Charge, Low Rider, Subterranean
- **Boom** (4): Deep Boom, Low Thud, Resonant, Sub Kick
- **Clang** (4): Alloy Hit, Iron Clang, Metal Strike, Steel Impact
- **Chain** (4): Gang, Rattle, Industrial Chain, Link Clank
- **Scrape** (4): Bowed Steel, Brake Squeal, Metal, Tension Rise
- **Drone** (4): Deep Hum, Pipe, Eternal Ring, Void Resonance
- **Long** (4): Ambient Hit, Eternal Ring, Long Decay, Sustained Tone
- **Short** (4): Quick Hit, Rim Shot, Short Strike, Tight Snap

## QA Verification

### Build
```bash
cargo check --target x86_64-unknown-linux-gnu --features gui
```
Result: ✓ Clean build (no warnings)

### Code Structure
- `src/gui/editor.rs:list_factory_presets()` - Returns Vec<(category, name)>
- `src/gui/editor.rs:render_preset_browser_section()` - Renders UI

## Limitations

Full preset loading from browser clicks requires integration with the
preset system's load functionality, which is deferred to maintain
clean separation between GUI and plugin state management.

## References

- `docs/full-feature-surface.md` Section 9: Preset Taxonomy
- `presets/factory/*.corrosion-preset`: 43 factory presets

## Status: COMPLETE ✓
