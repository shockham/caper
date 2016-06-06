extern crate genmesh;
extern crate time;
extern crate obj;

use std::ops::{Add, Mul};
use std::num::Zero;
use types::{ RenderItem, Transform, Vertex };

/// quick macro to use in the examples for easily defining all the modules and game loop
#[macro_export]
macro_rules! game_loop {
    ( $input:ident, $renderer:ident, $shaders:ident,
      $cam_state:ident, $render_items:ident, $text_items:ident,
      start => $start:block,
      update => $update:block,
      $ui:ident => $ui_update:block) => {
        {
            use caper::renderer::Renderer;
            use caper::types::{ CamState, Entity };
            use caper::input::{ Input, Key };
            use caper::imgui::Ui;

            let mut $input = Input::new();
            let mut $renderer = Renderer::new("Caper".to_string());

            //cam state
            let mut $cam_state = CamState {
                cam_pos: (0.0f32, 0.0, 0.0),
                cam_rot: (0.0f32, 0.0, 0.0)
            };

            $start;

            let render_imgui = |$ui: &Ui| $ui_update;

            // the main loop
            loop {
                //quit
                if $input.keys_down.contains(&Key::Escape) { break; }

                $renderer.draw(&$cam_state, &$render_items, &$text_items, &render_imgui);

                // updating and handling the inputs
                $input.update_inputs(&$renderer.display);

                for i in 0..$render_items.len() {
                    for t in 0..$render_items[i].instance_transforms.len() {
                        $render_items[i].instance_transforms[t].update();
                    }
                }

                for i in 0..$text_items.len() {
                    $text_items[i].update();
                }

                $update

            }
        }
    };
}


/// Returns a Vec<Vertex> that should be converted to buffer and rendered as `TrianglesList`.
pub fn load_wavefront( data: &[u8]) -> Vec<Vertex> {
    let mut data = ::std::io::BufReader::new(data);
    let data = obj::Obj::load(&mut data);

    let mut vertex_data = Vec::new();

    for shape in data.object_iter().next().unwrap().group_iter().flat_map(|g| g.indices().iter()) {
        match shape {
            &genmesh::Polygon::PolyTri(genmesh::Triangle { x: v1, y: v2, z: v3 }) => {
                for v in [v1, v2, v3].iter() {
                    let position = data.position()[v.0];
                    let texture = v.1.map(|index| data.texture()[index]);
                    let normal = v.2.map(|index| data.normal()[index]);

                    let texture = texture.unwrap_or([0.0, 0.0]);
                    let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                    vertex_data.push(Vertex {
                        position: position,
                        normal: normal,
                        texture: texture,
                    })
                }
            },
            _ => unimplemented!()
        }
    }

    vertex_data
}


/// Returns a RenderItem for the skydome
pub fn create_skydome() -> RenderItem {
    RenderItem {
        vertices: load_wavefront(include_bytes!("./resources/skydome.obj")),
        shader_name: "height",
        instance_transforms: vec![
            Transform {
                pos: (0.0, 0.0, 0.0),
                rot: (0f32, 0f32, 0f32, 1f32),
                scale: (100f32, 100f32, 100f32),
                update_fn: Vec::new(),
            }
        ]
    }
}

/// Returns the dot product of two vectors
pub fn dotp<T>(this: &[T], other: &[T]) -> T where T:Add<T, Output=T> + Mul<T, Output=T> + Zero + Copy {
    assert!(this.len() == other.len(), "The dimensions must be equal");

    let zero : T = Zero::zero();
    this.iter().zip(other.iter())
        .map(|(&a, &b)| a * b)
        .fold(zero, |sum, n| sum + n)
}

/// returns the cross product of two vectors
pub fn crossp(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [(a[1] * b[2]) - (a[2] * b[1]), (a[2] * b[0]) - (a[0] * b[2]), (a[0] * b[1]) - (a[1] * b[0])]
}

/// returns the resultant vector of a - b
pub fn sub_vec3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// returns the normal calculated from the three vectors supplied
pub fn calc_normal(p0: [f32; 3], p1: [f32; 3], p2: [f32; 3]) -> [f32; 3] {
    let a = sub_vec3(p1, p0);
    let b = sub_vec3(p2, p0);

    crossp(a, b)
}
