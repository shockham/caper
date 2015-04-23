
use glium::Display;
use glutin::VirtualKeyCode::*;
use glutin::Event::KeyboardInput;
use glutin::ElementState::{ Pressed, Released };

static MOVE_SPEED: f32 = 0.2f32;
static LOOK_SPEED: f32 = 0.02f32;

pub fn get_inputs(display: &Display, inputs: [bool; 9]) -> [bool; 9] {

    let mut btns_down = inputs;

    for event in display.poll_events() {
        match event {
            //Closed => btns_down[8] = true,
            KeyboardInput(Pressed, _, vkey) => {
                match vkey{
                    Some(Escape) => btns_down[8] = true,
                    Some(W) => btns_down[0] = true,
                    Some(S) => btns_down[1] = true,
                    Some(A) => btns_down[2] = true,
                    Some(D) => btns_down[3] = true,
                    Some(Left) => btns_down[4] = true,
                    Some(Right) => btns_down[5] = true,
                    Some(Up) => btns_down[6] = true,
                    Some(Down) => btns_down[7] = true,
                    Some(k) => println!("pressed key: {:?}", k),
                    _ => ()
                }
            }, 
            KeyboardInput(Released, _, vkey) => {
                match vkey{
                    Some(W) => btns_down[0] = false,
                    Some(S) => btns_down[1] = false,
                    Some(A) => btns_down[2] = false,
                    Some(D) => btns_down[3] = false,
                    Some(Left) => btns_down[4] = false,
                    Some(Right) => btns_down[5] = false,
                    Some(Up) => btns_down[6] = false,
                    Some(Down) => btns_down[7] = false,
                    Some(k) => println!("released key: {:?}", k),
                    _ => ()
                }
            }, 
            _ => ()
        }
    }

    btns_down
}

pub fn handle_inputs(btns_down: &mut [bool; 9], cam_pos: &mut [f32; 3], cam_rot: &mut [f32; 3], mv_matrix: [[f32; 4]; 4]){
    //changing the camera position based on input events
    for b in 0..btns_down.len() {
        if btns_down[b] {
            match b {
                0 => {
                    for i in 0..cam_pos.len() {
                        cam_pos[i] += mv_matrix[i][2] * MOVE_SPEED; 
                    }
                },
                1 => {
                    for i in 0..cam_pos.len() {
                        cam_pos[i] -= mv_matrix[i][2] * MOVE_SPEED; 
                    }
                },
                2 => {
                    for i in 0..cam_pos.len() {
                        cam_pos[i] += mv_matrix[i][0] * MOVE_SPEED; 
                    }
                },
                3 => {
                    for i in 0..cam_pos.len() {
                        cam_pos[i] -= mv_matrix[i][0] * MOVE_SPEED; 
                    }
                },
                4 => { cam_rot[1] += LOOK_SPEED; },
                5 => { cam_rot[1] -= LOOK_SPEED; },
                6 => { cam_rot[0] += LOOK_SPEED; },
                7 => { cam_rot[0] -= LOOK_SPEED; },
                _ => { },
            }
        }
    } 
}
