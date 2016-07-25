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

            vec3 base_color = vec3(1.0, 0.0, 0.0);

            vec3 color = base_color * (0.3 + (0.2 * lum) + (0.5 * dist));
            frag_output = vec4(color, 1.0);
        }
    ";

    // geometry shader
    pub const GEOM: &'static str =
        "
        #version 330

        layout(triangles) in;
        layout(line_strip, max_vertices=3) out;

        in vec3 te_normal[];
        in vec3 te_pos[];

        out vec3 g_normal;
        out vec3 g_pos;

        void main(void) {
            for(int i = 0; i < gl_in.length(); i++){
                g_normal = te_normal[i];
                g_pos = te_pos[i];
                gl_Position = gl_in[i].gl_Position;
                EmitVertex();
            }
            EndPrimitive();
        }
    ";
}
