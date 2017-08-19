/// OpenGL 3.3 shaders for distance roll off
pub mod gl330 {
    /// Distance fragment shader that rolls off to white the further from the camera
    pub const FRAG: &'static str = "
        #version 330

        uniform float time;
        uniform vec3 cam_pos;
        uniform sampler2D tex;
        uniform sampler2D normal_tex;
        uniform sampler1D dir_lights;

        in vec3 g_normal;
        in vec3 g_pos;
        in vec2 g_texture;

        out vec4 frag_output;

        void main() {
            int size = textureSize(dir_lights, 0);
            float lum = 0.0;
            float tex_lum = 0.0;
            for (int i = 0; i < size; i++) {
                vec3 light_norm = normalize(texture(dir_lights, i).xyz);
                lum += max(dot(normalize(g_normal), light_norm), 0.0);
                tex_lum += dot(normalize(vec3(texture(normal_tex, g_texture))), light_norm);
            }

            float avg_lum = (lum + tex_lum) / 2.0;

            float dist = abs(distance(cam_pos, g_pos)) / 80.0;

            frag_output = texture(tex, g_texture) * vec4(vec3((0.6 * avg_lum) + (0.4 * dist)), 1.0);
        }
    ";
}
