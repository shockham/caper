#![feature(zero_one)]

extern crate glutin;

#[macro_use]
extern crate glium;

extern crate clock_ticks;

use std::thread;

mod renderer;
use renderer::{ Renderer, RenderItem };

mod utils;
use utils::*;

mod input;
use input::Input;

mod shader;
use shader::*;


fn main() {

    let input = Input::new();
    let renderer = Renderer::new();
    let shaders = Shaders::new(&renderer.display);
    
    renderer.setup();

    //load the models in to vec<Vertex>
    let render_items = vec![
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/sphere.obj")),
            shader_index: 0,
        },
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/floor.obj")),
            shader_index: 1,
        }
    ];

    //cam state
    let mut cam_pos = [ 0.0f32, 0.0, 0.0 ];
    let mut cam_rot = [ 0.0f32, 0.0, 0.0 ];

    // the main loop
    let mut accumulator = 0;
    let mut previous_clock = clock_ticks::precise_time_ns();

    loop {
        renderer.draw(cam_pos, cam_rot, &render_items, &shaders);

        // updating and handling the inputs
        input.update_inputs(&renderer.display);
        input.handle_inputs(&mut cam_pos, &mut cam_rot);

        //quit
        if input.btns_down[8].get() { break; }

        let now = clock_ticks::precise_time_ns();
        accumulator += now - previous_clock;
        previous_clock = now;

        while accumulator >= FIXED_TIME_STAMP {
            accumulator -= FIXED_TIME_STAMP;
            //updates
        }

        thread::sleep_ms(((FIXED_TIME_STAMP - accumulator) / 1000000) as u32);
    }
    /*start_loop(|| {

        Action::Continue
    });*/
}
