extern crate time;
extern crate caper;

use caper::types::{RenderItemBuilder, TransformBuilder};
use caper::game::*;
use caper::mesh::gen_cube;
use caper::imgui::Ui;
use caper::input::Key;
use caper::utils::handle_fp_inputs;

#[derive(Clone)]
enum Tags {
    One,
    Two,
}

impl Default for Tags {
    fn default() -> Tags { Tags::One }
}

fn main() {
    // crate an instance of the game struct
    let mut game = Game::<Tags>::new();

    // define some items to be rendered
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_cube())
            .instance_transforms(vec![
                TransformBuilder::default()
                    .pos((-1.0, 0.0, -5.0))
                    .build()
                    .unwrap(),
            ])
            .tag(Tags::One)
            .build()
            .unwrap(),
    );
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_cube())
            .instance_transforms(vec![
                TransformBuilder::default()
                    .pos((1.0, 0.0, -5.0))
                    .build()
                    .unwrap(),
            ])
            .tag(Tags::Two)
            .build()
            .unwrap(),
    );

    loop {
        // run the engine update
        game.update(|_: &Ui| {});

        // update the first person inputs
        handle_fp_inputs(&mut game.input, &mut game.cams[0]);

        let frame_time = time::precise_time_s() - game.renderer.start_time;

        // update items by tag
        for i in 0..game.render_items_len() {
            let item = game.get_render_item(i);

            match item.tag {
                Tags::One => {
                    item.instance_transforms[0].pos.1 = frame_time.sin() as f32;
                },
                Tags::Two => {
                    item.instance_transforms[0].pos.1 = frame_time.cos() as f32;
                },
            };
        }

        // quit
        if game.input.keys_down.contains(&Key::Escape) {
            break;
        }
    }
}
