use glium::Display;
pub use glium::glutin::VirtualKeyCode as Key;
pub use glium::glutin::MouseButton as MouseButton;
use glium::glutin::MouseScrollDelta;
use glium::glutin::Event::{ KeyboardInput, MouseMoved, MouseInput, MouseWheel, ReceivedCharacter };
use glium::glutin::ElementState::{ Pressed, Released };
use glium::glutin::CursorState::{ Normal, Hide };

use std::f32::consts::PI;

use types::CamState;
use utils::build_fp_view_matrix;

/// Double pi
const TWO_PI:f32 = PI * 2f32;


/// struct for abstracting the state for all the inputs
pub struct Input {
    /// The position of the mouse
    pub mouse_pos: (i32, i32),
    /// The difference in mouse position from the last frame
    pub mouse_delta: (f32, f32),
    /// The difference in position of the mouse wheen from the previous frame
    pub mouse_wheel_delta: (f32, f32),
    /// The keys that are currently pressed down
    pub keys_down: Vec<Key>,
    /// The keys that have been pressed on this frame
    pub keys_pressed: Vec<Key>,
    /// The keys that have been released on this frame
    pub keys_released: Vec<Key>,
    /// Characters received that are pressed down
    pub characters_down: Vec<char>,
    /// The mouse buttons that are currently pressed down
    pub mouse_btns_down: Vec<MouseButton>,
    /// The mouse buttons that have been pressed down on this frame
    pub mouse_btns_pressed: Vec<MouseButton>,
    /// The mouse buttons that have been release on this frame
    pub mouse_btns_released: Vec<MouseButton>,
    /// Whether to show or hide the mouse
    pub hide_mouse: bool,
    /// Internal field to track if the cursor is grabbed
    cursor_grabbed: bool,
}

impl Input {
    /// Creates a new Input instance
    pub fn new() -> Input {
        Input {
            mouse_pos : (0, 0),
            mouse_delta : (0f32, 0f32),
            mouse_wheel_delta: (0f32, 0f32),
            keys_down : Vec::new(),
            keys_pressed: Vec::new(),
            keys_released: Vec::new(),
            characters_down: Vec::new(),
            mouse_btns_down: Vec::new(),
            mouse_btns_pressed: Vec::new(),
            mouse_btns_released: Vec::new(),
            hide_mouse: true,
            cursor_grabbed: false,
        }
    }

    /// This method updates the state of the inputs
    pub fn update_inputs(&mut self, display: &Display) {
        let window = display.get_window().unwrap();
        let (width, height) = window.get_inner_size().unwrap_or((800, 600));
        let hidpi_factor = window.hidpi_factor();

        // reset the delta incase the mouse does not move
        self.mouse_delta = (0f32, 0f32);
        self.mouse_wheel_delta = (0f32, 0f32);

        // keys pressed is only for a single frame so clear
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.mouse_btns_pressed.clear();
        self.mouse_btns_released.clear();
        self.characters_down.clear();

        // polling and handling the events received by the display
        for event in display.poll_events() {
            match event {
                KeyboardInput(Pressed, _, vkey) => {
                    self.keys_down.push(vkey.unwrap());
                    self.keys_pressed.push(vkey.unwrap());
                },
                KeyboardInput(Released, _, vkey) => {
                    self.keys_down.retain(|&k| k != vkey.unwrap());
                    self.keys_released.push(vkey.unwrap());
                },
                MouseMoved(x, y) => {
                    let mouse_diff = ((width / 2) as i32 - (x as f32 / hidpi_factor) as i32,
                                      (height / 2) as i32 - (y as f32 / hidpi_factor) as i32);
                    self.mouse_delta.0 = (mouse_diff.0 as f32)/(width as f32);
                    self.mouse_delta.1 = (mouse_diff.1 as f32)/(height as f32);
                    self.mouse_pos = (x, y);
                },
                MouseInput(Pressed, btn) => {
                    self.mouse_btns_down.push(btn);
                    self.mouse_btns_pressed.push(btn);
                },
                MouseInput(Released, btn) => {
                    self.mouse_btns_down.retain(|&mb| mb != btn);
                    self.mouse_btns_released.push(btn);
                },
                MouseWheel(delta, _) => {
                    self.mouse_wheel_delta = match delta {
                        MouseScrollDelta::LineDelta(x, y) => (x, y),
                        MouseScrollDelta::PixelDelta(x, y) => (x, y),
                    };
                },
                ReceivedCharacter(c) => self.characters_down.push(c),
                _ => ()
            }
        }

        if self.hide_mouse {
            // set the mouse to the centre of the screen
            if self.cursor_grabbed {
                window.set_cursor_state(Hide).ok();
                self.cursor_grabbed = false;
            }
            let _ = window.set_cursor_position((width / 2) as i32, (height / 2) as i32);
        } else {
            if !self.cursor_grabbed {
                window.set_cursor_state(Normal).ok();
                self.cursor_grabbed = true;
            }
        }
    }

    /// This method is where data transforms take place due to inputs
    /// for a first person camera
    pub fn handle_fp_inputs(&self, cam_state: &mut CamState) {
        // some static vals to use the fp inputs
        const MOVE_SPEED: f32 = 0.2f32;
        const MOUSE_SPEED: f32 = 10f32;

        let mv_matrix = build_fp_view_matrix(cam_state);

        if self.keys_down.contains(&Key::S) {
            cam_state.cam_pos.0 += mv_matrix[0][2] * MOVE_SPEED;
            cam_state.cam_pos.1 += mv_matrix[1][2] * MOVE_SPEED;
            cam_state.cam_pos.2 += mv_matrix[2][2] * MOVE_SPEED;
        }

        if self.keys_down.contains(&Key::W) {
            cam_state.cam_pos.0 -= mv_matrix[0][2] * MOVE_SPEED;
            cam_state.cam_pos.1 -= mv_matrix[1][2] * MOVE_SPEED;
            cam_state.cam_pos.2 -= mv_matrix[2][2] * MOVE_SPEED;
        }

        if self.keys_down.contains(&Key::D) {
            cam_state.cam_pos.0 += mv_matrix[0][0] * MOVE_SPEED;
            cam_state.cam_pos.1 += mv_matrix[1][0] * MOVE_SPEED;
            cam_state.cam_pos.2 += mv_matrix[2][0] * MOVE_SPEED;
        }

        if self.keys_down.contains(&Key::A) {
            cam_state.cam_pos.0 -= mv_matrix[0][0] * MOVE_SPEED;
            cam_state.cam_pos.1 -= mv_matrix[1][0] * MOVE_SPEED;
            cam_state.cam_pos.2 -= mv_matrix[2][0] * MOVE_SPEED;
        }

        cam_state.cam_rot.0 += self.mouse_delta.1 * MOUSE_SPEED;
        cam_state.cam_rot.1 += self.mouse_delta.0 * MOUSE_SPEED;

        cam_state.cam_rot.0 = fix_rot(cam_state.cam_rot.0);
        cam_state.cam_rot.1 = fix_rot(cam_state.cam_rot.1);

        // make sure cam_rot always between 0 and 2PI
        fn fix_rot (num:f32) -> f32 {
            if num < 0f32 {
                return TWO_PI - num;
            }

            num % TWO_PI
        }
    }
}
