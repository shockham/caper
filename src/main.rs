#![feature(collections)]
#![feature(zero_one)]

extern crate glutin;

#[macro_use]
extern crate glium;

use glium::Surface;

mod utils;
use utils::*;

mod input;

fn main() {
    use glium::DisplayBuild;

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .with_multisampling(16)
        .with_title("caper".to_string())
        .build_glium()
        .unwrap();

    let window = display.get_window().unwrap();

    //load the models in to vec<Vertex>
    let mut vertex_data = load_wavefront(include_bytes!("assets/floor3.obj"));
    vertex_data.append(&mut load_wavefront(include_bytes!("assets/screenolith.obj")));

    // building the vertex and index buffers
    let vertex_buffer = glium::vertex::VertexBuffer::new(&display, vertex_data);
    
    // the shader programs
    let program = match glium::Program::from_source(&display,
        // vertex shader
        "
            #version 330
            
            uniform mat4 projection_matrix;
            uniform mat4 modelview_matrix;
            
            layout(location = 0) in vec3 position;
            layout(location = 1) in vec3 normal;

            out vec3 v_normal;
            out vec3 v_pos;

            void main() {
                v_normal = normal;
                v_pos = position;
                gl_Position = projection_matrix * modelview_matrix * vec4(position, 1.0);
            }
        ",

        // fragment shader
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
        ",

        // geometry shader
        Some("
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
        ")){
            Ok(p) => p,
            Err(e) => panic!("glsl error: {}", e), 
        };
    
    // state for buttons and camera pos
    let mut btns_down = [false, false, false, false, false, false, false, false, false];
    let mut mouse_pos = (0, 0);
    let mut cam_pos = [ 0.0f32, 0.0, 0.0 ];
    let mut cam_rot = [ 0.0f32, 0.0, 0.0 ];

    let update = || {
    };

    // the main loop
    start_loop(|| {
        // building the uniforms
        
        let mv_matrix = build_fp_view_matrix(cam_pos, cam_rot);
        let uniforms = uniform! {
            projection_matrix: build_persp_proj_mat(60f32, 800f32/600f32, 0.01f32, 1000f32),
            modelview_matrix: mv_matrix,
        };

        // draw parameters
        let params = glium::DrawParameters {
            depth_test: glium::DepthTest::IfLess,
            depth_write: true,
            .. std::default::Default::default()
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);
        target.draw(&vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &program, &uniforms, &params).unwrap();
        target.finish();
        
        // polling and handling the events received by the window
        let inputs = input::get_inputs(&display, btns_down, mouse_pos);
        btns_down = inputs.0;

        let mouse_diff = (mouse_pos.0 - (inputs.1).0, mouse_pos.1 - (inputs.1).1);
        let win_size = window.get_inner_size().unwrap();
        let mouse_delta = ((mouse_diff.0 as f32)/(win_size.0 as f32), (mouse_diff.1 as f32)/(win_size.1 as f32));

        input::handle_inputs(&mut cam_pos, &mut cam_rot, btns_down, mouse_delta,  mv_matrix);
        mouse_pos = inputs.1;

        //quit
        if btns_down[8]{ return Action::Stop; }

        Action::Continue
    }, update);
}
