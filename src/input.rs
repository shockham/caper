use glium::Display;
use glium::glutin::VirtualKeyCode::*;
use glium::glutin::Event::{ KeyboardInput, MouseMoved };
use glium::glutin::ElementState::{ Pressed, Released };
use std::cell::Cell;
use renderer::{ Renderer, CamState };

static MOVE_SPEED: f32 = 0.2f32;
static LOOK_SPEED: f32 = 0.02f32;
static MOUSE_SPEED: f32 = 10f32;

pub struct Input {
    pub btns_down: [Cell<bool>; 9],
    mouse_pos: Cell<(i32, i32)>,
    mouse_delta: (Cell<f32>, Cell<f32>),
}

impl Input {
    /// Creates a new Input instance
    pub fn new() -> Input {
        Input {
            btns_down : [
                Cell::new(false), Cell::new(false), Cell::new(false),
                Cell::new(false), Cell::new(false), Cell::new(false),
                Cell::new(false),Cell::new(false),Cell::new(false)
            ],
            mouse_pos : Cell::new((0, 0)),
            mouse_delta : (Cell::new(0f32), Cell::new(0f32)),
        }
    }
    
    /// This method updates the state of the inputs
    pub fn update_inputs(&self, display: &Display) {

        let (width, height) = display.get_window().unwrap().get_inner_size().unwrap_or((800, 600));
        
        // reset the delta incase the mouse does not move
        self.mouse_delta.0.set(0f32);
        self.mouse_delta.1.set(0f32);

        // polling and handling the events received by the display
        for event in display.poll_events() {
            match event {
                KeyboardInput(Pressed, _, vkey) => {
                    match vkey{
                        Some(Escape) => self.btns_down[8].set(true),
                        Some(W) => self.btns_down[0].set(true),
                        Some(S) => self.btns_down[1].set(true),
                        Some(A) => self.btns_down[2].set(true),
                        Some(D) => self.btns_down[3].set(true),
                        Some(Left) => self.btns_down[4].set(true),
                        Some(Right) => self.btns_down[5].set(true),
                        Some(Up) => self.btns_down[6].set(true),
                        Some(Down) => self.btns_down[7].set(true),
                        Some(k) => println!("pressed key: {:?}", k),
                        _ => ()
                    }
                }, 
                KeyboardInput(Released, _, vkey) => {
                    match vkey{
                        Some(W) => self.btns_down[0].set(false),
                        Some(S) => self.btns_down[1].set(false),
                        Some(A) => self.btns_down[2].set(false),
                        Some(D) => self.btns_down[3].set(false),
                        Some(Left) => self.btns_down[4].set(false),
                        Some(Right) => self.btns_down[5].set(false),
                        Some(Up) => self.btns_down[6].set(false),
                        Some(Down) => self.btns_down[7].set(false),
                        Some(k) => println!("released key: {:?}", k),
                        _ => ()
                    }
                },
                MouseMoved(a) => { 
                    let mouse_diff = (self.mouse_pos.get().0 - a.0, self.mouse_pos.get().1 - a.1);
                    self.mouse_delta.0.set((mouse_diff.0 as f32)/(width as f32));
                    self.mouse_delta.1.set((mouse_diff.1 as f32)/(height as f32));
                    self.mouse_pos.set(a)
                },
                _ => ()
            }
        }
        // possible fix for grabbed cursor but not implemented on osx yet
        /*display.get_window()
            .unwrap()
            .set_cursor_position(0, 0);*/
    }
    
    /// This method is where data transforms take place due to inputs
    pub fn handle_inputs(&self, cam_state: &mut CamState) {
        let mv_matrix = Renderer::build_fp_view_matrix(cam_state.cam_pos, cam_state.cam_rot);

        //changing the camera position based on input events
        for b in 0..self.btns_down.len() {
            if self.btns_down[b].get() {
                match b {
                    0 => {
                        for i in 0..cam_state.cam_pos.len() {
                            cam_state.cam_pos[i] += mv_matrix[i][2] * MOVE_SPEED; 
                        }
                    },
                    1 => {
                        for i in 0..cam_state.cam_pos.len() {
                            cam_state.cam_pos[i] -= mv_matrix[i][2] * MOVE_SPEED; 
                        }
                    },
                    2 => {
                        for i in 0..cam_state.cam_pos.len() {
                            cam_state.cam_pos[i] += mv_matrix[i][0] * MOVE_SPEED; 
                        }
                    },
                    3 => {
                        for i in 0..cam_state.cam_pos.len() {
                            cam_state.cam_pos[i] -= mv_matrix[i][0] * MOVE_SPEED; 
                        }
                    },
                    4 => { cam_state.cam_rot[1] += LOOK_SPEED; },
                    5 => { cam_state.cam_rot[1] -= LOOK_SPEED; },
                    6 => { cam_state.cam_rot[0] += LOOK_SPEED; },
                    7 => { cam_state.cam_rot[0] -= LOOK_SPEED; },
                    _ => { },
                }
            }
        }

        cam_state.cam_rot[0] += self.mouse_delta.1.get() * MOUSE_SPEED;
        cam_state.cam_rot[1] += self.mouse_delta.0.get() * MOUSE_SPEED;
    }
}
