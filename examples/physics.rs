#[macro_use]
extern crate caper;

use caper::types::{ RenderItem, Transform, PhysicsType };
use caper::mesh::gen_cube;

fn main() {
    // define some items to be rendered
    let mut render_items = vec![
        RenderItem {
            vertices: gen_cube(),
            shader_name: "dist".to_string(),
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (-0.5, -5.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (10f32, 1f32, 10f32),
                }
            ],
            active: true,
            physics_type: PhysicsType::Static,
        },
        RenderItem {
            vertices: gen_cube(),
            shader_name: "dist".to_string(),
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (-0.5, 0.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                },
                Transform {
                    active: true,
                    pos: (-0.5, 5.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 2f32, 1f32),
                },
                Transform {
                    active: true,
                    pos: (-0.5, 10.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 2f32, 1f32),
                },
                Transform {
                    active: true,
                    pos: (2.0, 2.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 2f32, 1f32),
                },
                Transform {
                    active: true,
                    pos: (2.0, 6.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                },
                Transform {
                    active: true,
                    pos: (2.0, 10.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 2f32, 1f32),
                },
                Transform {
                    active: true,
                    pos: (-3.0, 2.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                },
                Transform {
                    active: true,
                    pos: (-3.0, 4.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                },
                Transform {
                    active: true,
                    pos: (-3.0, 6.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                },
            ],
            active: true,
            physics_type: PhysicsType::Dynamic,
        },
    ];

    // define a vector for potential text items
    let text_items = Vec::new();

    game_loop! {
        // following are identities for access to the frameworks systems
        Input => input,
        Renderer => renderer,
        CamState => cam_state,
        RenderItems => render_items,
        TextItems => text_items,
        // define a block for start
        start => {

        },
        // define block for update
        update => {
            input.handle_fp_inputs(&mut cam_state);

            if input.keys_down.contains(&Key::I) {
                render_items[1].instance_transforms[0].pos.2 -= 0.1f32;
            }
            if input.keys_down.contains(&Key::K) {
                render_items[1].instance_transforms[0].pos.2 += 0.1f32;
            }
            if input.keys_down.contains(&Key::L) {
                render_items[1].instance_transforms[0].pos.0 += 0.1f32;
            }
            if input.keys_down.contains(&Key::J) {
                render_items[1].instance_transforms[0].pos.0 -= 0.1f32;
            }
            if input.keys_down.contains(&Key::Space) {
                render_items[1].instance_transforms[0].pos.1 += 0.1f32;
            }
        },
        // block for ui rendering
        ui => {

        }
    }
}
