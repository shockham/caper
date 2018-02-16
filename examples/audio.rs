extern crate caper;

use caper::types::{DefaultTag, RenderItemBuilder, TransformBuilder};
use caper::game::*;
use caper::mesh::gen_cube;
use caper::imgui::Ui;
use caper::input::Key;
use caper::utils::handle_fp_inputs;

fn main() {
    // crate an instance of the game struct
    let mut game = Game::<DefaultTag>::new();

    // define some items to be rendered
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_cube())
            .instance_transforms(vec![
                TransformBuilder::default()
                    .pos((-0.5, 0.0, -5.0))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
    );

    // add some audio
    game.audio.add_audio("test", "./examples/assets/test.ogg");
    game.audio
        .add_packed_audio("test_packed", include_bytes!("./assets/test.ogg").to_vec());

    // play the audio on start
    //game.audio.play("test");

    loop {
        // run the engine update
        game.update(|_: &Ui| {});

        // update the first person inputs
        handle_fp_inputs(&mut game.input, &mut game.cams[0]);

        // play audio when e is pressed
        if game.input.keys_pressed.contains(&Key::E) {
            game.audio.play("test");
        }

        // play packed audio when q is pressed
        if game.input.keys_pressed.contains(&Key::Q) {
            game.audio.play("test_packed");
        }

        // quit
        if game.input.keys_down.contains(&Key::Escape) {
            break;
        }
    }
}
