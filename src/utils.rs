extern crate genmesh;
extern crate time;
extern crate obj;

use std::ops::{Add, Mul};
use std::iter::Sum;
use std::f32::consts::PI;

use types::{ RenderItem, Transform, Vertex, Quaternion, Vector3, CamState };

/// quick macro to use in the examples for easily defining all the modules and game loop
#[macro_export]
macro_rules! game_loop {
    ( Input => $input:ident,
      Renderer => $renderer:ident,
      CamState => $cam_state:ident,
      RenderItems => $render_items:ident,
      TextItems => $text_items:ident,
      start => $start:block,
      update => $update:block,
      $ui:ident => $ui_update:block) => {
        {
            use caper::renderer::Renderer;
            use caper::types::{ CamState, Entity };
            use caper::input::{ Input, Key, MouseButton };
            use caper::imgui::Ui;

            let mut $input = Input::new();
            let mut $renderer = Renderer::new("Caper".to_string());

            //cam state
            let mut $cam_state = CamState {
                cam_pos: (0.0f32, 0.0, 0.0),
                cam_rot: (0.0f32, 0.0, 0.0)
            };

            $start;

            // the main loop
            loop {
                // quit
                if $input.keys_down.contains(&Key::Escape) { break; }

                {
                    // define the closure for ui updating
                    let mut render_imgui = |$ui: &Ui| $ui_update;
                    // render the frame
                    $renderer.draw(&$cam_state, &$render_items, &$text_items, &mut render_imgui);

                    // updating and handling the inputs
                    $input.update_inputs(&$renderer.display);

                    // update the inputs for imgui
                    $renderer.update_imgui_input($input.mouse_pos,
                                                 ($input.mouse_btns_down.contains(&MouseButton::Left), false, false));
                }

                // auto call update functions
                {
                    // call the update functions for RenderItems
                    for i in 0..$render_items.len() {
                        for t in 0..$render_items[i].instance_transforms.len() {
                            $render_items[i].instance_transforms[t].update();
                        }
                    }

                    // call the update functions for the TextItems
                    for i in 0..$text_items.len() {
                        $text_items[i].update();
                    }
                }

                // the update block for other updates
                {
                    $update
                }
            }
        }
    };
}


/// Returns a Vec<Vertex> that should be converted to buffer and rendered as `TrianglesList`.
pub fn load_wavefront( data: &[u8]) -> Vec<Vertex> {
    let mut data = ::std::io::BufReader::new(data);
    let data = obj::Obj::load(&mut data);

    let mut vertex_data = Vec::new();

    for shape in data.object_iter().next().unwrap().group_iter().flat_map(|g| g.indices().iter()) {
        match shape {
            &genmesh::Polygon::PolyTri(genmesh::Triangle { x: v1, y: v2, z: v3 }) => {
                for v in [v1, v2, v3].iter() {
                    let position = data.position()[v.0];
                    let texture = v.1.map(|index| data.texture()[index]);
                    let normal = v.2.map(|index| data.normal()[index]);

                    let texture = texture.unwrap_or([0.0, 0.0]);
                    let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                    vertex_data.push(Vertex {
                        position: position,
                        normal: normal,
                        texture: texture,
                    })
                }
            },
            _ => unimplemented!()
        }
    }

    vertex_data
}


/// Returns a RenderItem for the skydome
pub fn create_skydome(shader_name: &'static str) -> RenderItem {
    RenderItem {
        vertices: load_wavefront(include_bytes!("./resources/skydome.obj")),
        shader_name: shader_name,
        instance_transforms: vec![
            Transform {
                active: true,
                pos: (0.0, 0.0, 0.0),
                rot: (0f32, 0f32, 0f32, 1f32),
                scale: (100f32, 100f32, 100f32),
                update_fn: Vec::new(),
            }
        ],
        active: true,
    }
}

/// Returns the dot product of two vectors
pub fn dotp<T>(this: &[T], other: &[T]) -> T where T:Add<T, Output=T> + Mul<T, Output=T> + Sum + Copy {
    assert!(this.len() == other.len(), "The dimensions must be equal");

    this.iter().zip(other.iter())
        .map(|(&a, &b)| a * b)
        .sum()
}

/// returns the cross product of two vectors
pub fn crossp(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [(a[1] * b[2]) - (a[2] * b[1]), (a[2] * b[0]) - (a[0] * b[2]), (a[0] * b[1]) - (a[1] * b[0])]
}

/// returns the resultant vector of a - b
pub fn sub_vec3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// returns the normal calculated from the three vectors supplied
pub fn calc_normal(p0: [f32; 3], p1: [f32; 3], p2: [f32; 3]) -> [f32; 3] {
    let a = sub_vec3(p1, p0);
    let b = sub_vec3(p2, p0);

    crossp(a, b)
}

/// returns a euler angle as a quaternion
pub fn to_quaternion(angle: Vector3) -> Quaternion {
    let (c3, c1, c2) = ((angle.0 / 2f32).cos(), (angle.1 / 2f32).cos(), (angle.2 / 2f32).cos());
    let (s3, s1, s2) = ((angle.0 / 2f32).sin(), (angle.1 / 2f32).sin(), (angle.2 / 2f32).sin());

    let c1c2 = c1 * c2;
    let s1s2 = s1 * s2;
    let w = c1c2 * c3 - s1s2 * s3;
    let x = c1c2 * s3 + s1s2 * c3;
    let y = s1 * c2 * c3 + c1 * s2 * s3;
    let z = c1 * s2 * c3 - s1 * c2 * s3;

    (x, y, z, w)
}

/// Returns perspective projection matrix given fov, aspect ratio, z near and far
pub fn build_persp_proj_mat(fov:f32,aspect:f32,znear:f32,zfar:f32) -> [[f32; 4]; 4] {
    let ymax = znear * (fov * (PI/360.0)).tan();
    let ymin = -ymax;
    let xmax = ymax * aspect;
    let xmin = ymin * aspect;

    let width = xmax - xmin;
    let height = ymax - ymin;

    let depth = zfar - znear;
    let q = -(zfar + znear) / depth;
    let qn = -2.0 * (zfar * znear) / depth;

    let w = 2.0 * znear / width;
    let h = 2.0 * znear / height;

    [[w, 0.0f32, 0.0f32, 0.0f32],
    [0.0f32, h, 0.0f32, 0.0f32],
    [0.0f32, 0.0f32, q, -1.0f32],
    [0.0f32, 0.0f32, qn, 0.0f32]]
}

/// Returns the model view matrix for a first person view given cam position and rotation
pub fn build_fp_view_matrix(cam_state: &CamState) -> [[f32; 4]; 4] {

    let (sin_yaw, cos_yaw, sin_pitch, cos_pitch) = (
        cam_state.cam_rot.1.sin(),
        cam_state.cam_rot.1.cos(),
        cam_state.cam_rot.0.sin(),
        cam_state.cam_rot.0.cos());
    let xaxis = [cos_yaw, 0.0, -sin_yaw];
    let yaxis = [sin_yaw * sin_pitch, cos_pitch, cos_yaw * sin_pitch];
    let zaxis = [sin_yaw * cos_pitch, -sin_pitch, cos_pitch * cos_yaw];

    let cam_arr = [cam_state.cam_pos.0, cam_state.cam_pos.1, cam_state.cam_pos.2];

    [[ xaxis[0], yaxis[0], zaxis[0], 0.0],
    [ xaxis[1], yaxis[1], zaxis[1], 0.0],
    [ xaxis[2], yaxis[2], zaxis[2], 0.0],
    [ -dotp(&xaxis, &cam_arr), -dotp(&yaxis, &cam_arr), -dotp(&zaxis, &cam_arr), 1.0f32]]
}
