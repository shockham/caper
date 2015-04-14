#![feature(collections)]

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

    //load the models in to vec<Vertex>
    let mut vertex_data = support::load_wavefront(include_bytes!("assets/untitled.obj"));
    vertex_data.append(&mut support::load_wavefront(include_bytes!("assets/teapot.obj")));

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

            void main() {
                v_normal = normal;
                gl_Position = projection_matrix * modelview_matrix * vec4(position, 1.0);
            }
        ",

        // fragment shader
        "
            #version 330

            const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

            in vec3 g_normal;
            
            out vec4 frag_output;

            void main() {
                float lum = max(dot(normalize(g_normal), normalize(LIGHT)), 0.0);
                vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
                frag_output = vec4(color, 1.0);
            }
        ",

        // geometry shader
        Some("
            #version 330

            layout(triangles) in;
            layout(triangle_strip, max_vertices=3) out;

            in vec3 v_normal[3];

            out vec3 g_normal;

            void main(void) {   
                for(int i=0; i<3; i++){
                    g_normal = v_normal[i];
                    gl_Position = gl_in[i].gl_Position;
                    EmitVertex();
                }
                EndPrimitive();
            }
        ")){
            Ok(p) => p,
            Err(e) => panic!("glsl error: {}", e), 
        };
    
    //quick and dirty vars for cam movement
    let mut cam_z = 0.0f32;
    let mut cam_x = 0.0f32;
    let move_speed = 0.2f32;

    fn update(){
        //put updates here
    }

    let mut move_btn_down = [false, false, false, false];

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
        // I kind of feel like this is a bit ugly
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return support::Action::Stop,
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, vkey) => {
                    match vkey{
                        Some(glutin::VirtualKeyCode::Escape) => return support::Action::Stop,
                        Some(glutin::VirtualKeyCode::W) => move_btn_down[0] = true,
                        Some(glutin::VirtualKeyCode::S) => move_btn_down[1] = true,
                        Some(glutin::VirtualKeyCode::A) => move_btn_down[2] = true,
                        Some(glutin::VirtualKeyCode::D) => move_btn_down[3] = true,
                        Some(k) => println!("pressed key: {:?}", k),
                        _ => ()
                    }
                }, 
                glutin::Event::KeyboardInput(glutin::ElementState::Released, _, vkey) => {
                    match vkey{
                        Some(glutin::VirtualKeyCode::W) => move_btn_down[0] = false,
                        Some(glutin::VirtualKeyCode::S) => move_btn_down[1] = false,
                        Some(glutin::VirtualKeyCode::A) => move_btn_down[2] = false,
                        Some(glutin::VirtualKeyCode::D) => move_btn_down[3] = false,
                        Some(k) => println!("released key: {:?}", k),
                        _ => ()
                    }
                }, 
                _ => ()
            }
        }
        
        //changing the camera position based on input events
        if move_btn_down[0] {
            cam_z += move_speed;
        }

        if move_btn_down[1] {
            cam_z -= move_speed;
        }

        if move_btn_down[2] {
            cam_x += move_speed;
        }

        if move_btn_down[3] {
            cam_x -= move_speed;
        }

        support::Action::Continue
    }, update);
}
