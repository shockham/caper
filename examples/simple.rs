#[macro_use]
extern crate caper;

use caper::types::{ RenderItem, Transform };
use caper::mesh::gen_cube;

fn main() {
    // define some items to be rendered
    let mut render_items = vec![
        RenderItem {
            vertices: gen_cube(),
            shader_name: "dist",
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (-0.5, 0.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                    update_fn: Vec::new(),
                }
            ],
            active: true,
        },
    ];

    // define a vector for potential text items
    let mut text_items = Vec::new();

    game_loop! {
        // following are identities for access to the frameworks systems
        Input => input,
        Renderer => renderer,
        CamState => cam_state,
        RenderItems => render_items,
        TextItems => text_items,
        // define a block for start
        start => {
            println!("{:?}", cam_state.cam_pos);
        },
        // define block for update
        update => {
            input.handle_fp_inputs(&mut cam_state);
        },
        // block for ui rendering
        ui => {

        }
    }
}
