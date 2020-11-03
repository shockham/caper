extern crate caper;

use caper::game::*;
use caper::imgui::im_str;
use caper::imgui::Ui;
use caper::input::Key;
use caper::mesh::gen_cube;
use caper::types::{DefaultTag, RenderItemBuilder, TransformBuilder};

use std::sync::{Arc, Mutex};

fn main() {
    // crate an instance of the game struct
    let (mut game, event_loop) = Game::<DefaultTag>::new();

    // we need to show the mouse in order to interact with the ui
    game.input.hide_mouse = false;

    // define some items to be rendered
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_cube())
            .name("cube")
            .instance_transforms(vec![TransformBuilder::default()
                .pos((0.0, 0.0, -5.0))
                .build()
                .unwrap()])
            .build()
            .unwrap(),
    );

    let pos_y = Arc::new(Mutex::new(0.));

    // run the engine update
    start_loop(event_loop, move |events| {
        game.update(
            |ui: &Ui| {
                ui.drag_float(im_str!("Y Pos"), &mut pos_y.lock().unwrap())
                    .speed(0.01)
                    .build();
            },
            |g: &mut Game<DefaultTag>| -> UpdateStatus {
                g.get_render_item_by_name("cube")
                    .unwrap()
                    .instance_transforms[0]
                    .pos
                    .1 = *pos_y.lock().unwrap();

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
