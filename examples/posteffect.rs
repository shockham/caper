extern crate caper;

use caper::game::*;
use caper::imgui::Ui;
use caper::input::Key;
use caper::mesh::gen_cube;
use caper::posteffect::PostShaderOptionsBuilder;
use caper::types::{DefaultTag, RenderItemBuilder, TransformBuilder};
use caper::utils::handle_fp_inputs;

fn main() {
    // crate an instance of the game struct
    let mut game = Game::<DefaultTag>::new();

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

    // example of how to configure the default post effect shader
    game.renderer.post_effect.post_shader_options = PostShaderOptionsBuilder::default()
        .chrom_amt(1f32)
        .blur_amt(2f32)
        .blur_radius(3f32)
        .bokeh(true)
        .bokeh_focal_depth(0.45f32)
        .bokeh_focal_width(0.4f32)
        .color_offset((1f32, 0.8f32, 1f32, 1f32))
        .greyscale(true)
        .noise(0.5f32)
        .scanline(0.1f32)
        .scanline_count(100)
        .build()
        .unwrap();

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
