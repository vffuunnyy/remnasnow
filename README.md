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

await init({ module_or_path: './pkg/remnasnow_bg.wasm' });

// Prepare texture (optional)
const img = new Image();
img.src = "snowflake.svg";
await img.decode();

// Configuration object
const config = {
    particleCount: 150000,
    gravity: 8.0,
    depth: 80.0,
    minSize: 2.5,
    minAlpha: 0.5,
    speedX: 0.2,
    speedY: 0.3,
    directionX: 1.0,
    directionY: 1.0,
    rotationSpeed: 1.0,
    color: [0.9, 0.4, 0.7], // optional RGB [0..1]
    texture: img,           // optional HTMLImageElement
};

const snowfall = new SnowfallShader('canvas-id', config);

function loop(time) {
    const result = snowfall.render(time);
    // result.fps, result.wind, result.particle_count, result.time
    requestAnimationFrame(loop);
}
requestAnimationFrame(loop);

// Handle resize
window.addEventListener('resize', () => snowfall.resize());
```

### Methods

**Getters**
```javascript
snowfall.get_fps();            // Current FPS
snowfall.get_time();           // Time since start
snowfall.get_wind();           // Current wind
snowfall.get_particle_count(); // Particle count
snowfall.get_config();         // Current config object
snowfall.is_configurable();    // Check if setters are available
```

**Setters** (Only available in `configurable` feature)
```javascript
snowfall.set_particle_count(n); // Reinitializes buffers with new count
snowfall.set_depth(n);          // Reinitializes buffers
snowfall.set_min_size(n);       // Reinitializes buffers
snowfall.set_min_alpha(n);      // Reinitializes buffers
snowfall.set_speed_x(n);        // Reinitializes buffers
snowfall.set_speed_y(n);        // Reinitializes buffers
snowfall.set_direction_x(n);    // Reinitializes buffers
snowfall.set_direction_y(n);
snowfall.set_rotation_speed(n);
snowfall.set_gravity(n);

snowfall.set_color(r, g, b);    // 0.0 - 1.0
snowfall.clear_color();         // Revert to original texture colors
snowfall.set_texture(img);      // HTMLImageElement
snowfall.clear_texture();       // Revert to default
```
