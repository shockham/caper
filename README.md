Caper
========
[![crates.io version](https://img.shields.io/crates/v/caper.svg)](https://crates.io/crates/caper)

Small game framework using [rust](https://www.rust-lang.org/) and [glium](https://github.com/tomaka/glium).
Currently has systems for:
- Rendering
- Input
- Physics (using [nphysics](https://github.com/sebcrozet/nphysics))

Still to be added:
- Audio

[**Documentation**](https://shockham.github.io/caper/caper/)

[Example](https://github.com/shockham/caper/blob/master/examples/simple.rs) of a basis for a game:
```rust
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

Check out the [examples](https://github.com/shockham/caper/tree/master/examples) and run with:
```
cargo run --example transforms
```

[License](https://github.com/shockham/caper/blob/master/LICENSE.md)
