extern crate caper;
#[macro_use]
extern crate imgui;

use caper::types::{ RenderItem, Transform, PhysicsType };
use caper::game::Game;
use caper::mesh::gen_cube;
use caper::imgui::Ui;
use caper::input::Key;
use imgui::*;

fn main() {
    let mut game = Game::new();
    // define some items to be rendered
    game.add_render_item(
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
        });
    game.add_render_item(
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
                }
            ],
            active: true,
            physics_type: PhysicsType::Dynamic,
        });

    loop {
        // updating the game & ui rendering
        game.update(|ui:&Ui| {
            ui.window(im_str!("Editor"))
                .size((500.0, 200.0), ImGuiSetCond_FirstUseEver)
                .position((0.0, 0.0), ImGuiSetCond_FirstUseEver)
                .movable(false)
                .build(|| {
                    /*let render_item = game.get_render_item(1);

                    ui.text(im_str!("{:?}", render_item.instance_transforms[0].pos));

                    let (mut x, mut y, mut z, mut w) =
                        (render_item.instance_transforms[0].rot.0.to_string(),
                        render_item.instance_transforms[0].rot.1.to_string(),
                        render_item.instance_transforms[0].rot.2.to_string(),
                        render_item.instance_transforms[0].rot.3.to_string());
                    x.truncate(5);
                    y.truncate(5);
                    z.truncate(5);
                    w.truncate(5);
                    ui.text(im_str!("|({},{},{},{})", x, y, z, w));*/
                });
        });

        //game.input.handle_fp_inputs(&mut game.cam_state);

        if game.input.keys_down.contains(&Key::W) {
            game.get_render_item(1).instance_transforms[0].pos.2 -= 0.1f32;
        }
        if game.input.keys_down.contains(&Key::S) {
            game.get_render_item(1).instance_transforms[0].pos.2 += 0.1f32;
        }
        if game.input.keys_down.contains(&Key::D) {
            game.get_render_item(1).instance_transforms[0].pos.0 += 0.1f32;
        }
        if game.input.keys_down.contains(&Key::A) {
            game.get_render_item(1).instance_transforms[0].pos.0 -= 0.1f32;
        }
        if game.input.keys_down.contains(&Key::Space) {
            game.get_render_item(1).instance_transforms[0].pos.1 += 0.1f32;
        }

        let player_pos = game.get_render_item(1).instance_transforms[0].pos;
        game.cam_state.cam_pos = (player_pos.0, player_pos.1 + 1.5f32, player_pos.2 + 8f32);

        // quit
        if game.input.keys_down.contains(&Key::Escape) { break; }
    }
}
