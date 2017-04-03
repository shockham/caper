extern crate time;
extern crate caper;
extern crate imgui;

use caper::utils::load_wavefront;
use caper::types::{ RenderItem, Transform, PhysicsType, MaterialBuilder };
use caper::mesh::{ gen_quad, gen_sphere, gen_cube };
use caper::game::Game;
use caper::imgui::Ui;
use caper::input::Key;

fn main() {
    let mut game = Game::new();

    fn sin_y (t:&mut Transform) {
        t.pos = (t.pos.0, time::precise_time_s().sin() as f32, t.pos.2);
    }

    fn circle (t:&mut Transform) {
        let update_time = time::precise_time_s();
        t.pos = (update_time.sin() as f32 * 3.0, t.pos.1, update_time.cos() as f32 * 3.0);
    }

    fn spin (t:&mut Transform) {
        let update_time = time::precise_time_s();
        t.rot = (update_time.cos() as f32, t.rot.1, t.rot.2, update_time.sin() as f32);
    }

    game.add_render_item(
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/sphere.obj")),
            material: MaterialBuilder::default().build().unwrap(),
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (0.0, (0.0 as f32).sin(), 0.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (0.5f32, 0.5f32, 0.5f32),
                },
                Transform {
                    active: true,
                    pos: (0.0f32.sin(), 0.0, 0.0f32.cos()),
                    rot: (0f32, 0f32, 0f32, 0f32),
                    scale: (1f32, 1f32, 1f32),
                }
            ],
            active: true,
            physics_type: PhysicsType::None,
        });
    game.add_render_item(
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/floor.obj")),
            material: MaterialBuilder::default()
                .shader_name("height".to_string())
                .build()
                .unwrap(),
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (0.0, 0.0, 0.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                },
                Transform {
                    active: true,
                    pos: (15.0, 0.0, 0.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (2.0f32, 2.0f32, 2.0f32),
                }
            ],
            active: true,
            physics_type: PhysicsType::None,
        });
    game.add_render_item(
        RenderItem {
            vertices: gen_quad(),
            material: MaterialBuilder::default()
                .shader_name("texture".to_string())
                .build()
                .unwrap(),
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (0.0, 1.0, 0.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                }
            ],
            active: true,
            physics_type: PhysicsType::None,
        });
    game.add_render_item(
        RenderItem {
            vertices: gen_sphere(),
            material: MaterialBuilder::default()
                .shader_name("texture".to_string())
                .build()
                .unwrap(),
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (0.0, 3.0, 0.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                }
            ],
            active: true,
            physics_type: PhysicsType::None,
        });
    game.add_render_item(
        RenderItem {
            vertices: gen_cube(),
            material: MaterialBuilder::default().build().unwrap(),
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (0.0, 8.0, 0.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                }
            ],
            active: true,
            physics_type: PhysicsType::None,
        });

    loop {
        // run the engine update
        game.update(|_:&Ui|{ });

        // first person input
        game.input.handle_fp_inputs(&mut game.cam_state);

        // temporary fix after removal of update_fn
        sin_y(&mut game.get_render_item(0).instance_transforms[0]);
        circle(&mut game.get_render_item(0).instance_transforms[0]);
        circle(&mut game.get_render_item(0).instance_transforms[1]);
        spin(&mut game.get_render_item(1).instance_transforms[1]);
        
        // quit
        if game.input.keys_down.contains(&Key::Escape) { break; }
    }
}
