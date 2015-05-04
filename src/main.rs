#![feature(collections)]
#![feature(zero_one)]

extern crate glutin;

#[macro_use]
extern crate glium;

use glium::Surface;

mod utils;
use utils::*;

mod input;
mod shader;

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
    window.set_cursor_state(glutin::CursorState::Grab).ok();

    //load the models in to vec<Vertex>
    let mut vertex_data = load_wavefront(include_bytes!("assets/birdbuilding.obj"));
    vertex_data.append(&mut load_wavefront(include_bytes!("assets/floor.obj")));

    // building the vertex and index buffers
    let vertex_buffer = glium::vertex::VertexBuffer::new(&display, vertex_data);
    
    // the shader programs
    let program = match glium::Program::from_source(&display,
        shader::get_dist_vert(),
        shader::get_dist_frag(),
        Some(shader::get_dist_geom())
        ){
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
