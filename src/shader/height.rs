/// OpenGL 3.3 shaders with height roll off
pub mod gl330 {
    /// Height fragment shader that rolls off to white in relation to the y-axis
    pub const FRAG: &'static str = "
        #version 330

        uniform vec3 cam_pos;
        const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

        in vec3 g_normal;
        in vec3 g_pos;

        out vec4 frag_output;

        void main() {
            float lum = max(dot(normalize(g_normal), normalize(LIGHT)), 0.0);
            float dist = abs(distance(cam_pos, g_pos)) / 80;

            float col_val = normalize(g_pos).y;
            vec3 base_color = vec3(col_val);
            base_color += dist;

            vec3 color = base_color * ((0.2 * lum) + (0.8 * dist));
            frag_output = vec4(color, 1.0);
        }
    ";
}
