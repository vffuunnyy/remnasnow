//! Snowfall configuration

use crate::constants::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct SnowConfig {
    pub particle_count: u32,
    pub gravity: f32,
    pub depth: f32,
    pub min_size: f32,
    pub min_alpha: f32,
    pub speed_x: f32,
    pub speed_y: f32,
    pub direction_x: f32,
    pub direction_y: f32,
    pub rotation_speed: f32,
    
    #[wasm_bindgen(getter_with_clone)]
    pub color: Option<Vec<f32>>,
    
    #[wasm_bindgen(getter_with_clone)]
    pub texture: Option<HtmlImageElement>,
}

impl Default for SnowConfig {
    fn default() -> Self {
        Self {
            particle_count: DEFAULT_PARTICLE_COUNT,
            gravity: DEFAULT_GRAVITY,
            depth: DEFAULT_DEPTH,
            min_size: DEFAULT_MIN_SIZE,
            min_alpha: DEFAULT_MIN_ALPHA,
            speed_x: DEFAULT_SPEED_X,
            speed_y: DEFAULT_SPEED_Y,
            direction_x: DEFAULT_DIRECTION_X,
            direction_y: DEFAULT_DIRECTION_Y,
            rotation_speed: DEFAULT_ROTATION_SPEED,
            color: None,
            texture: None,
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

impl SnowConfig {
    pub fn from_js(value: JsValue) -> Self {
        let mut config = Self::default();

        if !value.is_object() {
            if let Some(count) = value.as_f64() {
                config.particle_count = count as u32;
            }
            return config;
        }

        macro_rules! extract {
            ($field:ident, $key:expr, $ty:ty) => {
                if let Ok(v) = js_sys::Reflect::get(&value, &$key.into()) {
                    if let Some(num) = v.as_f64() {
                        config.$field = num as $ty;
                    }
                }
            };
        }

        extract!(particle_count, "particleCount", u32);
        extract!(gravity, "gravity", f32);
        extract!(depth, "depth", f32);
        extract!(min_size, "minSize", f32);
        extract!(min_alpha, "minAlpha", f32);
        extract!(speed_x, "speedX", f32);
        extract!(speed_y, "speedY", f32);
        extract!(direction_x, "directionX", f32);
        extract!(direction_y, "directionY", f32);
        extract!(rotation_speed, "rotationSpeed", f32);
        
        if let Ok(color_val) = js_sys::Reflect::get(&value, &"color".into()) {
            if js_sys::Array::is_array(&color_val) {
                let arr: js_sys::Array = color_val.into();
                let vec: Vec<f32> = arr.iter()
                    .filter_map(|x| x.as_f64().map(|n| n as f32))
                    .collect();
                if !vec.is_empty() {
                    config.color = Some(vec);
                }
            }
        }

        if let Ok(tex_val) = js_sys::Reflect::get(&value, &"texture".into()) {
            if !tex_val.is_undefined() && !tex_val.is_null() {
                if let Ok(img) = tex_val.dyn_into::<HtmlImageElement>() {
                    config.texture = Some(img);
                }
            }
        }

        config
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
