extern crate time;

#[macro_use]
extern crate caper;

#[macro_use]
extern crate imgui;

use caper::utils::load_wavefront;
use caper::types::{ RenderItem, Transform };
use caper::mesh::{ gen_quad, gen_sphere, gen_cube };
use imgui::*;

fn main() {

    fn sin_y (t:&mut Transform) {
        t.pos = (t.pos.0, time::precise_time_s().sin() as f32, t.pos.2);
    }

    fn circle (t:&mut Transform) {
        let update_time = time::precise_time_s();
        t.pos = (update_time.sin() as f32 * 3.0, t.pos.1, update_time.cos() as f32 * 3.0);
    }

    fn spin (t:&mut Transform) {
        let update_time = time::precise_time_s();
        t.rot = (update_time.cos() as f32, t.rot.1, t.rot.2, update_time.sin() as f32);
    }

    let mut render_items = vec![
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/sphere.obj")),
            shader_name: "dist",
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (0.0, (0.0 as f32).sin(), 0.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (0.5f32, 0.5f32, 0.5f32),
                    update_fn: vec![sin_y, circle],
                },
                Transform {
                    active: true,
                    pos: (0.0f32.sin(), 0.0, 0.0f32.cos()),
                    rot: (0f32, 0f32, 0f32, 0f32),
                    scale: (1f32, 1f32, 1f32),
                    update_fn: vec![circle],
                }
            ],
            active: true,
        },
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/floor.obj")),
            shader_name: "height",
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (0.0, 0.0, 0.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                    update_fn: Vec::new(),
                },
                Transform {
                    active: true,
                    pos: (15.0, 0.0, 0.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (2.0f32, 2.0f32, 2.0f32),
                    update_fn: vec![spin],
                }
            ],
            active: true,
        },
        RenderItem {
            vertices: gen_quad(),
            shader_name: "dist",
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (0.0, 1.0, 0.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                    update_fn: Vec::new(),
                }
            ],
            active: true,
        },
        RenderItem {
            vertices: gen_sphere(),
            shader_name: "dist",
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (0.0, 3.0, 0.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                    update_fn: Vec::new(),
                }
            ],
            active: true,
        },
        RenderItem {
            vertices: gen_cube(),
            shader_name: "dist",
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (0.0, 8.0, 0.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                    update_fn: Vec::new(),
                }
            ],
            active: true,
        },
    ];

    let mut text_items = Vec::new();

    game_loop! {
        Input => input,
        Renderer => renderer,
        CamState => cam_state,
        RenderItems => render_items,
        TextItems => text_items,
        // define a block for start
        start => {
            // yay start code
            println!("{:?}", cam_state.cam_pos);
        },
        // define block for update
        update => {
            // first person input
            input.handle_fp_inputs(&mut cam_state);
        },
        ui => {

        }
    }
}
