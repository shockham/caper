pub mod gl330 {
    // fragment shader
    pub const FRAG: &'static str =
        "
        #version 330

        const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

        in vec3 g_normal;
        in vec3 g_pos;

        out vec4 frag_output;

        void main() {
            float lum = max(dot(normalize(g_normal), normalize(LIGHT)), 0.0);
            float dist = max(dot(normalize(g_pos), normalize(LIGHT)), 0.0);

            vec3 base_color = vec3(1.0, 1.0, 1.0);

            vec3 color = base_color * (0.3 + (0.2 * lum) + (0.5 * dist));
            frag_output = vec4(color, 1.0);
        }
    ";
}
