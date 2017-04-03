/// OpenGL 3.3 shaders for distance roll off
pub mod gl330 {
    /// Distance fragment shader that rolls off to white the further from the camera
    pub const FRAG: &'static str =
        "
        #version 330

        uniform float time;
        uniform vec3 cam_pos;
        uniform sampler2D tex;
        uniform sampler2D normal_tex;
        const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

        in vec3 g_normal;
        in vec3 g_pos;
        in vec2 g_texture;

        out vec4 frag_output;

        void main() {
            float lum = dot(normalize(g_normal), normalize(LIGHT));
            float tex_lum = dot(normalize(vec3(texture(normal_tex, g_texture))), normalize(LIGHT));

            float avg_lum = (lum + tex_lum) / 2.0;

            float dist = abs(distance(cam_pos, g_pos)) / 80.0;

            frag_output = texture(tex, g_texture) * vec4(vec3((0.6 * avg_lum) + (0.4 * dist)), 1.0);
        }
    ";
}
