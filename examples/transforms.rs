extern crate caper;
extern crate time;

use caper::game::*;
use caper::imgui::Ui;
use caper::input::Key;
use caper::mesh::{gen_cube, gen_quad, gen_sphere};
use caper::types::{DefaultTag, MaterialBuilder, RenderItemBuilder, Transform, TransformBuilder};
use caper::utils::handle_fp_inputs;
use caper::utils::load_wavefront;

fn main() {
    let (mut game, event_loop) = Game::<DefaultTag>::new();

    fn sin_y(t: &mut Transform) {
        t.pos = (t.pos.0, time::precise_time_s().sin() as f32, t.pos.2);
    }

    fn circle(t: &mut Transform) {
        let update_time = time::precise_time_s();
        t.pos = (
            update_time.sin() as f32 * 3.0,
            t.pos.1,
            update_time.cos() as f32 * 3.0,
        );
    }

    fn spin(t: &mut Transform) {
        let update_time = time::precise_time_s();
        t.rot = (
            update_time.cos() as f32,
            t.rot.1,
            t.rot.2,
            update_time.sin() as f32,
        );
    }

    game.add_render_item(
        RenderItemBuilder::default()
            .name("sphere")
            .vertices(load_wavefront(include_bytes!("assets/sphere.obj")))
            .instance_transforms(vec![
                TransformBuilder::default()
                    .pos((0.0, (0.0 as f32).sin(), 0.0))
                    .rot((0f32, 0f32, 0f32, 1f32))
                    .scale((0.5f32, 0.5f32, 0.5f32))
                    .build()
                    .unwrap(),
                TransformBuilder::default()
                    .pos((0.0f32.sin(), 0.0, 0.0f32.cos()))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
    );
    game.add_render_item(
        RenderItemBuilder::default()
            .name("floor")
            .vertices(load_wavefront(include_bytes!("assets/floor.obj")))
            .material(
                MaterialBuilder::default()
                    .shader_name("height".to_string())
                    .build()
                    .unwrap(),
            )
            .instance_transforms(vec![
                Transform::default(),
                TransformBuilder::default()
                    .active(true)
                    .pos((15.0, 0.0, 0.0))
                    .rot((0f32, 0f32, 0f32, 1f32))
                    .scale((2.0f32, 2.0f32, 2.0f32))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
    );
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_quad())
            .material(
                MaterialBuilder::default()
                    .shader_name("texture".to_string())
                    .build()
                    .unwrap(),
            )
            .instance_transforms(vec![TransformBuilder::default()
                .pos((0.0, 1.0, 0.0))
                .build()
                .unwrap()])
            .build()
            .unwrap(),
    );
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_sphere())
            .material(
                MaterialBuilder::default()
                    .shader_name("texture".to_string())
                    .build()
                    .unwrap(),
            )
            .instance_transforms(vec![TransformBuilder::default()
                .pos((0.0, 3.0, 0.0))
                .build()
                .unwrap()])
            .build()
            .unwrap(),
    );
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_cube())
            .instance_transforms(vec![TransformBuilder::default()
                .pos((0.0, 8.0, 0.0))
                .build()
                .unwrap()])
            .build()
            .unwrap(),
    );

    game.renderer
        .lighting
        .add_directional_light("one".to_string(), (-0.2, 0.8, 0.1));
    game.renderer
        .lighting
        .add_directional_light("two".to_string(), (1.0, 0.0, 0.0));
    game.renderer
        .lighting
        .add_directional_light("three".to_string(), (0.0, 1.0, 0.0));

    // test getting a direcitonal light by name
    {
        let _ = game
            .renderer
            .lighting
            .get_directional_light_by_name("one")
            .unwrap();
    }

    event_loop.run(move |event, _, _control_flow| {
        // run the engine update
        game.update(
            |_: &Ui| {},
            |g: &mut Game<DefaultTag>| -> UpdateStatus {
                // update the first person inputs
                handle_fp_inputs(&mut g.input, &mut g.cams[0]);

                // temporary fix after removal of update_fn
                sin_y(
                    &mut g
                        .get_render_item_by_name("sphere")
                        .unwrap()
                        .instance_transforms[0],
                );
                circle(
                    &mut g
                        .get_render_item_by_name("sphere")
                        .unwrap()
                        .instance_transforms[0],
                );
                circle(
                    &mut g
                        .get_render_item_by_name("sphere")
                        .unwrap()
                        .instance_transforms[1],
                );
                spin(
                    &mut g
                        .get_render_item_by_name("floor")
                        .unwrap()
                        .instance_transforms[1],
                );

                // quit
                if g.input.keys_down.contains(&Key::Escape) {
                    return UpdateStatus::Finish;
                }

                UpdateStatus::Continue
            },
            event,
        );
    });
}
