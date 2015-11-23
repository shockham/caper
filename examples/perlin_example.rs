extern crate clock_ticks;

#[macro_use]
extern crate caper;

extern crate noise;
use std::thread;
use std::time::Duration;
use caper::input::{ Input, Key };
use caper::shader::Shaders;
use caper::utils::Vertex;
use caper::renderer::{ RenderItem, Transform, Renderer, CamState, FIXED_TIME_STAMP};
use noise::{ perlin2, Seed };

fn main() {
    let mut input = Input::new();
    let renderer = Renderer::new();
    let shaders = Shaders::new(&renderer.display);

    renderer.setup();

    //cam state
    let mut cam_state = CamState {
        cam_pos: (0.0f32, 0.0, 0.0),
        cam_rot: (0.0f32, 0.0, 0.0)
    };
    
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

    // the main loop
    let mut accumulator = 0;
    let mut previous_clock = clock_ticks::precise_time_ns();
    loop {
        renderer.draw(cam_state, &render_items, &shaders);

        let now = clock_ticks::precise_time_ns();
        accumulator += now - previous_clock;
        previous_clock = now;

        while accumulator >= FIXED_TIME_STAMP {
            accumulator -= FIXED_TIME_STAMP;

            // updating and handling the inputs
            input.update_inputs(&renderer.display);
            input.handle_fp_inputs(&mut cam_state);

            { 
                // update some items
                //let update_time = clock_ticks::precise_time_s();
            }
        }

        //quit
        if input.keys_down.contains(&Key::Escape) { break; }

        let sleep_duration =
            Duration::from_millis(((FIXED_TIME_STAMP - accumulator) / 1000000) as u64);
        thread::sleep(sleep_duration);
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
