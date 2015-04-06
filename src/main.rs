extern crate glutin;

#[macro_use]
extern crate glium;

use glium::Surface;

mod support;

fn main() {
    use glium::DisplayBuild;

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium()
        .unwrap();

    // building the vertex and index buffers
    let vertex_buffer = support::load_wavefront(&display, include_bytes!("assets/teapot.obj"));

    // the program
    let program = match glium::Program::from_source(&display,
        // vertex shader
        "
            #version 330
            
            uniform mat4 projection_matrix;
            uniform mat4 modelview_matrix;
            
            in vec3 position;
            in vec3 normal;

            out vec3 v_normal;

            void main() {
                v_normal = normal;
                gl_Position = projection_matrix * modelview_matrix * vec4(position, 1.0);
            }
        ",

        // fragment shader
        "
            #version 330

            const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

            in vec3 v_normal;
            
            out vec4 frag_output;

            void main() {
                float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
                frag_output = vec4(color, 1.0);
            }
        ",

        // geometry shader
        None){
            Ok(p) => p,
            Err(e) => panic!("{}", e), 
        };
    
    //quick and dirty vars for cam movement
    let mut cam_z = 0.0f32;
    let mut cam_x = 0.0f32;
    let move_speed = 0.8f32;

    // the main loop
    support::start_loop(|| {
        // building the uniforms
        let uniforms = uniform! {
           projection_matrix: support::build_persp_proj_mat(60f32, 800f32/600f32, 0.01f32, 1000f32),
            modelview_matrix: [
                [1.0f32, 0.0, 0.0, 0.0],
                [0.0, 1.0f32, 0.0, 0.0],
                [0.0, 0.0, 1.0f32, 0.0],
                [cam_x, 0.0, cam_z, 1.0f32]
            ]
        };

        // draw parameters
        let params = glium::DrawParameters {
            depth_test: glium::DepthTest::IfLess,
            depth_write: true,
            .. std::default::Default::default()
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target.draw(&vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &program, &uniforms, &params).unwrap();
        target.finish();

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return support::Action::Stop,
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, vkey) => {
                    match vkey{
                        Some(glutin::VirtualKeyCode::Escape) => return support::Action::Stop,
                        Some(glutin::VirtualKeyCode::W) => cam_z += move_speed,
                        Some(glutin::VirtualKeyCode::S) => cam_z -= move_speed,
                        Some(glutin::VirtualKeyCode::A) => cam_x += move_speed,
                        Some(glutin::VirtualKeyCode::D) => cam_x -= move_speed,
                        Some(k) => println!("key: {:?}", k),
                        _ => ()
                    }
                }, 
                _ => ()
            }
        }

        support::Action::Continue
    });
}
