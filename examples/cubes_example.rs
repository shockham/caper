extern crate clock_ticks;

#[macro_use]
extern crate caper;

extern crate noise;

use caper::utils::load_wavefront;
use caper::renderer::{ RenderItem, Transform };
use noise::{ perlin2, Seed };

fn main() {
    // generate the instance positions 
    let map_size = 50f32;
    let transforms = (0 .. 2500)
        .map(|i| {
            //let size = ((i as f32 % map_size) + (i as f32 / map_size)).sin().abs();
            let pos = ((i as f32 % map_size) * 2f32, ((i / map_size as i32) * 2) as f32);
            let size = perlin2(&Seed::new(0), &[pos.0 / 10f32, pos.1 / 10f32]).abs();
            Transform {
                pos: (pos.0, size, pos.1),
                rot: (0f32, 0f32, 0f32, 1f32),
                scale: (0.8f32, size, 0.8f32)
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
