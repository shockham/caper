extern crate time;
extern crate caper;
extern crate imgui;

extern crate noise;

use caper::utils::load_wavefront;
use caper::types::{ RenderItemBuilder, TransformBuilder };
use caper::game::Game;
use caper::input::Key;
use caper::imgui::Ui;
use noise::{ perlin2, Seed };

fn main() {
    // create an instance of Game
    let mut game = Game::new();

    // generate the instance positions
    let map_size = 50f32;
    let transforms = (0 .. 2500)
        .map(|i| {
            let pos = ((i as f32 % map_size) * 2f32, ((i / map_size as i32) * 2) as f32);
            let size = perlin2(&Seed::new(0), &[pos.0 / 10f32, pos.1 / 10f32]).abs() * 8f32;
            TransformBuilder::default()
                .pos((pos.0 * 5f32, size, pos.1 * 5f32))
                .scale((4.2f32, size, 4.2f32))
                .build()
                .unwrap()
        })
    .collect::<Vec<_>>();

    // add a render item to the game
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(load_wavefront(include_bytes!("assets/cube.obj")))
            .instance_transforms(transforms)
            .build()
            .unwrap());

    loop {
        // run the engine update
        game.update(|_:&Ui|{ });

        // update the first person inputs
        game.input.handle_fp_inputs(&mut game.cam_state);

        // quit
        if game.input.keys_down.contains(&Key::Escape) { break; }
    }
}
