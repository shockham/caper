/*!

Small game framework.
Currently has systems for:
- Rendering
- Input
- Physics (using [nphysics](https://github.com/sebcrozet/nphysics))

Still to be added:
- Audio

[Example](https://github.com/shockham/caper/blob/master/examples/simple.rs) of a basis for a game:

```
extern crate caper;

use caper::types::{ RenderItemBuilder, TransformBuilder };
use caper::game::Game;
use caper::mesh::gen_cube;
use caper::imgui::Ui;
use caper::input::Key;

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

    loop {
        // run the engine update
        game.update(|_:&Ui|{ });

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
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate serde_derive;

pub extern crate imgui;
pub extern crate ncollide;
pub extern crate nphysics3d;
pub extern crate nalgebra;
pub extern crate image;

extern crate glium_text_rusttype as glium_text;
extern crate noise;
extern crate time;
extern crate fps_counter;
extern crate bincode;
extern crate serde;
extern crate gif;
extern crate rodio;

/// Module for utility functions for textures
#[macro_use]
pub mod texture;
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
/// Module for the lighting system
pub mod lighting;
/// Module for the audio system
pub mod audio;
