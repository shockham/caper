#![feature(collections)]
#![feature(zero_one)]

extern crate glutin;

#[macro_use]
extern crate glium;


mod renderer;
use renderer::Renderer;

mod utils;
use utils::*;

mod input;
use input::Input;

mod shader;

fn main() {

    let renderer = Renderer::new();
    renderer.setup();

    //load the models in to vec<Vertex>
    let mut vertex_data = load_wavefront(include_bytes!("assets/ship.obj"));
    vertex_data.append(&mut load_wavefront(include_bytes!("assets/floor.obj")));

    // state for input and camera
    let input = Input::new();
    let mut cam_pos = [ 0.0f32, 0.0, 0.0 ];
    let mut cam_rot = [ 0.0f32, 0.0, 0.0 ];

    // the main loop
    start_loop(|| {
        renderer.draw(cam_pos, cam_rot, &mut vertex_data);

        // updating and handling the inputs
        input.update_inputs(&renderer.display);
        input.handle_inputs(&mut cam_pos, &mut cam_rot);

        //quit
        if input.btns_down[8].get() { return Action::Stop; }

        Action::Continue
    });
}
