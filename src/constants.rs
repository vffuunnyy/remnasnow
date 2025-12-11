//! Snowfall constants
//!
//! In debug mode all parameters can be changed through the API.
//! In release mode parameters are hardcoded and cannot be changed.

// Particles
pub const DEFAULT_PARTICLE_COUNT: u32 = 130_000;
pub const DEFAULT_GRAVITY: f32 = 6.0;
pub const DEFAULT_DEPTH: f32 = 110.0;
pub const DEFAULT_MIN_SIZE: f32 = 3.0;
pub const DEFAULT_MIN_ALPHA: f32 = 0.5;
pub const DEFAULT_SPEED_X: f32 = 0.2;
pub const DEFAULT_SPEED_Y: f32 = 0.6;
pub const DEFAULT_DIRECTION_X: f32 = 1.0;
pub const DEFAULT_DIRECTION_Y: f32 = 1.0;
pub const DEFAULT_ROTATION_SPEED: f32 = 1.7;

// Wind
pub const WIND_FORCE_INITIAL: f32 = 0.09;
pub const WIND_TARGET_INITIAL: f32 = 0.05;
pub const WIND_MIN: f32 = 0.05;
pub const WIND_MAX: f32 = 0.15;
pub const WIND_EASING: f32 = 0.003;

// Camera
pub const FOV_DEGREES: f32 = 45.0;
pub const NEAR_PLANE: f32 = 0.1;
pub const FAR_PLANE: f32 = 200.0;
pub const WORLD_HEIGHT: f32 = 110.0;

// Feature flag
pub const RUNTIME_CONFIGURABLE: bool = cfg!(feature = "configurable");
