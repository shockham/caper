extern crate caper;

use std::thread::sleep;
use std::time::Duration;

use caper::game::*;
use caper::imgui::Ui;
use caper::input::Key;
use caper::mesh::gen_cube;
use caper::types::{DefaultTag, RenderItemBuilder, TransformBuilder};
use caper::utils::handle_fp_inputs;
use caper::audio::rodio;

fn main() {
    // crate an instance of the game struct
    let (mut game, event_loop) = Game::<DefaultTag>::new();

    // define some items to be rendered
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_cube())
            .instance_transforms(vec![TransformBuilder::default()
                .pos((-0.5, 0.0, -5.0))
                .build()
                .unwrap()])
            .build()
            .unwrap(),
    );

    start_loop(event_loop, move |events| {
        // run the engine update
        game.update(
            |_: &Ui| {},
            |g: &mut Game<DefaultTag>| -> UpdateStatus {
                // update the first person inputs
                handle_fp_inputs(&mut g.input, &mut g.cams[0]);

                // play audio when e is pressed
                if g.input.keys_pressed.contains(&Key::E) {
                    let source = rodio::source::SineWave::new(440);
                    let mut sound = g.audio.play_at(source, [50.0, 1.0, 0.0]);

                    // move sound from right to left
                    sound.set_velocity([-10.0, 0.0, 0.0]);
                    for i in 0..1000 {
                        sound.adjust_position([50.0 - i as f32 / 10.0, 1.0, 0.0]);
                        sleep(Duration::from_millis(10));
                    }
                    sound.set_velocity([0.0, 0.0, 0.0]);
                }

                // quit
                if g.input.keys_down.contains(&Key::Escape) {
                    return UpdateStatus::Finish;
                }

                UpdateStatus::Continue
            },
            events,
        )
    });
}
