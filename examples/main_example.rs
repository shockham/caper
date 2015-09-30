extern crate clock_ticks;

#[macro_use]
extern crate caper;

use caper::utils::load_wavefront;
use caper::renderer::{ RenderItem, Transform };

fn main() {
    // load the models in to vec<Vertex>
    // for efficiency all the verts with the same shader should be one RenderItem
    let mut render_items = vec![
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/sphere.obj")),
            shader_index: 0,
            instance_transforms: vec![
                Transform {
                    pos: (0.0, (0.0 as f32).sin(), 0.0),
                    rot: (0f32, 0f32, 0f32),
                    scale: (0.5f32, 0.5f32, 0.5f32)
                },
                Transform {
                    pos: (0.0f32.sin(), 0.0, 0.0f32.cos()),
                    rot: (0f32, 0f32, 0f32),
                    scale: (1f32, 1f32, 1f32)
                }
            ]
        },
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/floor.obj")),
            shader_index: 1,
            instance_transforms: vec![
                Transform {
                    pos: (0.0, 0.0, 0.0),
                    rot: (0f32, 0f32, 0f32),
                    scale: (1f32, 1f32, 1f32)
                },
                Transform {
                    pos: (10.0, 0.0, 0.0),
                    rot: (0f32, 0f32, 0f32),
                    scale: (0.5f32, 0.5f32, 0.5f32)
                }
            ]
        }
    ];

    game_loop! {
        // pass the items to be rendered
        render_items,

        // update some items
        let update_time = clock_ticks::precise_time_ns() as f32 / 100000000.0f32;
        render_items[0].instance_transforms[0].pos = 
            (0.0, update_time.sin(), 0.0);
        render_items[0].instance_transforms[1].pos = 
            (update_time.sin() * 3.0, 0.0, update_time.cos() * 3.0);
    }
}
