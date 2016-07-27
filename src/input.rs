use glium::Display;
pub use glium::glutin::VirtualKeyCode as Key;
pub use glium::glutin::MouseButton as MouseButton;
use glium::glutin::MouseScrollDelta;
use glium::glutin::Event::{ KeyboardInput, MouseMoved, MouseInput, MouseWheel };
use glium::glutin::ElementState::{ Pressed, Released };
use glium::glutin::CursorState::{ Normal, Hide };
use renderer::{ Renderer };
use types::CamState;


/// struct for abstracting the state for all the inputs
pub struct Input {
    pub mouse_pos: (i32, i32),
    pub mouse_delta: (f32, f32),
    pub mouse_wheel_delta: (f32, f32),
    pub keys_down: Vec<Key>,
    pub mouse_btns_down: Vec<MouseButton>,
    pub hide_mouse: bool,
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
            mouse_btns_down: Vec::new(),
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

        // polling and handling the events received by the display
        for event in display.poll_events() {
            match event {
                KeyboardInput(Pressed, _, vkey) => {
                    self.keys_down.push(vkey.unwrap());
                },
                KeyboardInput(Released, _, vkey) => {
                    self.keys_down.retain(|&k| k != vkey.unwrap());
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
                },
                MouseInput(Released, btn) => {
                    self.mouse_btns_down.retain(|&mb| mb != btn);
                },
                MouseWheel(delta, _) => {
                    self.mouse_wheel_delta = match delta {
                        MouseScrollDelta::LineDelta(x, y) => (x, y),
                        MouseScrollDelta::PixelDelta(x, y) => (x, y),
                    };
                },
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

        let mv_matrix = Renderer::build_fp_view_matrix(cam_state);

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
    }
}
