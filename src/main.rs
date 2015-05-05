#![feature(collections)]
#![feature(zero_one)]

extern crate glutin;

#[macro_use]
extern crate glium;

use glium::Surface;

mod utils;
use utils::*;

mod input;
use input::Input;

mod shader;
use shader::dist::*;

fn main() {
    use glium::DisplayBuild;

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .with_multisampling(16)
        .with_title("caper".to_string())
        .build_glium()
        .unwrap();
    
    // get the window for various values
    let window = display.get_window().unwrap();
    window.set_cursor_state(glutin::CursorState::Grab).ok();

    //load the models in to vec<Vertex>
    let mut vertex_data = load_wavefront(include_bytes!("assets/birdbuilding.obj"));
    vertex_data.append(&mut load_wavefront(include_bytes!("assets/floor.obj")));

    // building the vertex and index buffers
    let vertex_buffer = glium::vertex::VertexBuffer::new(&display, vertex_data);
    
    // the shader programs
    let program = match glium::Program::from_source(&display,
            shader::dist::vert(),
            shader::dist::frag(),
            Some(shader::dist::geom())
        ){
            Ok(p) => p,
            Err(e) => panic!("glsl error: {}", e), 
        };
    
    // state for input and camera
    let input = Input::new();
    let mut cam_pos = [ 0.0f32, 0.0, 0.0 ];
    let mut cam_rot = [ 0.0f32, 0.0, 0.0 ];
    
    // define the update function
    // TODO make this less useless
    let update = || { };

    // the main loop
    start_loop(|| {
        // building the uniforms, and getting various values for this 
        let mv_matrix = build_fp_view_matrix(cam_pos, cam_rot);
        // possibly set this to an event
        let (width, height) = window.get_inner_size().unwrap_or((800, 600));
        
        let uniforms = uniform! {
            projection_matrix: build_persp_proj_mat(60f32, width as f32/height as f32, 0.01f32, 1000f32),
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
        
        // updating and handling the inputs
        input.update_inputs(&display);
        input.handle_inputs(&mut cam_pos, &mut cam_rot, mv_matrix);

        //quit
        if input.btns_down[8].get() { return Action::Stop; }

        Action::Continue
    }, update);
}
