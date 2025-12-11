# Changelog

All notable changes to this project will be documented in this file.

## [0.1.1-pre]

### Added
- **Configurable Constructor**: `SnowfallShader` can now be initialized with a full configuration object instead of just particle count.
- **Color Support**: Added `color` option (array `[r, g, b]`) to initialization config.
- **Texture Support**: Added `texture` option (HTMLImageElement) to initialization config.
- **Frontend Copy/Paste**: Added support for copying current configuration as a JSON-like object and pasting it to apply settings.
- **Texture Config Export**: Copying configuration changed.
- **Config Preloading**: `index.html` demo now preloads textures if specified in the initial config.

### Changed
- **Initialization**: Deprecated passing `particle_count` number to constructor.
- **WASM Init**: `init()` now accepts an object `{ module_or_path: "path" }`.
- **Rust Config**: `SnowConfig` struct now have color and is no longer `Copy`.
- **Dependencies**: Removed reliance on hardcoded color defaults in shader if a config color is provided.
