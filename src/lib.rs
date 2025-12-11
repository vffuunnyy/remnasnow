//! RemnaSnow - WebGL snowfall effect in WASM

use wasm_bindgen::prelude::*;
use web_sys::console;

pub mod config;
pub mod constants;
pub mod renderer;
pub mod shaders;

pub use config::{SnowConfig, WindState};
pub use constants::*;
pub use renderer::{RenderResult, SnowfallShader};

#[wasm_bindgen(start)]
pub fn main() {
    let mode = if RUNTIME_CONFIGURABLE {
        "configurable"
    } else {
        "release"
    };
    console::log_1(&format!("[RemnaSnow] WASM loaded ({mode})").into());
}

#[wasm_bindgen]
pub fn is_runtime_configurable() -> bool {
    RUNTIME_CONFIGURABLE
}

#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").into()
}
