//! GLSL shaders

pub const VERTEX_SHADER_SOURCE: &str = r#"
    precision highp float;

    attribute vec3 a_position;
    attribute vec4 a_color;
    attribute vec3 a_rotation;
    attribute vec3 a_speed;
    attribute float a_size;

    uniform float u_time;
    uniform mat4 u_projection;
    uniform vec3 u_worldSize;
    uniform float u_gravity;
    uniform float u_wind;
    uniform float u_pointScale;      // precomputed: resolution.y * 0.015
    uniform float u_rotationSpeed;

    // Use lower-precision varyings where suitable to save bandwidth on mobile GPUs
    varying lowp vec4 v_color;
    varying mediump vec2 v_rotSinCos;  // Pre-computed sin/cos for rotation

    void main() {
        mediump float t = u_time;
        mediump float rot_factor = t * a_rotation.y * u_rotationSpeed * 0.1;
        mediump float oscillation = t * a_speed.z * u_rotationSpeed * 0.3;
        mediump float swing = a_rotation.z * 2.0;

        // Evaluate sin/cos once for the shared oscillation argument
        mediump float s = sin(oscillation);
        mediump float c = cos(oscillation);

        vec3 pos = a_position;

        pos.x = mod(pos.x + t * 0.5 + u_wind * a_speed.x, u_worldSize.x * 2.0) - u_worldSize.x;
        pos.y = mod(pos.y - t * a_speed.y * u_gravity, u_worldSize.y * 2.0) - u_worldSize.y;

        pos.x += s * swing;
        pos.z += c * swing;

        vec4 projected = u_projection * vec4(pos, 1.0);
        gl_Position = projected;
        gl_PointSize = a_size * u_pointScale / projected.w;

        v_color = a_color;
        
        mediump float final_rotation = a_rotation.x + rot_factor;
        v_rotSinCos = vec2(sin(final_rotation), cos(final_rotation));
    }
"#;

pub const FRAGMENT_SHADER_SOURCE: &str = r#"
    precision mediump float;

    uniform sampler2D u_texture;
    uniform bool u_useTexture;
    uniform vec3 u_colorTint;  // default (1,1,1)
    uniform bool u_useColorTint;
    varying lowp vec4 v_color;
    varying mediump vec2 v_rotSinCos;

    void main() {
        vec2 coord = gl_PointCoord - 0.5;

        vec2 rotated = vec2(
            coord.x * v_rotSinCos.y - coord.y * v_rotSinCos.x,
            coord.x * v_rotSinCos.x + coord.y * v_rotSinCos.y
        );

        if (u_useTexture) {
            vec4 texColor = texture2D(u_texture, rotated + 0.5);
            if (u_useColorTint) {
                float intensity = dot(texColor.rgb, vec3(0.299, 0.587, 0.114));
                gl_FragColor = vec4(u_colorTint * intensity * v_color.rgb, texColor.a * v_color.a);
            } else {
                gl_FragColor = vec4(texColor.rgb * v_color.rgb, texColor.a * v_color.a);
            }
            return;
        }

        // default (circle)
        float dist_sq = dot(rotated, rotated);
        
        // thresholds: radius 0.25 and 0.5 squared = 0.0625, 0.25
        float alpha = 1.0 - smoothstep(0.0625, 0.25, dist_sq);

        // Glow using squared distance approximation
        // exp(-sqrt(x) * 5) ≈ exp(-x * 2.5) for small x
        float glow = exp(-dist_sq * 10.0) * 0.4;

        gl_FragColor = vec4(u_colorTint * (1.0 + glow), alpha * v_color.a);
    }
"#;
