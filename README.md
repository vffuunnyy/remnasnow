# RemnaSnow ❄️

Snowfall effect on WebGL with calculations in WebAssembly.

![WASM](https://img.shields.io/badge/WebAssembly-654FF0?style=flat&logo=webassembly&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![WebGL](https://img.shields.io/badge/WebGL-990000?style=flat&logo=webgl&logoColor=white)

## Quickstart

### Requirements

- [Rust](https://rustup.rs/) (1.70+)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```bash
cargo install wasm-pack
```

### Build

```bash
# Release build (hardcoded parameters)
make release-opt

# Dev build (parameters configurable in runtime)
make dev

# Run local server
make serve
```

## API

### Usage

```javascript
import init, { SnowfallShader } from './pkg/remnasnow.js';

await init();

// Create (canvas-id, particle_count - optional)
const snowfall = new SnowfallShader('canvas-id', 130000);

function loop(time) {
    const result = snowfall.render(time);
    // result.fps, result.wind, result.particle_count, result.time
    requestAnimationFrame(loop);
}
requestAnimationFrame(loop);

// Handle resize
window.addEventListener('resize', () => snowfall.resize());
```

### Getters

```javascript
snowfall.get_fps();            // Current FPS
snowfall.get_time();           // Time since start
snowfall.get_wind();           // Current wind
snowfall.get_particle_count(); // Particle count
snowfall.get_config();         // Current config
snowfall.is_configurable();    // Can parameters be changed
```
