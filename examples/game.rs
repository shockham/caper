
extern crate caper;

use caper::types::{ RenderItem, Transform, PhysicsType };
use caper::game::Game;
use caper::mesh::gen_cube;
use caper::imgui::Ui;

fn main() {
    // crate an instance of the game struct
    let mut game = Game::new();

    // define some items to be rendered
    game.render_items = vec![
        RenderItem {
            vertices: gen_cube(),
            shader_name: "dist".to_string(),
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (-0.5, 0.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                }
            ],
            active: true,
            physics_type: PhysicsType::None,
        },
    ];

    game.init();
    game.start(
        // update function
        |game:&Game| {
            //game.input.handle_fp_inputs(&mut game.cam_state);
        },
        // function for ui rendering
        |ui:&Ui|{

        });
}
