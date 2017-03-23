/*!

Small game framework.

[Example](https://github.com/shockham/caper/blob/master/examples/simple.rs) of a basis for a game:

```
#[macro_use]
extern crate caper;

use caper::types::{ RenderItem, Transform, PhysicsType };
use caper::mesh::gen_cube;

fn main() {
    // define some items to be rendered
    let mut render_items = vec![
        RenderItem {
            vertices: gen_cube(),
            shader_name: String::from("dist"),
            instance_transforms: vec![
                Transform {
                    active: true,
                    pos: (-0.5, 0.0, -5.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                }
            ],
            active: true,
            physics_type: PhysicsType::None,
        },
    ];

    // define a vector for potential text items
    let text_items = Vec::new();

    game_loop! {
        // following are identities for access to the frameworks systems
        Input => input,
        Renderer => renderer,
        CamState => cam_state,
        RenderItems => render_items,
        TextItems => text_items,
        // define a block for start
        start => {
            println!("{:?}", cam_state.cam_pos);
        },
        // define block for update
        update => {
            input.handle_fp_inputs(&mut cam_state);
        },
        // block for ui rendering
        ui => {

        }
    }
}
```

*/

#![deny(missing_docs)]

#[macro_use]
pub extern crate glium;
pub extern crate imgui;
pub extern crate ncollide;
pub extern crate nphysics3d;
pub extern crate nalgebra;

extern crate glium_text;
extern crate noise;
extern crate time;
extern crate fps_counter;
extern crate bincode;
extern crate rustc_serialize;
extern crate image;
extern crate gif;

pub mod renderer;
pub mod utils;
pub mod input;
pub mod shader;
pub mod mesh;
pub mod posteffect;
pub mod types;
pub mod collision;
pub mod persist;
