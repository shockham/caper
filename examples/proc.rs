extern crate time;

#[macro_use]
extern crate caper;

use caper::utils::load_wavefront;
use caper::types::{ RenderItem, Transform };

fn main() {
    // generate the instance positions
    let transforms = (0 .. 200)
        .map(|i| {
            Transform {
                active: true,
                pos: ((i as f32 % 10f32) * 2f32, 0.0f32, (i as f32 / 10f32) * 2f32),
                rot: (0f32, 0f32, 0f32, 1f32),
                scale: (1f32, 1f32, 1f32),
            }
        })
    .collect::<Vec<_>>();

    // create a vector of render items
    let mut render_items = vec![
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/sphere.obj")),
            shader_name: "height".to_string(),
            instance_transforms: transforms,
            active: true,
        }
    ];

    let text_items = Vec::new();

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
        // define a block for update
        update => {
            // first person input
            input.handle_fp_inputs(&mut cam_state);

            // update some items
            let update_time = time::precise_time_s();

            for t in render_items[0].instance_transforms.iter_mut() {
                t.pos = (t.pos.0,
                         ((t.pos.0 / 5f32).sin() *
                          (t.pos.2 / 5f32).cos() *
                          update_time.sin() as f32) * 2f32,
                          t.pos.2);
                t.scale = (update_time.sin() as f32,
                          update_time.sin() as f32,
                          update_time.sin() as f32);
            }
    },
    ui => {

    }
}
}
