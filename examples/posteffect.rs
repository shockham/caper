extern crate caper;

use caper::types::{ RenderItemBuilder, TransformBuilder };
use caper::game::Game;
use caper::mesh::gen_cube;
use caper::imgui::Ui;
use caper::input::Key;
use caper::posteffect::PostShaderOptionsBuilder;

fn main() {
    // crate an instance of the game struct
    let mut game = Game::new();

    // define some items to be rendered
    game.add_render_item(
        RenderItemBuilder::default()
        .vertices(gen_cube())
        .instance_transforms(vec![
            TransformBuilder::default()
                .pos((-0.5, 0.0, -5.0))
                .build()
                .unwrap()
        ])
        .build()
        .unwrap());

    // example of how to configure the default post effect shader
    game.renderer.post_effect.post_shader_options = PostShaderOptionsBuilder::default()
        .chrom_amt(1f32)
        .blur_amt(2f32)
        .blur_radius(3f32)
        .bokeh(true)
        .bokeh_focal_depth(0.45f32)
        .bokeh_focal_width(0.4f32)
        .color_offset((1f32, 0.8f32, 1f32, 1f32))
        .build()
        .unwrap();

    loop {
        // run the engine update
        game.update(|_:&Ui|{ });

        // update the first person inputs
        game.input.handle_fp_inputs(&mut game.cam_state);

        // quit
        if game.input.keys_down.contains(&Key::Escape) { break; }
    }
}
