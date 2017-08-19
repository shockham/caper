extern crate caper;

use caper::types::{RenderItemBuilder, TransformBuilder, MaterialBuilder};
use caper::game::Game;
use caper::mesh::gen_cube;
use caper::imgui::Ui;
use caper::input::Key;
use caper::utils::handle_fp_inputs;

fn main() {
    // crate an instance of the game struct
    let mut game = Game::new();

    // define some items to be rendered
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_cube())
            .material(
                MaterialBuilder::default()
                    .shader_name("texture".to_string())
                    .texture_name(Some("default".to_string()))
                    .build()
                    .unwrap(),
            )
            .instance_transforms(vec![
                TransformBuilder::default()
                    .pos((-0.5, 0.0, -5.0))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
    );

    loop {
        // run the engine update
        game.update(|_: &Ui| {});

        // update the first person inputs
        handle_fp_inputs(&mut game.input, &mut game.cam_state);

        // quit
        if game.input.keys_down.contains(&Key::Escape) {
            break;
        }
    }
}
