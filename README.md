Caper
========
[![crates.io version](https://img.shields.io/crates/v/caper.svg)](https://crates.io/crates/caper)

Small game framework using [rust](https://www.rust-lang.org/) and [glium](https://github.com/tomaka/glium).

[**Documentation**](https://shockham.github.io/caper/caper/)

[Example](https://github.com/shockham/caper/blob/master/examples/simple.rs) of a basis for a game:
```rust
extern crate caper;

use caper::types::{ RenderItem, Transform, PhysicsType, MaterialBuilder };
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
            material: MaterialBuilder::default().build().unwrap(),
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

Check out the [examples](https://github.com/shockham/caper/tree/master/examples) and run with:
```
cargo run --example transforms
```

[License](https://github.com/shockham/caper/blob/master/LICENSE.md)
