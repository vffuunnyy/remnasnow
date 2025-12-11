//! Snowfall configuration

use wasm_bindgen::prelude::*;
use crate::constants::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct SnowConfig {
    pub gravity: f32,
    pub depth: f32,
    pub min_size: f32,
    pub min_alpha: f32,
    pub speed_x: f32,
    pub speed_y: f32,
    pub direction_x: f32,
    pub direction_y: f32,
    pub rotation_speed: f32,
}

impl Default for SnowConfig {
    fn default() -> Self {
        Self {
            gravity: DEFAULT_GRAVITY,
            depth: DEFAULT_DEPTH,
            min_size: DEFAULT_MIN_SIZE,
            min_alpha: DEFAULT_MIN_ALPHA,
            speed_x: DEFAULT_SPEED_X,
            speed_y: DEFAULT_SPEED_Y,
            direction_x: DEFAULT_DIRECTION_X,
            direction_y: DEFAULT_DIRECTION_Y,
            rotation_speed: DEFAULT_ROTATION_SPEED,
        }
    }
}

#[wasm_bindgen]
impl SnowConfig {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WindState {
    pub current: f32,
    pub force: f32,
    pub target: f32,
    pub min: f32,
    pub max: f32,
    pub easing: f32,
}

impl Default for WindState {
    fn default() -> Self {
        Self {
            current: 0.0,
            force: WIND_FORCE_INITIAL,
            target: WIND_TARGET_INITIAL,
            min: WIND_MIN,
            max: WIND_MAX,
            easing: WIND_EASING,
        }
    }
}