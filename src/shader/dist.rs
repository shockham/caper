/// OpenGL 3.3 shaders for distance roll off
pub mod gl330 {
    /// Distance fragment shader that rolls off to white the further from the camera
    pub const FRAG: &'static str = "
        #version 330

        uniform vec3 cam_pos;
        uniform sampler1D dir_lights;

        in vec3 g_normal;
        in vec3 g_pos;

        out vec4 frag_output;

        void main() {
            int size = textureSize(dir_lights, 0);
            float lum = 0.0;
            for (int i = 0; i < size; i++) {
                vec3 light_norm = normalize(texture(dir_lights, i).xyz);
                lum += max(dot(normalize(g_normal), light_norm), 0.0);
            }

            float dist = abs(distance(cam_pos, g_pos)) / 80.0;

            vec3 color = vec3(0.3 + (0.2 * lum) + (0.5 * dist));
            frag_output = vec4(color, 1.0);
        }
    ";
}
