extern crate clock_ticks;

#[macro_use]
extern crate caper;

extern crate noise;

use caper::utils::Vertex;
use caper::renderer::{ RenderItem, Transform };
use noise::{ perlin2, Seed };

fn main() {



    // create a vector of render items
    let render_items = vec![
        RenderItem {
            vertices: gen_perlin_mesh(),
            shader_index: 2,
            instance_transforms: vec![
                Transform {
                    pos: (0.0, 0.0, 0.0),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32)
                }
            ]
        }
    ];

    game_loop! {
        // pass the items to be rendered
        render_items,
        // define a block for update
        { 
            // update some items
            //let update_time = clock_ticks::precise_time_s();
        }
    }
}

fn gen_perlin_mesh() -> Vec<Vertex> {
    // generate the instance positions 
    let map_size = 50f32;
    let mut vertices = Vec::new();

    for i in 0 .. 2500 {
        let pos = ((i as f32 % map_size), ((i / map_size as i32)) as f32);

        // get all four possible heights for the chunk
        let size_00 = perlin2(&Seed::new(0), &[pos.0 / 10f32, pos.1 / 10f32]).abs() * 8f32;
        let size_10 = perlin2(&Seed::new(0), &[(pos.0 + 1f32) / 10f32, pos.1 / 10f32]).abs() * 8f32;
        let size_01 = perlin2(&Seed::new(0), &[pos.0 / 10f32, (pos.1 + 1f32) / 10f32]).abs() * 8f32;
        let size_11 = perlin2(&Seed::new(0), &[(pos.0 + 1f32) / 10f32, (pos.1 + 1f32) / 10f32]).abs() * 8f32;

        // create the two tris for this chunk
        vertices.push(Vertex {
            position: [pos.0 + 1f32, size_10, pos.1],
            normal: [0f32, 0f32, 0f32],
            texture: [0f32, 0f32]
        });
        vertices.push(Vertex {
            position: [pos.0, size_00, pos.1],
            normal: [0f32, 0f32, 0f32],
            texture: [0f32, 0f32]
        });
        vertices.push(Vertex {
            position: [pos.0 + 1f32, size_11, pos.1 + 1f32],
            normal: [0f32, 0f32, 0f32],
            texture: [0f32, 0f32]
        });
        vertices.push(Vertex {
            position: [pos.0, size_00, pos.1],
            normal: [0f32, 0f32, 0f32],
            texture: [0f32, 0f32]
        });
        vertices.push(Vertex {
            position: [pos.0 , size_01, pos.1 + 1f32],
            normal: [0f32, 0f32, 0f32],
            texture: [0f32, 0f32]
        });
        vertices.push(Vertex {
            position: [pos.0 + 1f32, size_11, pos.1 + 1f32],
            normal: [0f32, 0f32, 0f32],
            texture: [0f32, 0f32]
        });
    }

    vertices
}
