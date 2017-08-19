/// OpenGL 3.3 default shaders
pub mod gl330 {
    /// Default vertex shader
    pub const VERT: &'static str = "
        #version 330

        layout(location = 0) in vec3 position;
        layout(location = 1) in vec3 normal;
        layout(location = 2) in vec3 world_position;
        layout(location = 3) in vec4 world_rotation;
        layout(location = 4) in vec3 world_scale;
        layout(location = 5) in vec2 texture;

        out vec3 v_normal;
        out vec2 v_texture;

        void main() {
            vec3 pos_scaled = position * world_scale;

            vec3 temp = cross(world_rotation.xyz, pos_scaled) + world_rotation.w * pos_scaled;
            vec3 pos_rotated = pos_scaled + 2.0 * cross(world_rotation.xyz, temp);

            vec3 pos_final = pos_rotated + world_position;

            gl_Position = vec4(pos_final, 1.0);

            v_normal = normal;
            v_texture = texture;
        }
    ";

    /// Default geometry shader
    pub const GEOM: &'static str = "
        #version 330

        layout(triangles) in;
        layout(triangle_strip, max_vertices=3) out;

        in vec3 te_normal[];
        in vec3 te_pos[];
        in vec2 te_texture[];

        out vec3 g_normal;
        out vec3 g_pos;
        out vec2 g_texture;

        void main(void) {
            for(int i = 0; i < gl_in.length(); i++){
                g_normal = te_normal[i];
                g_pos = te_pos[i];
                g_texture = te_texture[i];
                gl_Position = gl_in[i].gl_Position;
                EmitVertex();
            }
            EndPrimitive();
        }
    ";

    /// Default tessellation control shader
    pub const TESS_CONTROL: &'static str = "
        #version 400

        layout(vertices = 3) out;

        in vec3 v_normal[];
        in vec2 v_texture[];

        out vec3 tc_normal[];
        out vec2 tc_texture[];

        const float tess_level = 1.0;

        void main() {
            tc_normal[gl_InvocationID] = v_normal[gl_InvocationID];
            tc_texture[gl_InvocationID] = v_texture[gl_InvocationID];
            gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;

            gl_TessLevelOuter[0] = tess_level;
            gl_TessLevelOuter[1] = tess_level;
            gl_TessLevelOuter[2] = tess_level;
            gl_TessLevelInner[0] = tess_level;
        }
    ";

    /// Default tessellation evaluation shader
    pub const TESS_EVAL: &'static str = "
        #version 400

        uniform mat4 projection_matrix;
        uniform mat4 modelview_matrix;

        layout(triangles, equal_spacing, ccw) in;

        in vec3 tc_normal[];
        in vec2 tc_texture[];

        out vec3 te_normal;
        out vec3 te_pos;
        out vec2 te_texture;

        vec3 tess_calc (vec3 one, vec3 two, vec3 three) {
            return ((gl_TessCoord.x) * one) +
                            ((gl_TessCoord.y) * two) +
                            ((gl_TessCoord.z) * three);
        }

        vec2 tex_calc (vec2 one, vec2 two, vec2 three) {
            return ((gl_TessCoord.x) * one) +
                            ((gl_TessCoord.y) * two) +
                            ((gl_TessCoord.z) * three);
        }

        void main () {
            te_normal = tess_calc(tc_normal[0], tc_normal[1], tc_normal[2]);

            vec3 position = tess_calc(gl_in[0].gl_Position.xyz,
                gl_in[1].gl_Position.xyz,
                gl_in[2].gl_Position.xyz);

            te_pos = position;

            vec2 texture = tex_calc(tc_texture[0], tc_texture[1], tc_texture[2]);
            te_texture = texture;

            gl_Position = projection_matrix *
                modelview_matrix *
                vec4(position, 1.0);
        }
    ";
}
