#![feature(zero_one)]

#[macro_use]
extern crate glium;
extern crate glium_text;

extern crate clock_ticks;

use std::thread;

mod renderer;
use renderer::{ Renderer, RenderItem, FIXED_TIME_STAMP };

mod utils;
use utils::load_wavefront;

mod input;
use input::Input;

mod shader;
use shader::Shaders;


fn main() {

    let input = Input::new();
    let renderer = Renderer::new();
    let shaders = Shaders::new(&renderer.display);

    renderer.setup();

    // load the models in to vec<Vertex>
    // for efficiency all the verts with the same shader should be one RenderItem
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

        let now = clock_ticks::precise_time_ns();
        accumulator += now - previous_clock;
        previous_clock = now;

        while accumulator >= FIXED_TIME_STAMP {
            accumulator -= FIXED_TIME_STAMP;

            // updating and handling the inputs
            input.update_inputs(&renderer.display);
            input.handle_inputs(&mut cam_pos, &mut cam_rot);
        }

        //quit
        if input.btns_down[8].get() { break; }

        thread::sleep_ms(((FIXED_TIME_STAMP - accumulator) / 1000000) as u32);
    }
}
