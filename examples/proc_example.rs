extern crate clock_ticks;

#[macro_use]
extern crate caper;

use caper::utils::load_wavefront;
use caper::renderer::RenderItem;

fn main() {
    // generate the instance positions 
    let positions = (0 .. 100)
        .map(|i| {
            ((i as f32 % 10f32) * 5.0f32, 0.0f32, (i as f32 / 10f32) * 5f32)
        })
        .collect::<Vec<_>>();

    // create a vector of render items
    let mut render_items = vec![
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/sphere.obj")),
            shader_index: 0,
            instance_positions: positions
        }
    ];

    game_loop! {
        // pass the items to be rendered
        render_items,

        // update some items
        let update_time = clock_ticks::precise_time_ns() as f32 / 100000000.0f32;
        
        render_items[0].instance_positions =  render_items[0].instance_positions.iter().map(|v| {
            (v.0, (v.0.sin() * v.2.cos() * update_time.sin()) * 2f32, v.2)
        }).collect::<Vec<_>>();
    }
}
