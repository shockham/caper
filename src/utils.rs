extern crate genmesh;
extern crate clock_ticks;
extern crate obj;

use std::ops::{Add, Mul};
use std::num::Zero;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    texture: [f32; 2],
}

/// Returns a vertex buffer that should be rendered as `TrianglesList`.
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