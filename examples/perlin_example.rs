extern crate time;

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

    let map_size = 20f32;
    let fixed_val = -(map_size/2f32);
    let move_speed = 0.01f32;

    //cam state
    let mut cam_state = CamState {
        cam_pos: (fixed_val, -1f32, fixed_val),
        cam_rot: (0.0f32, 0.0, 0.0)
    };
    
    let mut pseu_cam_pos = (0f32, 0f32);
    
    // create a vector of render items
    let mut render_items = vec![
        RenderItem {
            vertices: gen_perlin_mesh(pseu_cam_pos, map_size),
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
    let mut previous_clock = time::precise_time_ns();
    loop {
        renderer.draw(cam_state, &render_items, &shaders);

        let now = time::precise_time_ns();
        accumulator += now - previous_clock;
        previous_clock = now;

        while accumulator >= FIXED_TIME_STAMP {
            accumulator -= FIXED_TIME_STAMP;

            // updating and handling the inputs
            input.update_inputs(&renderer.display);
            input.handle_fp_inputs(&mut cam_state);
            cam_state.cam_pos = (fixed_val, -1f32, fixed_val);

            { 
                if input.keys_down.contains(&Key::W) {
                    pseu_cam_pos.0 += move_speed; 
                }
                if input.keys_down.contains(&Key::S) {
                    pseu_cam_pos.0 -= move_speed; 
                }
                if input.keys_down.contains(&Key::D) {
                    pseu_cam_pos.1 += move_speed; 
                }
                if input.keys_down.contains(&Key::A) {
                    pseu_cam_pos.1 -= move_speed; 
                }
            }
        }

        render_items[0].vertices = gen_perlin_mesh(pseu_cam_pos, map_size);
        cam_state.cam_pos.1 = -2f32 - perlin2(&Seed::new(0),
            &[(pseu_cam_pos.0 - fixed_val) / 10f32, (pseu_cam_pos.1 - fixed_val) / 10f32]).abs() * 8f32;

        //quit
        if input.keys_down.contains(&Key::Escape) { break; }

        let sleep_duration =
            Duration::from_millis(((FIXED_TIME_STAMP - accumulator) / 1000000) as u64);
        thread::sleep(sleep_duration);
    }
}

fn gen_perlin_mesh(pseu_pos: (f32, f32), map_size: f32) -> Vec<Vertex> {
    // generate the instance positions 
    let mut vertices = Vec::new();

    let point_total = (map_size * map_size) as i32;

    for i in 0 .. point_total {
        let pos = ((i as f32 % map_size), (i / map_size as i32) as f32);
        let seed = Seed::new(0);

        let p_pos = (pos.0 + pseu_pos.0, pos.1 + pseu_pos.1);
        // get all four possible heights for the chunk
        let size_00 = perlin2(&seed, &[p_pos.0 / 10f32, p_pos.1 / 10f32]).abs() * 8f32;
        let size_10 = perlin2(&seed, &[(p_pos.0 + 1f32) / 10f32, p_pos.1 / 10f32]).abs() * 8f32;
        let size_01 = perlin2(&seed, &[p_pos.0 / 10f32, (p_pos.1 + 1f32) / 10f32]).abs() * 8f32;
        let size_11 = perlin2(&seed, 
                              &[(p_pos.0 + 1f32) / 10f32, (p_pos.1 + 1f32) / 10f32]).abs() * 8f32;

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
