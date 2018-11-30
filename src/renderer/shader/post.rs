/// OpenGL 3.3 default post shaders
pub mod gl330 {
    /// Default post vertex shader
    pub const VERT: &str = "
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
    pub const FRAG: &str = "
        #version 330

        #define M_PI 3.1415926535897932384626433832795

        uniform vec2 resolution;
        uniform sampler2D tex;
        uniform sampler2D depth_buf;
        uniform float time;

        // chromatic aberration params
        uniform float chrom_amt;
        uniform float chrom_offset;

        // blur params
        uniform bool blur;
        uniform float blur_amt;
        uniform float blur_radius;
        uniform float blur_weight;

        // bokeh params
        uniform bool bokeh;
        uniform float bokeh_focal_depth;
        uniform float bokeh_focal_width;

        // color grading/offset
        uniform vec4 color_offset;

        // greyscale
        uniform bool greyscale;

        // noise
        uniform float noise;

        // scanline
        uniform float scanline;
        uniform int scanline_count;

        in vec2 v_tex_coords;

        out vec4 frag_output;

        float w_offset[4] = float[](
            (-1.5 * blur_radius) / resolution.x,
            (-0.5 * blur_radius) / resolution.x,
            (0.5 * blur_radius) / resolution.x,
            (1.5 * blur_radius) / resolution.x
        );
        float h_offset[4] = float[](
            (-1.5 * blur_radius) / resolution.y,
            (-0.5 * blur_radius) / resolution.y,
            (0.5 * blur_radius) / resolution.y,
            (1.5 * blur_radius) / resolution.y
        );

        float rand (vec2 s) {
            return fract(sin(dot(s, vec2(12.9898, 78.233))) * 43758.5453);
        }

        void main() {
            vec4 color = texture(tex, v_tex_coords);
            float depth = texture(depth_buf, v_tex_coords).r;

            // scanline
            if (scanline > 0.0) {
                color = texture(tex,
                    vec2(
                        v_tex_coords.x
                            + rand(
                                vec2(ceil(v_tex_coords.y * scanline_count) / scanline_count, 0.5)
                            )
                            * scanline
                            * sin(tan(time)),
                        v_tex_coords.y
                    )
                );
            }

            // pseudo chromatic aberration
            if (chrom_amt > 0.0) {
                vec2 edge_offset =
                    vec2(cos(v_tex_coords.x * M_PI), cos(v_tex_coords.y * M_PI)) * chrom_offset;
                float chrom_r =
                    texture(tex, clamp(v_tex_coords + edge_offset, vec2(0.0), vec2(1.0))).r;
                color.r = mix(color.r, chrom_r, chrom_amt);
            }

            // blur
            vec3 blur_color = color.rgb;
            if (blur || bokeh) {
               for (int i = 0 ; i < 4 ; i++) {
                    for (int j = 0 ; j < 4 ; j++) {
                        vec2 sample_pos =
                            vec2(v_tex_coords.x + w_offset[j], v_tex_coords.y + h_offset[i]);
                        blur_color += texture(tex, clamp(sample_pos, vec2(0.01), vec2(0.99))).xyz;
                    }
                }

                blur_color /= 16.0;
                blur_color *= blur_weight;
            }

            // mix with depth buffer for bokeh
            if (bokeh) {
                float bokeh_blur_amt = abs(sin(depth * M_PI / 2.0) - bokeh_focal_depth);
                float focal_width_amt =
                    smoothstep(0.0, bokeh_focal_width / 2.0, bokeh_blur_amt) * bokeh_blur_amt;
                color = mix(color, vec4(blur_color, 1.0), focal_width_amt);
            } else if (blur) {
                color = mix(color, vec4(blur_color, 1.0), blur_amt);
            }

            // color grading and noise
            vec4 graded = color  * color_offset;

            // noise
            graded = mix(graded, vec4(vec3(rand(v_tex_coords + time)), 1.0), noise);

            // greyscale or not
            if (greyscale) {
                frag_output = vec4(vec3((graded.r + graded.g + graded.b) / 3.0), graded.a);
            } else {
                frag_output = graded;
            }
        }
    ";
}
