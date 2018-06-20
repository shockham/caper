extern crate caper;
extern crate time;
extern crate rayon;

use caper::game::*;
use caper::imgui::Ui;
use caper::input::Key;
use caper::mesh::gen_cube;
use caper::types::{RenderItemBuilder, TransformBuilder};
use caper::utils::handle_fp_inputs;

use rayon::prelude::*;

#[derive(Clone)]
enum Tags {
    One,
    Two,
}

impl Default for Tags {
    fn default() -> Tags {
        Tags::One
    }
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
        let status = game.update(
            |_: &Ui| {},
            |g: &mut Game<Tags>| -> UpdateStatus {
                // update the first person inputs
                handle_fp_inputs(&mut g.input, &mut g.cams[0]);

                let frame_time = time::precise_time_s() - g.renderer.start_time;

                // update items by tag
                g.render_items_iter_mut().for_each(|item| {
                    match item.tag {
                        Tags::One => {
                            item.instance_transforms[0].pos.1 = frame_time.sin() as f32;
                        }
                        Tags::Two => {
                            item.instance_transforms[0].pos.1 = frame_time.cos() as f32;
                        }
                    };
                });

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
