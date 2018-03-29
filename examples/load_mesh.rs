extern crate caper;

use caper::types::{DefaultTag, RenderItemBuilder, TransformBuilder};
use caper::game::*;
use caper::imgui::Ui;
use caper::input::Key;
use caper::utils::load_wavefront;
use caper::utils::handle_fp_inputs;

fn main() {
    // crate an instance of the game struct
    let mut game = Game::<DefaultTag>::new();

    // define some items to be rendered
    // this example shows how to use the utiliy fn load_wavefront
    // to render your own .obj meshes made outside the engine
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(load_wavefront(include_bytes!("assets/sphere.obj")))
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
        let status = game.update(
            |_: &Ui| {},
            |g: &mut Game<DefaultTag>| -> UpdateStatus {
                // update the first person inputs
                handle_fp_inputs(&mut g.input, &mut g.cams[0]);

                // quit
                if g.input.keys_down.contains(&Key::Escape) {
                    return UpdateStatus::Finish;
                }

                UpdateStatus::Continue
            },
        );

        if let UpdateStatus::Finish = status {
            break;
        }
    }
}
