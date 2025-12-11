//! Snowfall rendering module

use js_sys::Math;
use wasm_bindgen::prelude::*;
use web_sys::{
    console, HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL,
    WebGlShader, WebGlUniformLocation,
};

use crate::config::{SnowConfig, WindState};
use crate::constants::*;
use crate::shaders::{FRAGMENT_SHADER_SOURCE, VERTEX_SHADER_SOURCE};

struct Uniforms {
    time: Option<WebGlUniformLocation>,
    projection: Option<WebGlUniformLocation>,
    world_size: Option<WebGlUniformLocation>,
    gravity: Option<WebGlUniformLocation>,
    wind: Option<WebGlUniformLocation>,
    resolution: Option<WebGlUniformLocation>,
    rotation_speed: Option<WebGlUniformLocation>,
    point_scale: Option<WebGlUniformLocation>,
}

impl Uniforms {
    fn load(gl: &GL, program: &WebGlProgram) -> Self {
        let get = |name| gl.get_uniform_location(program, name);
        Self {
            time: get("u_time"),
            projection: get("u_projection"),
            world_size: get("u_worldSize"),
            gravity: get("u_gravity"),
            wind: get("u_wind"),
            resolution: get("u_resolution"),
            rotation_speed: get("u_rotationSpeed"),
            point_scale: get("u_pointScale"),
        }
    }
}

#[wasm_bindgen]
pub struct RenderResult {
    pub fps: u32,
    pub time: f32,
    pub wind: f32,
    pub particle_count: u32,
}

// These buffer fields are used indirectly by WebGL (bound and used by the GPU).
// Suppress the dead_code warning so the compiler doesn't warn about them being unused in Rust.
#[allow(dead_code)]
struct Buffers {
    position: WebGlBuffer,
    color: WebGlBuffer,
    rotation: WebGlBuffer,
    size: WebGlBuffer,
    speed: WebGlBuffer,
}

#[wasm_bindgen]
pub struct SnowfallShader {
    gl: GL,
    canvas: HtmlCanvasElement,
    program: WebGlProgram,
    uniforms: Uniforms,
    buffers: Option<Buffers>,

    base_particle_count: u32,
    actual_particle_count: i32,

    time: f32,
    last_time: f32,

    wind: WindState,
    config: SnowConfig,

    frame_count: u32,
    fps_last_time: f32,
    current_fps: u32,

    world_width: f32,
    world_height: f32,
    world_depth: f32,
}

#[wasm_bindgen]
impl SnowfallShader {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str, initial_particle_count: Option<u32>) -> Result<Self, JsValue> {
        console::log_1(&"[RemnaSnow] Initializing WASM module...".into());

        let particle_count = initial_particle_count.unwrap_or(DEFAULT_PARTICLE_COUNT);
        let document = web_sys::window()
            .and_then(|w| w.document())
            .ok_or("Failed to get document")?;

        let canvas: HtmlCanvasElement = document
            .get_element_by_id(canvas_id)
            .ok_or("Canvas not found")?
            .dyn_into()?;

        let gl: GL = canvas
            .get_context("webgl")?
            .ok_or("WebGL is not supported")?
            .dyn_into()?;

        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        gl.clear_color(0.0, 0.0, 0.0, 0.0);

        let program = Self::create_program(&gl)?;
        gl.use_program(Some(&program));

        let uniforms = Uniforms::load(&gl, &program);
        let config = SnowConfig::default();

        let mut shader = Self {
            gl,
            canvas,
            program,
            uniforms,
            buffers: None,
            base_particle_count: particle_count,
            actual_particle_count: 0,
            time: 0.0,
            last_time: 0.0,
            wind: WindState::default(),
            config,
            frame_count: 0,
            fps_last_time: 0.0,
            current_fps: 0,
            world_width: 0.0,
            world_height: WORLD_HEIGHT,
            world_depth: config.depth,
        };

        shader.resize()?;
        shader.setup_buffers()?;

        console::log_1(
            &format!(
                "[RemnaSnow] Initialized with {} particles",
                shader.actual_particle_count
            )
            .into(),
        );
        Ok(shader)
    }

    fn create_program(gl: &GL) -> Result<WebGlProgram, JsValue> {
        let vs = Self::compile_shader(gl, GL::VERTEX_SHADER, VERTEX_SHADER_SOURCE)?;
        let fs = Self::compile_shader(gl, GL::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE)?;

        let program = gl.create_program().ok_or("Failed to create program")?;
        gl.attach_shader(&program, &vs);
        gl.attach_shader(&program, &fs);
        gl.link_program(&program);

        if !gl
            .get_program_parameter(&program, GL::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            let info = gl.get_program_info_log(&program).unwrap_or_default();
            return Err(format!("Link error: {info}").into());
        }
        Ok(program)
    }

    fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, JsValue> {
        let shader = gl
            .create_shader(shader_type)
            .ok_or("Failed to create shader")?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if !gl
            .get_shader_parameter(&shader, GL::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            let info = gl.get_shader_info_log(&shader).unwrap_or_default();
            gl.delete_shader(Some(&shader));
            return Err(format!("Shader compilation error: {info}").into());
        }
        Ok(shader)
    }

    fn create_buffer(&self, name: &str, data: &[f32], size: i32) -> Result<WebGlBuffer, JsValue> {
        let buffer = self.gl.create_buffer().ok_or("Failed to create buffer")?;
        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let array = js_sys::Float32Array::view(data);
            self.gl
                .buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
        }

        let location = self.gl.get_attrib_location(&self.program, name);
        if location >= 0 {
            let loc = location as u32;
            self.gl.enable_vertex_attrib_array(loc);
            self.gl
                .vertex_attrib_pointer_with_i32(loc, size, GL::FLOAT, false, 0, 0);
        }
        Ok(buffer)
    }

    fn setup_buffers(&mut self) -> Result<(), JsValue> {
        let (w, h) = (self.canvas.width() as f32, self.canvas.height() as f32);
        let aspect = if h > 0.0 { w / h } else { 1.0 };

        let (width, height, depth) = (aspect * WORLD_HEIGHT, WORLD_HEIGHT, self.config.depth);
        self.world_width = width;
        self.world_height = height;
        self.world_depth = depth;

        let count = (aspect * self.base_particle_count as f32) as usize;
        let (mut positions, mut colors, mut sizes, mut rotations, mut speeds) = (
            Vec::with_capacity(count * 3),
            Vec::with_capacity(count * 4),
            Vec::with_capacity(count),
            Vec::with_capacity(count * 3),
            Vec::with_capacity(count * 3),
        );

        let rand = || Math::random() as f32;
        let pi2 = std::f32::consts::TAU;

        for _ in 0..count {
            positions.extend_from_slice(&[
                -width + rand() * width * 2.0,
                -height + rand() * height * 2.0,
                -depth + rand() * depth * 2.0,
            ]);

            speeds.extend_from_slice(&[
                (self.config.speed_x + rand() * 0.4) * self.config.direction_x,
                self.config.speed_y + rand() * 0.5,
                rand() * 2.0,
            ]);

            rotations.extend_from_slice(&[
                rand() * pi2,
                rand() * 5.0 * self.config.rotation_speed,
                rand() * 3.0,
            ]);

            let alpha = self.config.min_alpha + rand() * (1.0 - self.config.min_alpha);
            colors.extend_from_slice(&[1.0, 1.0, 1.0, alpha]);

            sizes.push(self.config.min_size + rand() * 4.5);
        }

        self.buffers = Some(Buffers {
            position: self.create_buffer("a_position", &positions, 3)?,
            color: self.create_buffer("a_color", &colors, 4)?,
            rotation: self.create_buffer("a_rotation", &rotations, 3)?,
            size: self.create_buffer("a_size", &sizes, 1)?,
            speed: self.create_buffer("a_speed", &speeds, 3)?,
        });

        self.set_uniform_3f(&self.uniforms.world_size, width, height, depth);
        self.set_uniform_1f(
            &self.uniforms.gravity,
            self.config.gravity * self.config.direction_y,
        );
        self.set_uniform_1f(&self.uniforms.rotation_speed, self.config.rotation_speed);

        self.actual_particle_count = count as i32;
        Ok(())
    }

    #[inline]
    fn set_uniform_1f(&self, loc: &Option<WebGlUniformLocation>, v: f32) {
        if let Some(l) = loc {
            self.gl.uniform1f(Some(l), v);
        }
    }

    #[inline]
    fn set_uniform_2f(&self, loc: &Option<WebGlUniformLocation>, x: f32, y: f32) {
        if let Some(l) = loc {
            self.gl.uniform2f(Some(l), x, y);
        }
    }

    #[inline]
    fn set_uniform_3f(&self, loc: &Option<WebGlUniformLocation>, x: f32, y: f32, z: f32) {
        if let Some(l) = loc {
            self.gl.uniform3f(Some(l), x, y, z);
        }
    }

    pub fn resize(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("Failed to get window")?;
        let dpi = window.device_pixel_ratio();

        let (width, height) = (
            (window.inner_width()?.as_f64().unwrap_or(800.0) * dpi) as u32,
            (window.inner_height()?.as_f64().unwrap_or(600.0) * dpi) as u32,
        );

        self.canvas.set_width(width);
        self.canvas.set_height(height);
        self.gl.viewport(0, 0, width as i32, height as i32);

        let aspect = width as f32 / height as f32;
        let fov = FOV_DEGREES.to_radians();
        let f = 1.0 / (fov / 2.0).tan();

        let projection = [
            f / aspect,
            0.0,
            0.0,
            0.0,
            0.0,
            f,
            0.0,
            0.0,
            0.0,
            0.0,
            (FAR_PLANE + NEAR_PLANE) / (NEAR_PLANE - FAR_PLANE),
            -1.0,
            0.0,
            0.0,
            (2.0 * FAR_PLANE * NEAR_PLANE) / (NEAR_PLANE - FAR_PLANE),
            0.0,
        ];

        if let Some(l) = &self.uniforms.projection {
            self.gl
                .uniform_matrix4fv_with_f32_array(Some(l), false, &projection);
        }
        self.set_uniform_2f(&self.uniforms.resolution, width as f32, height as f32);
        self.set_uniform_1f(&self.uniforms.point_scale, height as f32 * 0.015);
        Ok(())
    }

    pub fn render(&mut self, current_time: f32) -> RenderResult {
        let delta = (current_time - self.last_time) * 0.001;
        self.last_time = current_time;

        self.frame_count += 1;
        if current_time - self.fps_last_time >= 1000.0 {
            self.current_fps = self.frame_count;
            self.frame_count = 0;
            self.fps_last_time = current_time;
        }

        self.time += delta;
        self.update_wind(delta);

        self.set_uniform_1f(&self.uniforms.time, self.time);
        self.set_uniform_1f(&self.uniforms.wind, self.wind.current);

        self.gl.clear(GL::COLOR_BUFFER_BIT);
        self.gl
            .draw_arrays(GL::POINTS, 0, self.actual_particle_count);

        RenderResult {
            fps: self.current_fps,
            time: self.time,
            wind: self.wind.current,
            particle_count: self.actual_particle_count as u32,
        }
    }

    fn update_wind(&mut self, delta: f32) {
        self.wind.force += (self.wind.target - self.wind.force) * self.wind.easing;
        self.wind.current += self.wind.force * delta * 0.5;

        if Math::random() > 0.99 {
            let range = self.wind.max - self.wind.min;
            let sign = if Math::random() > 0.5 { 1.0 } else { -1.0 };
            self.wind.target = (self.wind.min + Math::random() as f32 * range) * sign;
        }
    }

    #[cfg(feature = "configurable")]
    pub fn set_particle_count(&mut self, count: u32) -> Result<(), JsValue> {
        self.base_particle_count = count;
        self.setup_buffers()
    }

    #[cfg(feature = "configurable")]
    pub fn set_gravity(&mut self, value: f32) {
        self.config.gravity = value;
        self.set_uniform_1f(&self.uniforms.gravity, value * self.config.direction_y);
    }

    #[cfg(feature = "configurable")]
    pub fn set_depth(&mut self, value: f32) -> Result<(), JsValue> {
        self.config.depth = value;
        self.setup_buffers()
    }

    #[cfg(feature = "configurable")]
    pub fn set_min_size(&mut self, value: f32) -> Result<(), JsValue> {
        self.config.min_size = value;
        self.setup_buffers()
    }

    #[cfg(feature = "configurable")]
    pub fn set_min_alpha(&mut self, value: f32) -> Result<(), JsValue> {
        self.config.min_alpha = value;
        self.setup_buffers()
    }

    #[cfg(feature = "configurable")]
    pub fn set_speed_x(&mut self, value: f32) -> Result<(), JsValue> {
        self.config.speed_x = value;
        self.setup_buffers()
    }

    #[cfg(feature = "configurable")]
    pub fn set_speed_y(&mut self, value: f32) -> Result<(), JsValue> {
        self.config.speed_y = value;
        self.setup_buffers()
    }

    #[cfg(feature = "configurable")]
    pub fn set_direction_x(&mut self, value: f32) -> Result<(), JsValue> {
        self.config.direction_x = value;
        self.setup_buffers()
    }

    #[cfg(feature = "configurable")]
    pub fn set_direction_y(&mut self, value: f32) {
        self.config.direction_y = value;
        self.set_uniform_1f(&self.uniforms.gravity, self.config.gravity * value);
    }

    #[cfg(feature = "configurable")]
    pub fn set_rotation_speed(&mut self, value: f32) {
        self.config.rotation_speed = value;
        self.set_uniform_1f(&self.uniforms.rotation_speed, value);
    }

    pub fn get_fps(&self) -> u32 {
        self.current_fps
    }
    pub fn get_time(&self) -> f32 {
        self.time
    }
    pub fn get_wind(&self) -> f32 {
        self.wind.current
    }
    pub fn get_particle_count(&self) -> u32 {
        self.actual_particle_count as u32
    }
    pub fn get_config(&self) -> SnowConfig {
        self.config
    }
    pub fn is_configurable(&self) -> bool {
        RUNTIME_CONFIGURABLE
    }
}
