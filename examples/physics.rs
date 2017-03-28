#[macro_use]
extern crate caper;
#[macro_use]
extern crate imgui;

use caper::types::{ RenderItem, Transform, PhysicsType };
use caper::mesh::gen_cube;
use imgui::*;

fn main() {
    // define some items to be rendered
    let mut render_items = vec![
        RenderItem {
            vertices: gen_cube(),
            shader_name: "dist".to_string(),
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (0f32, -5.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (20f32, 1f32, 20f32),
                }
            ],
            active: true,
            physics_type: PhysicsType::Static,
        },
        RenderItem {
            vertices: gen_cube(),
            shader_name: "height".to_string(),
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (0f32, 0.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                },
                Transform {
                    active: true,
                    pos: (0f32, 5.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 2f32, 1f32),
                },
                Transform {
                    active: true,
                    pos: (0f32, 10.0, -5.0),
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
                    pos: (-2.0, 2.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                },
                Transform {
                    active: true,
                    pos: (-2.0, 4.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                },
                Transform {
                    active: true,
                    pos: (-2.0, 6.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                }],
            active: true,
            physics_type: PhysicsType::Dynamic,
        }];

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
            //input.handle_fp_inputs(&mut cam_state);

            if input.keys_down.contains(&Key::W) {
                render_items[1].instance_transforms[0].pos.2 -= 0.1f32;
            }
            if input.keys_down.contains(&Key::S) {
                render_items[1].instance_transforms[0].pos.2 += 0.1f32;
            }
            if input.keys_down.contains(&Key::D) {
                render_items[1].instance_transforms[0].pos.0 += 0.1f32;
            }
            if input.keys_down.contains(&Key::A) {
                render_items[1].instance_transforms[0].pos.0 -= 0.1f32;
            }
            if input.keys_down.contains(&Key::Space) {
                render_items[1].instance_transforms[0].pos.1 += 0.1f32;
            }

            let player_pos = render_items[1].instance_transforms[0].pos;
            cam_state.cam_pos = 
                (player_pos.0, player_pos.1 + 1.5f32, player_pos.2 + 8f32);
        },
        // block for ui rendering
        ui => {

            ui.window(im_str!("Editor"))
                .size((500.0, 200.0), ImGuiSetCond_FirstUseEver)
                .position((0.0, 0.0), ImGuiSetCond_FirstUseEver)
                .movable(false)
                .build(|| {
                    ui.text(im_str!("{:?}", render_items[1].instance_transforms[0].pos));

                    let (mut x, mut y, mut z, mut w) =
                        (render_items[1].instance_transforms[0].rot.0.to_string(),
                        render_items[1].instance_transforms[0].rot.1.to_string(),
                        render_items[1].instance_transforms[0].rot.2.to_string(),
                        render_items[1].instance_transforms[0].rot.3.to_string());
                    x.truncate(5);
                    y.truncate(5);
                    z.truncate(5);
                    w.truncate(5);
                    ui.text(im_str!("|({},{},{},{})", x, y, z, w));
                });
        }
    }
}
