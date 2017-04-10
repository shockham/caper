
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
            vec2 tex_size = vec2(1.0 - chrom_offset);
            float edge_fade = 1.0 - (sin(v_tex_coords.x * M_PI) * sin(v_tex_coords.y * M_PI));
            float chrom_r = texture(tex, vec2(min(v_tex_coords.x + chrom_offset, tex_size.x), v_tex_coords.y)).r;
            float chrom_g = texture(tex, vec2(min(v_tex_coords.x - chrom_offset, tex_size.x), v_tex_coords.y)).g;
            float chrom_b = texture(tex, vec2(v_tex_coords.x, min(v_tex_coords.y + chrom_offset, tex_size.y))).b;
            color.r = mix(color.r, chrom_r, edge_fade * chrom_amt);
            color.b = mix(color.g, chrom_g, edge_fade * chrom_amt);
            color.b = mix(color.b, chrom_b, edge_fade * chrom_amt);

            frag_output = color;
        }
    ";
}
