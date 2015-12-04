extern crate time;
extern crate fps_counter;
extern crate noise;

#[macro_use]
extern crate caper;

use std::thread;
use std::time::Duration;
use caper::input::{ Input, Key };
use caper::shader::Shaders;
use caper::utils::{ Vertex, calc_normal };
use caper::renderer::{ RenderItem, Transform, Renderer, CamState, FIXED_TIME_STAMP};
use noise::{ perlin2, Seed };
use fps_counter::FPSCounter;

fn main() {
    // init the systems
    let mut input = Input::new();
    let renderer = Renderer::new();
    let shaders = Shaders::new(&renderer.display);
    let mut fps = FPSCounter::new();

    renderer.setup();

    let map_size = 50f32;
    let fixed_val = -(map_size/2f32);
    let move_speed = 0.05f32;
    let mouse_speed = 10f32;

    //cam state
    let mut cam_state = CamState {
        cam_pos: (0f32, -1f32, 0f32),
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
                    pos: (fixed_val, 0.0, fixed_val),
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

        let mut movement_dirty = false;

        while accumulator >= FIXED_TIME_STAMP {
            accumulator -= FIXED_TIME_STAMP;

            // updating and handling the inputs
            input.update_inputs(&renderer.display);

            {
                let mv_matrix = Renderer::build_fp_view_matrix(cam_state); 

                // this can probably be cleaned up a bit
                if input.keys_down.contains(&Key::S) {
                    pseu_cam_pos.0 += mv_matrix[0][2] * move_speed; 
                    pseu_cam_pos.1 += mv_matrix[2][2] * move_speed; 
                    movement_dirty = true;
                }

                if input.keys_down.contains(&Key::W) {
                    pseu_cam_pos.0 -= mv_matrix[0][2] * move_speed; 
                    pseu_cam_pos.1 -= mv_matrix[2][2] * move_speed; 
                    movement_dirty = true;
                }

                if input.keys_down.contains(&Key::D) {
                    pseu_cam_pos.0 += mv_matrix[0][0] * move_speed; 
                    pseu_cam_pos.1 += mv_matrix[2][0] * move_speed; 
                    movement_dirty = true;
                }

                if input.keys_down.contains(&Key::A) {
                    pseu_cam_pos.0 -= mv_matrix[0][0] * move_speed; 
                    pseu_cam_pos.1 -= mv_matrix[2][0] * move_speed; 
                    movement_dirty = true;
                }

                cam_state.cam_rot.0 += input.mouse_delta.1 * mouse_speed;
                cam_state.cam_rot.1 += input.mouse_delta.0 * mouse_speed;
            }
        }

        if movement_dirty {
            render_items[0].vertices = gen_perlin_mesh(pseu_cam_pos, map_size);
            cam_state.cam_pos.1 = -2f32 - perlin2(&Seed::new(0),
            &[(pseu_cam_pos.0 - fixed_val) / 10f32,
            (pseu_cam_pos.1 - fixed_val) / 10f32]).abs() * 8f32;
        }

        //quit
        if input.keys_down.contains(&Key::Escape) { break; }

        println!("fps:{}", fps.tick());

        let sleep_duration =
            Duration::from_millis(((FIXED_TIME_STAMP - accumulator) / 1000000) as u64);
        thread::sleep(sleep_duration);
    }
}

fn gen_perlin_mesh(pseu_pos: (f32, f32), map_size: f32) -> Vec<Vertex> {
    // generate the instance positions 
    let mut vertices = Vec::new();

    let point_total = (map_size * map_size) as i32;
    let seed = Seed::new(0);

    // get all heights for first chunk 
    let mut size_00 = perlin2(&seed, &[0f32, 0f32]).abs() * 8f32;
    let mut size_10;
    let mut size_01 = perlin2(&seed, &[0f32, 0f32]).abs() * 8f32;
    let mut size_11;

    //let def_normal = [0f32, 0f32, 0f32];
    let def_uv = [0f32, 0f32];

    for i in 0 .. point_total {
        let pos = ((i as f32 % map_size), (i / map_size as i32) as f32);

        let p_pos = (pos.0 + pseu_pos.0, pos.1 + pseu_pos.1);

        size_10 = perlin2(&seed, &[(p_pos.0 + 1f32) / 10f32, p_pos.1 / 10f32]).abs() * 8f32;
        size_11 = perlin2(&seed, 
                          &[(p_pos.0 + 1f32) / 10f32, (p_pos.1 + 1f32) / 10f32]).abs() * 8f32;


        let p0 = [pos.0 + 1f32, size_10, pos.1];
        let p1 = [pos.0, size_00, pos.1];
        let p2 = [pos.0 + 1f32, size_11, pos.1 + 1f32];

        let calc_normal = calc_normal(p0, p1, p2);

        // create the two tris for this chunk
        vertices.push(Vertex {
            position: p0,
            normal: calc_normal,
            texture: def_uv
        });
        vertices.push(Vertex {
            position: p1,
            normal: calc_normal,
            texture: def_uv
        });
        vertices.push(Vertex {
            position: p2,
            normal: calc_normal,
            texture: def_uv
        });
        vertices.push(Vertex {
            position: [pos.0, size_00, pos.1],
            normal: calc_normal,
            texture: def_uv
        });
        vertices.push(Vertex {
            position: [pos.0 , size_01, pos.1 + 1f32],
            normal: calc_normal,
            texture: def_uv
        });
        vertices.push(Vertex {
            position: [pos.0 + 1f32, size_11, pos.1 + 1f32],
            normal: calc_normal,
            texture: def_uv
        });

        size_00 = size_10;
        size_01 = size_11;
    }

    vertices
}
