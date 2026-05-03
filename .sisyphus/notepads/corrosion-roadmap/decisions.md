## 2026-05-03 - G2-9 Preset Format Decisions

- Use `Object` as the serde-backed enum with `PascalCase` names so `.corrosion-preset` stores the human-readable object name directly.
- Keep `Preset` as the canonical external file shape and rebuild `CorrosionParams` from it on load instead of mutating NIH-plug params in place.
- Mirror the default parameter ranges in preset reconstruction so load/save stays aligned with the UI and host-facing values.
