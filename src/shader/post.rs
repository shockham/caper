
/// OpenGL 3.3 default post shaders
pub mod gl330 {
    /// Default post vertex shader
    pub const VERT: &'static str =
        "
        #version 330

        layout(location = 0) in vec3 position;
        layout(location = 1) in vec2 texture;

        out vec2 v_tex_coords;

        void main() {
            gl_Position = vec4(position, 1.0);
            v_tex_coords = texture;
        }
    ";
    /// Default post fragment shader
    pub const FRAG: &'static str = 
        "
        #version 330

        uniform vec2 resolution;
        uniform sampler2D tex;
        uniform sampler2D depth_buf;

        in vec2 v_tex_coords;

        out vec4 frag_output;

        void main() {
            vec4 color = texture(tex, v_tex_coords);
            float depth = texture(depth_buf, v_tex_coords).r;

            frag_output = color;
        }
    ";
}
