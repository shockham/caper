extern crate genmesh;
extern crate obj;

use std::ops::{Add, Mul};
use std::iter::Sum;
use std::f32::consts::PI;

use types::{ RenderItem, Transform, Vertex, Quaternion, Vector3, CamState, PhysicsType };

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
            use caper::types::{ CamState, PhysicsType };
            use caper::input::{ Input, Key, MouseButton };
            use caper::imgui::Ui;
            use caper::nalgebra::Vector3 as nVector3;
            use caper::nalgebra::Translation3;
            use caper::nalgebra::core::storage::OwnedStorage;
            use caper::nphysics3d::world::World;
            use caper::nphysics3d::object::{ RigidBody, WorldObject };
            use caper::ncollide::shape::Cuboid;

            use std::boxed::Box;
            use std::time::Instant;

            const PHYSICS_DIVISOR:f32 = 2f32;
            const GLOBAL_REST:f32 = 0.05f32;

            // init caper systems
            let mut $input = Input::new();
            let mut $renderer = Renderer::new("caper window".to_string());

            // init physics
            let mut world = World::new();
            world.set_gravity(nVector3::new(0.0, -9.81, 0.0));

            // add physics items
            for i in 0 .. $render_items.len() {
                match $render_items[i].physics_type {
                    PhysicsType::Static => {
                        for j in 0 .. $render_items[i].instance_transforms.len() {
                            let ri_trans = $render_items[i].instance_transforms[j];

                            let geom = Cuboid::new(
                                nVector3::new(ri_trans.scale.0, ri_trans.scale.1, ri_trans.scale.2));
                            let mut rb = RigidBody::new_static(geom, GLOBAL_REST, 0.6);

                            rb.append_translation(&Translation3::new(ri_trans.pos.0, ri_trans.pos.1, ri_trans.pos.2));

                            // track which render item instance this refers to
                            rb.set_user_data(Some(Box::new((i, j))));

                            rb.set_margin(0f32);

                            world.add_rigid_body(rb);
                        }
                    },
                    PhysicsType::Dynamic => {
                        for j in 0 .. $render_items[i].instance_transforms.len() {
                            let ri_trans = $render_items[i].instance_transforms[j];

                            let geom = Cuboid::new(
                                nVector3::new(ri_trans.scale.0, ri_trans.scale.1, ri_trans.scale.2));
                            let mut rb = RigidBody::new_dynamic(geom, 5.0, GLOBAL_REST, 0.8);

                            rb.append_translation(&Translation3::new(ri_trans.pos.0, ri_trans.pos.1, ri_trans.pos.2));

                            // track which render item instance this refers to
                            rb.set_user_data(Some(Box::new((i, j))));

                            rb.set_margin(0f32);

                            if i == 1 && j == 0 {
                                rb.set_deactivation_threshold(None);
                            }

                            world.add_rigid_body(rb);
                        }
                    },
                    PhysicsType::None => {},
                }
            }

            //cam state
            let mut $cam_state = CamState {
                cam_pos: (0.0f32, 0.0, 0.0),
                cam_rot: (0.0f32, 0.0, 0.0)
            };

            $start;

            let mut delta = 0.016666667f32;

            // the main loop
            loop {
                let frame_start = Instant::now();
                // quit
                if $input.keys_down.contains(&Key::Escape) { break; }

                // block for updating physics
                {
                    world.step(delta);

                    for rbi in world.rigid_bodies() {
                        // actually get access to the rb :|
                        let wo = WorldObject::RigidBody(rbi.clone());
                        let rb = wo.borrow_rigid_body();

                        // update the RenderItem transform pos
                        let trans = rb.position().translation.vector;
                        let rot = rb.position().rotation.coords.data.as_slice();
                        //let quat = to_quaternion((rot.x, rot.y, rot.z));

                        let user_data = rb.user_data().unwrap();
                        let &(ri_i, ri_it_i) = user_data.downcast_ref::<(usize, usize)>().unwrap();

                        $render_items[ri_i].instance_transforms[ri_it_i].pos =
                            (trans.x / PHYSICS_DIVISOR,
                             trans.y / PHYSICS_DIVISOR,
                             trans.z / PHYSICS_DIVISOR);
                        $render_items[ri_i].instance_transforms[ri_it_i].rot = (rot[0], rot[1], rot[2], rot[3]);
                    }
                }

                {
                    // define the closure for ui updating
                    let mut render_imgui = |$ui: &Ui| $ui_update;
                    // render the frame
                    $renderer.draw(&$cam_state, &$render_items, &$text_items, &mut render_imgui);

                    // updating and handling the inputs
                    $input.update_inputs(&$renderer.display);

                    // update the inputs for imgui
                    $renderer.update_imgui_input($input.mouse_pos,
                                                 ($input.mouse_btns_down.contains(
                                                         &MouseButton::Left), false, false));
                }

                // the update block for other updates
                {
                    $update
                }

                // update the new positions back to rb
                {
                    for rbi in world.rigid_bodies() {
                        // actually get access to the rb :|
                        let mut wo = WorldObject::RigidBody(rbi.clone());

                        let (ri_i, ri_it_i) = {
                            let rb = wo.borrow_rigid_body();

                            let user_data = rb.user_data().unwrap();
                            let tup_ref = user_data.downcast_ref::<(usize, usize)>().unwrap();

                            *tup_ref
                        };

                        let mut rb = wo.borrow_mut_rigid_body();

                        // update the rb transform pos
                        let ri_pos = $render_items[ri_i].instance_transforms[ri_it_i].pos;
                        rb.set_translation(
                            Translation3::new(ri_pos.0 * PHYSICS_DIVISOR,
                                              ri_pos.1 * PHYSICS_DIVISOR,
                                              ri_pos.2 * PHYSICS_DIVISOR));

                        /* re updating the rb causes problems
                        let ri_rot = to_euler($render_items[ri_i].instance_transforms[ri_it_i].rot);
                        rb.set_rotation(nVector3::new(ri_rot.0, ri_rot.1, ri_rot.2));
                        */
                    }
                }

                delta = 0.000000001f32 * frame_start.elapsed().subsec_nanos() as f32;
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
        shader_name: String::from(shader_name),
        instance_transforms: vec![
            Transform {
                active: true,
                pos: (0.0, 0.0, 0.0),
                rot: (0f32, 0f32, 0f32, 1f32),
                scale: (300f32, 300f32, 300f32),
            }
        ],
        active: true,
        physics_type: PhysicsType::None,
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

/// returns a quaternion from a euler angle
pub fn to_euler(angle: Quaternion) -> Vector3 {
    let ysqr = angle.1 * angle.1;
    let t0 = -2.0f32 * (ysqr + angle.2 * angle.2) + 1.0f32;
    let t1 = 2.0f32 * (angle.0 * angle.1 - angle.3 * angle.2);
    let mut t2 = -2.0f32 * (angle.0 * angle.2 + angle.3 * angle.1);
    let t3 = 2.0f32 * (angle.1 * angle.2 - angle.3 * angle.0);
    let t4 = -2.0f32 * (angle.0 * angle.0 + ysqr) + 1.0f32;

    t2 = if t2 > 1.0f32 { 1.0f32 } else { t2 };
    t2 = if t2 < -1.0f32 { -1.0f32 } else { t2 };

    let pitch = t2.asin();
    let roll = t3.atan2(t4);
    let yaw = t1.atan2(t0);

    (pitch, roll, yaw)
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
