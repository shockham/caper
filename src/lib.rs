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

Another [example](https://github.com/shockham/caper/blob/master/examples/game.rs) using the Game struct:


```
extern crate caper;

use caper::types::{ RenderItem, Transform, PhysicsType };
use caper::game::Game;
use caper::mesh::gen_cube;
use caper::imgui::Ui;
use caper::input::Key;

fn main() {
    // crate an instance of the game struct
    let mut game = Game::new();

    // define some items to be rendered
    game.add_render_item(
        RenderItem {
            vertices: gen_cube(),
            shader_name: "dist".to_string(),
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
        });

    loop {
        // run the engine update
        game.update(|ui:&Ui|{ });

        // update the first person inputs
        game.input.handle_fp_inputs(&mut game.cam_state);

        // quit
        if game.input.keys_down.contains(&Key::Escape) { break; }
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

/// A module for rendering items
pub mod renderer;
/// Utility functions and macros
pub mod utils;
/// Module for input handing
pub mod input;
/// Module for dealing with shaders
pub mod shader;
/// Module for procedurally generated meshes
pub mod mesh;
/// Rendering post processing effects
pub mod posteffect;
/// All of the caper types
pub mod types;
/// Simple collision detection
pub mod collision;
/// Module for saving and loading data
pub mod persist;
/// Module represent another way of creating a game
pub mod game;
