extern crate time;
extern crate fps_counter;
extern crate noise;

#[macro_use]
extern crate caper;

use caper::utils::create_skydome;
use caper::renderer::{ RenderItem, TextItem, Transform };
use caper::mesh::{ gen_perlin_mesh, gen_sphere };
use noise::{ perlin2, Seed };
use fps_counter::FPSCounter;

fn main() {
    let mut fps = FPSCounter::new();

    let map_size = 50f32;
    let fixed_val = -(map_size/2f32);
    let move_speed = 0.05f32;
    let mouse_speed = 10f32;

    let mut pseu_cam_pos = (0f32, 0f32);
    let sphere_pos = (8f32, 10f32);

    // create a vector of render items
    let mut render_items = vec![
        RenderItem {
            vertices: gen_perlin_mesh(pseu_cam_pos, map_size),
            shader_index: 3,
            instance_transforms: vec![
                Transform {
                    pos: (fixed_val, 0.0, fixed_val),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                    update_fn: Vec::new(),
                }
            ]
        },
        create_skydome(),
        RenderItem {
            vertices: gen_sphere(),
            shader_index: 4,
            instance_transforms: vec![
                Transform {
                    pos: (sphere_pos.0, 3.0, sphere_pos.1),
                    rot: (0f32, 0f32, 0f32, 1f32),
                    scale: (1f32, 1f32, 1f32),
                    update_fn: Vec::new(),
                }
            ]
        }
    ];

    let mut text_items = vec![
        TextItem {
            text: "text".to_string(),
            pos: (-1.0f32, 0.95f32, 0f32),
            color: (0f32, 0f32, 0f32, 1f32),
            scale: (1f32, 1f32, 1f32),
            update_fn: Vec::new(),
        }
    ];

    let mut movement_dirty = false;

    game_loop! {
        input,
        renderer,
        shaders,
        cam_state,
        render_items,
        text_items,
        // define a block for update
        {
            // block for handling the inputs
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

            // only regenerate the mesh if movement
            if movement_dirty {
                render_items[0].vertices = gen_perlin_mesh(pseu_cam_pos, map_size);
                cam_state.cam_pos.1 = -2.5f32 - perlin2(&Seed::new(0),
                &[(pseu_cam_pos.0 - fixed_val) / 10f32,
                (pseu_cam_pos.1 - fixed_val) / 10f32]).abs() * 8f32;

                // update the sphere location
                render_items[2].instance_transforms[0].pos = (sphere_pos.0 - pseu_cam_pos.0, 3.0, sphere_pos.1 - pseu_cam_pos.1);
            }

            //quit
            if input.keys_down.contains(&Key::Escape) { break; }

            // set the fps counter
            text_items[0].text = format!(
                "fps:{}|t:{}|pos:{:?}",
                fps.tick(),
                (time::precise_time_s() - renderer.start_time) as f32,
                pseu_cam_pos
            );
        }
    }
}
