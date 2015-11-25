use glium::Display;
pub use glium::glutin::VirtualKeyCode as Key;
use glium::glutin::Event::{ KeyboardInput, MouseMoved };
use glium::glutin::ElementState::{ Pressed, Released };
use renderer::{ Renderer, CamState };

static MOVE_SPEED: f32 = 0.2f32;
static MOUSE_SPEED: f32 = 10f32;

pub struct Input {
    mouse_pos: (i32, i32),
    pub mouse_delta: (f32, f32),
    pub keys_down: Vec<Key>
}

impl Input {
    /// Creates a new Input instance
    pub fn new() -> Input {
        Input {
            mouse_pos : (0, 0),
            mouse_delta : (0f32, 0f32),
            keys_down : Vec::new(),
        }
    }

    /// This method updates the state of the inputs
    pub fn update_inputs(&mut self, display: &Display) {

        let (width, height) = display.get_window().unwrap().get_inner_size().unwrap_or((800, 600));

        // reset the delta incase the mouse does not move
        self.mouse_delta.0 = 0f32;
        self.mouse_delta.1 = 0f32;

        // polling and handling the events received by the display
        for event in display.poll_events() {
            match event {
                KeyboardInput(Pressed, _, vkey) => {
                    self.keys_down.push(vkey.unwrap());
                }, 
                KeyboardInput(Released, _, vkey) => {
                    self.keys_down.retain(|&k| k != vkey.unwrap());
                },
                MouseMoved(a) => { 
                    let mouse_diff = (self.mouse_pos.0 - a.0, self.mouse_pos.1 - a.1);
                    self.mouse_delta.0 = (mouse_diff.0 as f32)/(width as f32);
                    self.mouse_delta.1 = (mouse_diff.1 as f32)/(height as f32);
                    self.mouse_pos = a
                },
                _ => ()
            }
        }
        // possible fix for grabbed cursor but not implemented on osx yet
        /*let _ = display.get_window()
          .unwrap()
          .set_cursor_position(0, 0);*/
    }

    /// This method is where data transforms take place due to inputs
    /// for a first person camera
    pub fn handle_fp_inputs(&self, cam_state: &mut CamState) {
        let mv_matrix = Renderer::build_fp_view_matrix(*cam_state); 

        // this can probably be cleaned up a bit
        if self.keys_down.contains(&Key::W) {
            cam_state.cam_pos.0 += mv_matrix[0][2] * MOVE_SPEED; 
            cam_state.cam_pos.1 += mv_matrix[1][2] * MOVE_SPEED; 
            cam_state.cam_pos.2 += mv_matrix[2][2] * MOVE_SPEED; 
        }

        if self.keys_down.contains(&Key::S) {
            cam_state.cam_pos.0 -= mv_matrix[0][2] * MOVE_SPEED; 
            cam_state.cam_pos.1 -= mv_matrix[1][2] * MOVE_SPEED; 
            cam_state.cam_pos.2 -= mv_matrix[2][2] * MOVE_SPEED; 
        }

        if self.keys_down.contains(&Key::A) {
            cam_state.cam_pos.0 += mv_matrix[0][0] * MOVE_SPEED; 
            cam_state.cam_pos.1 += mv_matrix[1][0] * MOVE_SPEED; 
            cam_state.cam_pos.2 += mv_matrix[2][0] * MOVE_SPEED; 
        }

        if self.keys_down.contains(&Key::D) {
            cam_state.cam_pos.0 -= mv_matrix[0][0] * MOVE_SPEED; 
            cam_state.cam_pos.1 -= mv_matrix[1][0] * MOVE_SPEED; 
            cam_state.cam_pos.2 -= mv_matrix[2][0] * MOVE_SPEED; 
        }

        cam_state.cam_rot.0 += self.mouse_delta.1 * MOUSE_SPEED;
        cam_state.cam_rot.1 += self.mouse_delta.0 * MOUSE_SPEED;
    }
}
