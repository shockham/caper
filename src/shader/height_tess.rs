pub mod gl330 {
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
    
    pub const TESS_CONTROL: &'static str =
        // tessellation control shader
        "
        #version 400

        layout(vertices = 3) out;
        
        in vec3 v_normal[];

        out vec3 tc_normal[];

        const float tess_level = 5.0;

        void main() {
            tc_normal[gl_InvocationID] = v_normal[gl_InvocationID];
            gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;

            gl_TessLevelOuter[0] = tess_level;
            gl_TessLevelOuter[1] = tess_level;
            gl_TessLevelOuter[2] = tess_level;
            gl_TessLevelInner[0] = tess_level;
        }
    ";
    
    pub const TESS_EVAL: &'static str =
        // tessellation evaluation shader
        "
        #version 400
        
        uniform mat4 projection_matrix;
        uniform mat4 modelview_matrix;
        uniform float time;

        layout(triangles, equal_spacing, ccw) in;
        
        in vec3 tc_normal[];

        out vec3 te_normal;
        out vec3 te_pos;

        float rand (vec2 s) {
            return fract(sin(dot(s, vec2(12.9898, 78.233))) * 43758.5453); 
        }

        vec3 tess_calc (vec3 one, vec3 two, vec3 three) {
            return ((gl_TessCoord.x) * one) +
                            ((gl_TessCoord.y) * two) +
                            ((gl_TessCoord.z) * three); 
        }

        void main () {
            te_normal = tess_calc(tc_normal[0], tc_normal[1], tc_normal[2]);

            vec3 position = tess_calc(gl_in[0].gl_Position.xyz,
                gl_in[1].gl_Position.xyz,
                gl_in[2].gl_Position.xyz);

            position += rand(position.xy + time);

            te_pos = position;

            gl_Position = projection_matrix *
                modelview_matrix *
                vec4(position, 1.0);
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
