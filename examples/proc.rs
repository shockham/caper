extern crate time;

#[macro_use]
extern crate caper;

#[macro_use]
extern crate imgui;

use caper::utils::load_wavefront;
use caper::renderer::{ RenderItem, Transform };
use imgui::*;

fn main() {
    // generate the instance positions
    let transforms = (0 .. 200)
        .map(|i| {
            Transform {
                pos: ((i as f32 % 10f32) * 2f32, 0.0f32, (i as f32 / 10f32) * 2f32),
                rot: (0f32, 0f32, 0f32, 1f32),
                scale: (1f32, 1f32, 1f32),
                update_fn: Vec::new(),
            }
        })
    .collect::<Vec<_>>();

    // create a vector of render items
    let mut render_items = vec![
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/sphere.obj")),
            shader_index: 1,
            instance_transforms: transforms
        }
    ];

    let mut text_items = Vec::new();

    game_loop! {
        input,
        renderer,
        shaders,
        cam_state,
        render_items,
        text_items,
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
            let update_time = time::precise_time_s();

            render_items[0].instance_transforms =
                render_items[0].instance_transforms.iter().map(|t| {
                    Transform {
                        pos: (t.pos.0,
                              ((t.pos.0 / 5f32).sin() *
                               (t.pos.2 / 5f32).cos() *
                               update_time.sin() as f32) * 2f32,
                               t.pos.2),
                        rot: (0f32, 0f32, 0f32, 1f32),
                        scale: (update_time.sin() as f32,
                               update_time.sin() as f32,
                               update_time.sin() as f32),
                        update_fn: Vec::new(),
                    }
                }).collect::<Vec<_>>();
        },
        ui => {

        }
    }
}
