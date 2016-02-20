pub mod gl330 {
    pub const VERT: &'static str =
        // vertex shader
        "
        #version 330

        uniform mat4 projection_matrix;
        uniform mat4 modelview_matrix;

        layout(location = 0) in vec3 position;
        layout(location = 1) in vec3 normal;
        layout(location = 2) in vec3 world_position;
        layout(location = 3) in vec4 world_rotation;
        layout(location = 4) in vec3 world_scale;

        out vec3 v_normal;
        out vec3 v_pos;

        void main() {
            vec3 pos_scaled = position * world_scale;

            vec3 temp = cross(world_rotation.xyz, pos_scaled) + world_rotation.w * pos_scaled;
            vec3 pos_rotated = pos_scaled + 2.0 * cross(world_rotation.xyz, temp);

            vec3 pos_final = pos_rotated + world_position;

            gl_Position = projection_matrix *
                modelview_matrix *
                vec4(pos_final, 1.0);
            
            v_normal = normal;
            v_pos = pos_final;
        }
    ";

    pub const FRAG: &'static str =
        // fragment shader
        "
        #version 330

        uniform vec3 cam_pos;
        const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

        in vec3 g_normal;
        in vec3 g_pos;

        out vec4 frag_output;

        void main() {
            float lum = max(dot(normalize(g_normal), normalize(LIGHT)), 0.0);
            float dist = (abs(distance(cam_pos, g_pos)) / 25);

            float col_val = normalize(g_pos).y;
            vec3 base_color = vec3(col_val);
            base_color += dist; 

            //base_color.r *= step(0.05, col_val);

            vec3 color = base_color * ((0.2 * lum) + (0.8 * dist));
            frag_output = vec4(color, 1.0);
        }
    ";

    pub const GEOM: &'static str =
        // geometry shader
        "
        #version 330

        layout(triangles) in;
        layout(triangle_strip, max_vertices=3) out;

        in vec3 v_normal[3];
        in vec3 v_pos[3];

        out vec3 g_normal;
        out vec3 g_pos;

        void main(void) {   
            for(int i=0; i<3; i++){
                g_normal = v_normal[i];
                g_pos = v_pos[i];
                gl_Position = gl_in[i].gl_Position;
                EmitVertex();
            }
            EndPrimitive();
        }
    ";
}

pub mod gl110 {
    pub const VERT: &'static str =
        // vertex shader
        "
        #version 110

        uniform mat4 projection_matrix;
        uniform mat4 modelview_matrix;

        attribute vec3 position;
        attribute vec3 normal;
        attribute vec3 world_position;
        attribute vec4 world_rotation;
        attribute vec3 world_scale;

        varying vec3 v_normal;
        varying vec3 v_pos;

        void main() {
            v_normal = normal;
            v_pos = position;
            gl_Position = projection_matrix *
                modelview_matrix *
                vec4((position * world_scale) + world_position, 1.0);
        }
    ";

    pub const FRAG: &'static str =
        // fragment shader
        "
        #version 110

        uniform vec3 cam_pos;
        const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

        varying vec3 v_normal;
        varying vec3 v_pos;

        void main() {
            float lum = max(dot(normalize(g_normal), normalize(LIGHT)), 0.0);
            float dist = (abs(distance(cam_pos, g_pos)) / 25);

            float col_val = normalize(g_pos).y;
            vec3 base_color = vec3(col_val)
            base_color += dist; 
            
            //base_color.r *= step(0.05, col_val);

            vec3 color = base_color * ((0.2 * lum) + (0.8 * dist));
            gl_FragColor = vec4(color, 1.0);
        }
    ";
}
