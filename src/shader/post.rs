
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

        #define M_PI 3.1415926535897932384626433832795

        uniform vec2 resolution;
        uniform sampler2D tex;
        uniform sampler2D depth_buf;

        // chromatic aberration params
        uniform float chrom_amt;
        uniform float chrom_offset;

        in vec2 v_tex_coords;

        out vec4 frag_output;

        void main() {
            vec4 color = texture(tex, v_tex_coords);
            float depth = texture(depth_buf, v_tex_coords).r;

            // pseudo chromatic aberration
            vec2 edge_offset = vec2(cos(v_tex_coords.x * M_PI), cos(v_tex_coords.y * M_PI)) * chrom_offset;
            float chrom_r = texture(tex, v_tex_coords + edge_offset).r;
            color.r = mix(color.r, chrom_r, chrom_amt);

            frag_output = color;
        }
    ";
}
