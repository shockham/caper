extern crate time;
extern crate caper;

use caper::utils::load_wavefront;
use caper::types::{RenderItemBuilder, TransformBuilder, MaterialBuilder};
use caper::game::Game;
use caper::imgui::Ui;
use caper::input::Key;
use caper::utils::handle_fp_inputs;

fn main() {
    let mut game = Game::new();

    // generate the instance positions
    let transforms = (0..200)
        .map(|i| {
            TransformBuilder::default()
                .pos((
                    (i as f32 % 10f32) * 2f32,
                    0.0f32,
                    (i as f32 / 10f32) * 2f32,
                ))
                .rot((0f32, 0f32, 0f32, 1f32))
                .scale((1f32, 1f32, 1f32))
                .build()
                .unwrap()
        })
        .collect::<Vec<_>>();

    // create a vector of render items
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(load_wavefront(include_bytes!("assets/sphere.obj")))
            .material(
                MaterialBuilder::default()
                    .shader_name("height".to_string())
                    .build()
                    .unwrap(),
            )
            .instance_transforms(transforms)
            .build()
            .unwrap(),
    );


    loop {
        // run the engine update
        game.update(|_: &Ui| {});

        // update the first person inputs
        handle_fp_inputs(&mut game.input, &mut game.cams[0]);

        // update some items
        let update_time = time::precise_time_s();

        for t in game.get_render_item(0).instance_transforms.iter_mut() {
            t.pos = (
                t.pos.0,
                ((t.pos.0 / 5f32).sin() * (t.pos.2 / 5f32).cos() * update_time.sin() as f32) *
                    2f32,
                t.pos.2,
            );
            t.scale = (
                update_time.sin() as f32,
                update_time.sin() as f32,
                update_time.sin() as f32,
            );
        }

        // quit
        if game.input.keys_down.contains(&Key::Escape) {
            break;
        }
    }
}
