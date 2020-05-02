extern crate caper;
extern crate imgui;

use caper::game::*;
use caper::imgui::Ui;
use caper::input::Key;
use caper::mesh::gen_cube;
use caper::types::{DefaultTag, MaterialBuilder, PhysicsType, RenderItemBuilder, TransformBuilder};
use imgui::*;

fn main() {
    let (mut game, event_loop) = Game::<DefaultTag>::new();

    // define some items to be rendered
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_cube())
            .instance_transforms(vec![TransformBuilder::default()
                .pos((0f32, -5.0, -5.0))
                .rot((0f32, 0f32, 0f32, 1f32))
                .scale((20f32, 1f32, 20f32))
                .build()
                .unwrap()])
            .physics_type(PhysicsType::Static)
            .build()
            .unwrap(),
    );
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_cube())
            .material(
                MaterialBuilder::default()
                    .shader_name("height")
                    .build()
                    .unwrap(),
            )
            .instance_transforms(vec![
                TransformBuilder::default()
                    .pos((0f32, 0.0, -5.0))
                    .build()
                    .unwrap(),
                TransformBuilder::default()
                    .pos((0f32, 5.0, -5.0))
                    .scale((1f32, 2f32, 1f32))
                    .build()
                    .unwrap(),
                TransformBuilder::default()
                    .pos((0f32, 10.0, -5.0))
                    .scale((1f32, 2f32, 1f32))
                    .build()
                    .unwrap(),
                TransformBuilder::default()
                    .pos((2.0, 2.0, -5.0))
                    .scale((1f32, 2f32, 1f32))
                    .build()
                    .unwrap(),
                TransformBuilder::default()
                    .pos((2.0, 6.0, -5.0))
                    .build()
                    .unwrap(),
                TransformBuilder::default()
                    .pos((2.0, 10.0, -5.0))
                    .scale((1f32, 2f32, 1f32))
                    .build()
                    .unwrap(),
                TransformBuilder::default()
                    .pos((-2.0, 2.0, -5.0))
                    .build()
                    .unwrap(),
                TransformBuilder::default()
                    .pos((-2.0, 4.0, -5.0))
                    .build()
                    .unwrap(),
                TransformBuilder::default()
                    .pos((-2.0, 6.0, -5.0))
                    .build()
                    .unwrap(),
            ])
            .physics_type(PhysicsType::Dynamic)
            .build()
            .unwrap(),
    );

    start_loop(event_loop, move |events| {
        // clone of the RenderItem for access in the ui rendering
        let debug_render_item = game.get_render_item(1).clone();
        // updating the game & ui rendering
        game.update(
            |ui: &Ui| {
                Window::new(im_str!("Editor"))
                    .size([500f32, 200f32], Condition::FirstUseEver)
                    .position([0f32, 0f32], Condition::FirstUseEver)
                    .movable(false)
                    .build(&ui, || {
                        ui.text(im_str!(
                            "{:?}",
                            debug_render_item.instance_transforms[0].pos
                        ));

                        let (mut x, mut y, mut z, mut w) = (
                            debug_render_item.instance_transforms[0].rot.0.to_string(),
                            debug_render_item.instance_transforms[0].rot.1.to_string(),
                            debug_render_item.instance_transforms[0].rot.2.to_string(),
                            debug_render_item.instance_transforms[0].rot.3.to_string(),
                        );
                        x.truncate(5);
                        y.truncate(5);
                        z.truncate(5);
                        w.truncate(5);
                        ui.text(im_str!("|({},{},{},{})", x, y, z, w));
                    });
            },
            |g: &mut Game<DefaultTag>| -> UpdateStatus {
                if g.input.keys_down.contains(&Key::W) {
                    g.get_render_item(1).instance_transforms[0].pos.2 -= 0.1f32;
                }
                if g.input.keys_down.contains(&Key::S) {
                    g.get_render_item(1).instance_transforms[0].pos.2 += 0.1f32;
                }
                if g.input.keys_down.contains(&Key::D) {
                    g.get_render_item(1).instance_transforms[0].pos.0 += 0.1f32;
                }
                if g.input.keys_down.contains(&Key::A) {
                    g.get_render_item(1).instance_transforms[0].pos.0 -= 0.1f32;
                }
                if g.input.keys_down.contains(&Key::Space) {
                    g.get_render_item(1).instance_transforms[0].pos.1 += 0.1f32;
                }

                let player_pos = g.get_render_item(1).instance_transforms[0].pos;
                g.cams[0].pos = (player_pos.0, player_pos.1 + 1.5f32, player_pos.2 + 8f32);

                // quit
                if g.input.keys_down.contains(&Key::Escape) {
                    return UpdateStatus::Finish;
                }

                UpdateStatus::Continue
            },
            events,
        );

        UpdateStatus::Continue
    });
}
