extern crate clock_ticks;
extern crate caper;

use std::thread;
use caper::renderer::{ Renderer, RenderItem, CamState, FIXED_TIME_STAMP };
use caper::utils::load_wavefront;
use caper::input::Input;
use caper::shader::Shaders;

fn main() {

    let input = Input::new();
    let renderer = Renderer::new();
    let shaders = Shaders::new(&renderer.display);

    renderer.setup();

    // load the models in to vec<Vertex>
    // for efficiency all the verts with the same shader should be one RenderItem
    let mut render_items = vec![
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/sphere.obj")),
            shader_index: 0,
            instance_positions: vec![
                (0.0, (0.0 as f32).sin(), 0.0),
                (5.0, 0.0, 0.0)
            ]
        },
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/floor.obj")),
            shader_index: 1,
            instance_positions: vec![(0.0, 0.0, 0.0)]
        }
    ];

    //cam state
    let mut cam_state = CamState {
        cam_pos: [ 0.0f32, 0.0, 0.0 ],
        cam_rot: [ 0.0f32, 0.0, 0.0 ]
    };

    // the main loop
    let mut accumulator = 0;
    let mut previous_clock = clock_ticks::precise_time_ns();
    loop {
        renderer.draw(cam_state, &render_items, &shaders);

        let now = clock_ticks::precise_time_ns();
        accumulator += now - previous_clock;
        previous_clock = now;

        while accumulator >= FIXED_TIME_STAMP {
            accumulator -= FIXED_TIME_STAMP;

            // keeping the camera on a single plane
            cam_state.cam_pos[1] = -1.0f32;

            // updating and handling the inputs
            input.update_inputs(&renderer.display);
            input.handle_inputs(&mut cam_state);

            // update an item
            render_items[0].instance_positions[0] = (0.0, ((clock_ticks::precise_time_ns() as f32) / 30.0).sin(), 0.0);
        }

        //quit
        if input.btns_down[8].get() { break; }

        thread::sleep_ms(((FIXED_TIME_STAMP - accumulator) / 1000000) as u32);
    }
}
