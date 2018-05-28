/*!

Minimalist game framework.
Currently has systems for:
- Rendering ([glium](https://github.com/tomaka/glium))
- Input ([winit](https://github.com/tomaka/winit)
    via [volition](https://github.com/shockham/volition))
- Physics ([nphysics](https://github.com/sebcrozet/nphysics))
- Audio ([rodio](https://github.com/tomaka/rodio))

[Example](https://github.com/shockham/caper/blob/master/examples/simple.rs) of a basis for a game:

## Setup
### Linux
Due to the crate alsa-sys being use for linux the following packages are required:
#### Debian/Ubuntu etc
`apt install libasound2-dev pkg-config`
#### Fedora/RHEL/CentOS
`dnf install alsa-lib-devel`

## Usage
```no_run
extern crate caper;

use caper::types::{DefaultTag, RenderItemBuilder, TransformBuilder};
use caper::game::*;
use caper::mesh::gen_cube;
use caper::imgui::Ui;
use caper::input::Key;
use caper::utils::handle_fp_inputs;

fn main() {
    // crate an instance of the game struct
    let mut game = Game::<DefaultTag>::new();

    // define some items to be rendered
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_cube())
            .instance_transforms(vec![
                TransformBuilder::default()
                    .pos((-0.5, 0.0, -5.0))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
    );

    loop {
        // run the engine update
        let status = game.update(|_: &Ui| {}, |g: &mut Game<DefaultTag>| -> UpdateStatus {
            // update the first person inputs
            handle_fp_inputs(&mut g.input, &mut g.cams[0]);

            // quit
            if g.input.keys_down.contains(&Key::Escape) {
                return UpdateStatus::Finish;
            }

            UpdateStatus::Continue
        });

        if let UpdateStatus::Finish = status {
            break;
        }
    }
}
```

*/

#![deny(missing_docs)]

#[macro_use]
extern crate derive_builder;
#[macro_use]
pub extern crate glium;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

pub extern crate image;
pub extern crate imgui;
pub extern crate impose as audio;
pub extern crate nalgebra;
pub extern crate ncollide3d as ncollide;
pub extern crate nphysics3d;
pub extern crate volition as input;

extern crate bincode;
extern crate fps_counter;
extern crate gif;
extern crate glium_text_rusttype as glium_text;
extern crate imgui_glium_renderer;
extern crate noise;
extern crate rayon;
extern crate serde;
extern crate time;

/// Simple collision detection
pub mod collision;
/// Module represent another way of creating a game
pub mod game;
/// Module for procedurally generated meshes
pub mod mesh;
/// Module for saving and loading data
pub mod persist;
/// A module for rendering items
pub mod renderer;
/// All of the caper types
pub mod types;
/// Utility functions and macros
pub mod utils;

pub use renderer::lighting;
pub use renderer::posteffect;
pub use renderer::texture;
pub use renderer::shader;
