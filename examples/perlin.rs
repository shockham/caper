extern crate time;
extern crate fps_counter;
extern crate noise;

#[macro_use]
extern crate caper;

#[macro_use]
extern crate imgui;

use caper::utils::create_skydome;
use caper::types::{ RenderItem, TextItem, Transform, PhysicsType, MaterialBuilder };
use caper::mesh::{ gen_perlin_mesh, gen_sphere, get_pos_perlin, DEF_SEED_BASE };
use caper::game::Game;
use caper::input::Key;
use noise::Seed;
use fps_counter::FPSCounter;
use imgui::*;

fn main() {
    let mut game = Game::new();
    let mut fps = FPSCounter::new();

    let map_size = 100f32;
    let fixed_val = -(map_size/2f32);
    let move_speed = 0.1f32;
    let mouse_speed = 3f32;
    let sphere_pos = (8f32, 10f32);

    let mut pseu_cam_pos = (0f32, 0f32);
    let mut movement_dirty = true;
    let mut debug_mode = false;
    let mut test_check = false;

    // create a vector of render items
    game.add_render_item(
        RenderItem {
            vertices: gen_perlin_mesh(pseu_cam_pos, map_size),
            material: MaterialBuilder::default()
                .shader_name("height".to_string())
                .build()
                .unwrap(),
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (fixed_val, 0.0, fixed_val),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                }
            ],
            active: true,
            physics_type: PhysicsType::None,
        });
    game.add_render_item(create_skydome("height"));
    game.add_render_item(
        RenderItem {
            vertices: gen_sphere(),
            material: MaterialBuilder::default()
                .shader_name("line".to_string())
                .build()
                .unwrap(),
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (sphere_pos.0, 3.0, sphere_pos.1),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                }
            ],
            active: true,
            physics_type: PhysicsType::None,
        });

    game.add_text_item(
        TextItem {
            text: "test text".to_string(),
            pos: (-1.0f32, 0.5f32, 0f32),
            color: (0f32, 0f32, 0f32, 1f32),
            scale: (1f32, 1f32, 1f32),
            active: true,
        });

    game.cam_state.cam_pos.1 = 2.5f32 + get_pos_perlin(((pseu_cam_pos.0 - fixed_val),
                                                            (pseu_cam_pos.1 - fixed_val)),
                                                            &Seed::new(DEF_SEED_BASE));
    loop {
        // run the engine update
        game.update(|ui:&Ui|{
            if debug_mode {
                ui.window(im_str!("debug"))
                    .size((300.0, 200.0), ImGuiSetCond_FirstUseEver)
                    .position((0.0, 0.0), ImGuiSetCond_FirstUseEver)
                    .build(|| {
                        ui.text(im_str!("map_size: {}", map_size));
                        ui.text(im_str!("fixed_val: {}", fixed_val));
                        ui.separator();
                        ui.text(im_str!("move_speed: {}", move_speed));
                        ui.text(im_str!("mouse_speed: {}", mouse_speed));
                        ui.separator();
                        ui.text(im_str!("pseu_cam_pos: {:?}", pseu_cam_pos));
                        ui.text(im_str!("fps: {:?}", fps.tick()));
                        ui.checkbox(im_str!("test_check"), &mut test_check);
                    });
            }
        });

        if game.input.hide_mouse {
            let mv_matrix = caper::utils::build_fp_view_matrix(&game.cam_state);

            if game.input.keys_down.contains(&Key::S) {
                pseu_cam_pos.0 += mv_matrix[0][2] * move_speed;
                pseu_cam_pos.1 += mv_matrix[2][2] * move_speed;
                movement_dirty = true;
            }

            if game.input.keys_down.contains(&Key::W) {
                pseu_cam_pos.0 -= mv_matrix[0][2] * move_speed;
                pseu_cam_pos.1 -= mv_matrix[2][2] * move_speed;
                movement_dirty = true;
            }

            if game.input.keys_down.contains(&Key::D) {
                pseu_cam_pos.0 += mv_matrix[0][0] * move_speed;
                pseu_cam_pos.1 += mv_matrix[2][0] * move_speed;
                movement_dirty = true;
            }

            if game.input.keys_down.contains(&Key::A) {
                pseu_cam_pos.0 -= mv_matrix[0][0] * move_speed;
                pseu_cam_pos.1 -= mv_matrix[2][0] * move_speed;
                movement_dirty = true;
            }

            game.cam_state.cam_rot.0 += game.input.mouse_delta.1 * mouse_speed;
            game.cam_state.cam_rot.1 += game.input.mouse_delta.0 * mouse_speed;
        }

        // only regenerate the mesh if movement
        if movement_dirty {
            game.get_render_item(0).vertices = gen_perlin_mesh(pseu_cam_pos, map_size);
            game.cam_state.cam_pos.1 = 2.5f32 + get_pos_perlin(((pseu_cam_pos.0 - fixed_val),
                                                            (pseu_cam_pos.1 - fixed_val)),
                                                            &Seed::new(DEF_SEED_BASE));

            // update the sphere location
            game.get_render_item(2).instance_transforms[0].pos =
                (sphere_pos.0 - pseu_cam_pos.0, 3.0, sphere_pos.1 - pseu_cam_pos.1);
        }

        if game.input.keys_down.contains(&Key::L) { debug_mode = true; }
        if game.input.keys_down.contains(&Key::K) { debug_mode = false; }
        game.input.hide_mouse = !debug_mode;
        // quit
        if game.input.keys_down.contains(&Key::Escape) { break; }
    }
}
