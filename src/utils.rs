extern crate genmesh;
extern crate clock_ticks;
extern crate obj;

use std::ops::{Add, Mul};
use std::num::Zero;

#[macro_export]
macro_rules! game_loop {
    ( $items:ident, $update:block ) => {
        {
            use std::thread;
            use std::time::Duration;
            use caper::renderer::{ Renderer, CamState, FIXED_TIME_STAMP };
            use caper::input::Input;
            use caper::shader::Shaders;

            let input = Input::new();
            let renderer = Renderer::new();
            let shaders = Shaders::new(&renderer.display);

            renderer.setup();

            //cam state
            let mut cam_state = CamState {
                cam_pos: [ 0.0f32, 0.0, 0.0 ],
                cam_rot: [ 0.0f32, 0.0, 0.0 ]
            };

            // the main loop
            let mut accumulator = 0;
            let mut previous_clock = clock_ticks::precise_time_ns();
            loop {
                renderer.draw(cam_state, &$items, &shaders);

                let now = clock_ticks::precise_time_ns();
                accumulator += now - previous_clock;
                previous_clock = now;

                while accumulator >= FIXED_TIME_STAMP {
                    accumulator -= FIXED_TIME_STAMP;

                    // keeping the camera on a single plane
                    //cam_state.cam_pos[1] = -1.0f32;

                    // updating and handling the inputs
                    input.update_inputs(&renderer.display);
                    input.handle_inputs(&mut cam_state);

                    $update
                }

                //quit
                if input.btns_down[8].get() { break; }

                let sleep_duration =
                    Duration::from_millis(((FIXED_TIME_STAMP - accumulator) / 1000000) as u64);
                thread::sleep(sleep_duration);
            }
        }
    };
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    texture: [f32; 2],
}

/// Returns a Vec<Vertex> that should be converted to buffer and rendered as `TrianglesList`.
pub fn load_wavefront( data: &[u8]) -> Vec<Vertex> {

    implement_vertex!(Vertex, position, normal, texture);

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

    //glium::vertex::VertexBuffer::new(display, vertex_data).into_vertex_buffer_any()
    vertex_data
}

/// Returns the dot product of two vectors
pub fn dotp<T>(this: &[T], other: &[T]) -> T where T:Add<T, Output=T> + Mul<T, Output=T> + Zero + Copy {
    assert!(this.len() == other.len(), "The dimensions must be equal");

    let zero : T = Zero::zero();
    this.iter().zip(other.iter())
        .map(|(&a, &b)| a * b)
        .fold(zero, |sum, n| sum + n)
}
