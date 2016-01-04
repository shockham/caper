extern crate time;

#[macro_use]
extern crate caper;

use caper::utils::load_wavefront;
use caper::renderer::{ RenderItem, Transform };
use caper::mesh::gen_quad;

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
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (0.5f32, 0.5f32, 0.5f32)
                },
                Transform {
                    pos: (0.0f32.sin(), 0.0, 0.0f32.cos()),
                    rot: (0f32, 0f32, 0f32, 0f32),
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
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32)
                },
                Transform {
                    pos: (15.0, 0.0, 0.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (2.0f32, 2.0f32, 2.0f32)
                }
            ]
        },
        RenderItem {
            vertices: gen_quad(),
            shader_index: 0,
            instance_transforms: vec![
                Transform {
                    pos: (0.0, 1.0, 0.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32)
                }
            ]
        }
    ];

    game_loop! {
        // pass the items to be rendered
        render_items,
        {
            // update some items
            let update_time = time::precise_time_s();

            render_items[0].instance_transforms[0].pos = 
                (0.0, update_time.sin() as f32, 0.0);
            render_items[0].instance_transforms[1].pos = 
                (update_time.sin() as f32 * 3.0, 0.0, update_time.cos() as f32 * 3.0);

            render_items[1].instance_transforms[1].rot =
                (update_time.cos() as f32, 0.0, 0.0, update_time.sin() as f32);
        }
    }
}
