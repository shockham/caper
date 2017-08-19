extern crate caper;

use caper::types::{RenderItemBuilder, TransformBuilder};
use caper::game::Game;
use caper::mesh::{gen_sphere, gen_sphere_segments};
use caper::imgui::Ui;
use caper::input::Key;
use caper::utils::handle_fp_inputs;

fn main() {
    // crate an instance of the game struct
    let mut game = Game::new();

    // define some items to be rendered
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_sphere())
            .instance_transforms(vec![
                TransformBuilder::default()
                    .pos((0.0, 0.0, -5.0))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
    );
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_sphere_segments(10f32, 5f32))
            .instance_transforms(vec![
                TransformBuilder::default()
                    .pos((-5.0, 0.0, -5.0))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
    );
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_sphere_segments(5f32, 10f32))
            .instance_transforms(vec![
                TransformBuilder::default()
                    .pos((5.0, 0.0, -5.0))
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
