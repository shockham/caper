extern crate clock_ticks;

#[macro_use]
extern crate caper;

use caper::utils::load_wavefront;
use caper::renderer::{ RenderItem, Transform };

fn main() {
    // generate the instance positions 
    let transforms = (0 .. 200)
        .map(|i| {
            let size = (i as f32 / 20f32).sin().abs();
            Transform {
                pos: ((i as f32 % 10f32) * 2f32, size, ((i / 10) * 2) as f32),
                rot: (0f32, 0f32, 0f32, 1f32),
                scale: (1f32, size, 1f32)
            }
        })
    .collect::<Vec<_>>();

    // create a vector of render items
    let render_items = vec![
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/cube.obj")),
            shader_index: 0,
            instance_transforms: transforms
        }
    ];

    game_loop! {
        // pass the items to be rendered
        render_items,
        // define a block for update
        { 
            // update some items
            //let update_time = clock_ticks::precise_time_s();
        }
    }
}
