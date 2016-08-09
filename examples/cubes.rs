extern crate time;

#[macro_use]
extern crate caper;

#[macro_use]
extern crate imgui;

extern crate noise;

use caper::utils::load_wavefront;
use caper::types::{ RenderItem, Transform };
use noise::{ perlin2, Seed };
use imgui::*;

fn main() {
    // generate the instance positions
    let map_size = 50f32;
    let transforms = (0 .. 2500)
        .map(|i| {
            let pos = ((i as f32 % map_size) * 2f32, ((i / map_size as i32) * 2) as f32);
            let size = perlin2(&Seed::new(0), &[pos.0 / 10f32, pos.1 / 10f32]).abs() * 8f32;
            Transform {
                active: true,
                pos: (pos.0 * 5f32, size, pos.1 * 5f32),
                rot: (0f32, 0f32, 0f32, 1f32),
                scale: (4.2f32, size, 4.2f32),
                update_fn: Vec::new(),
            }
        })
    .collect::<Vec<_>>();

    // create a vector of render items
    let mut render_items = vec![
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/cube.obj")),
            shader_name: "dist",
            instance_transforms: transforms,
            active: true,
        }
    ];

    let mut text_items = Vec::new();

    game_loop! {
        Input => input,
        Renderer => renderer,
        CamState => cam_state,
        RenderItems => render_items,
        TextItems => text_items,
        // define a block for start
        start => {
            // yay start code
            println!("{:?}", cam_state.cam_pos);
        },
        // define a block for update
        update => {
            // first person input
            input.handle_fp_inputs(&mut cam_state);

            // update some items
            //let update_time = time::precise_time_s();
        },
        ui => {
            // ui code here
        }
    }
}
