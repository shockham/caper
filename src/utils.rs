extern crate genmesh;
extern crate obj;

use std::ops::{Add, Mul};
use std::iter::Sum;
use std::f32::consts::PI;

use types::{RenderItem, RenderItemBuilder, TransformBuilder};
use types::{Vertex, Quaternion, Vector3, Matrix4, Camera, MaterialBuilder};

use input::{Key, Input};

const TWO_PI: f32 = PI * 2f32;


/// Returns a Vec<Vertex> that should be converted to buffer and rendered as `TrianglesList`.
pub fn load_wavefront(data: &[u8]) -> Vec<Vertex> {
    let mut data = ::std::io::BufReader::new(data);
    let data = obj::Obj::load_buf(&mut data).unwrap();

    let mut vertex_data = Vec::new();

    for shape in data.objects.iter().next().unwrap().groups.iter().flat_map(
        |g| {
            g.polys.iter()
        },
    )
    {
        match shape {
            &genmesh::Polygon::PolyTri(genmesh::Triangle {
                                           x: v1,
                                           y: v2,
                                           z: v3,
                                       }) => {
                for v in [v1, v2, v3].iter() {
                    let position = data.position[v.0];
                    let texture = v.1.map(|index| data.texture[index]);
                    let normal = v.2.map(|index| data.normal[index]);

                    let texture = texture.unwrap_or([0.0, 0.0]);
                    let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                    vertex_data.push(Vertex {
                        position: position,
                        normal: normal,
                        texture: texture,
                    })
                }
            }
            _ => unimplemented!(),
        }
    }

    vertex_data
}


/// Returns a RenderItem for the skydome
pub fn create_skydome<T: Clone + Default>(shader_name: &'static str) -> RenderItem<T> {
    RenderItemBuilder::default()
        .name("skydome".to_string())
        .vertices(load_wavefront(include_bytes!("./resources/skydome.obj")))
        .material(
            MaterialBuilder::default()
                .shader_name(shader_name.to_string())
                .build()
                .unwrap(),
        )
        .instance_transforms(vec![
            TransformBuilder::default()
                .scale((300f32, 300f32, 300f32))
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap()
}

/// Returns the dot product of two vectors
pub fn dotp<T>(this: &[T], other: &[T]) -> T
where
    T: Add<T, Output = T> + Mul<T, Output = T> + Sum + Copy,
{
    assert!(this.len() == other.len(), "The dimensions must be equal");

    this.iter().zip(other.iter()).map(|(&a, &b)| a * b).sum()
}

/// returns the cross product of two vectors
pub fn crossp(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        (a[1] * b[2]) - (a[2] * b[1]),
        (a[2] * b[0]) - (a[0] * b[2]),
        (a[0] * b[1]) - (a[1] * b[0]),
    ]
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

/// returns the two matrices multiplied together
pub fn mul_mat4(a: Matrix4, b: Matrix4) -> Matrix4 {
    let mut new_mat: Matrix4 = [[0f32; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            for x in 0..4 {
                new_mat[i][j] += a[x][i] * b[j][x];
            }
        }
    }

    new_mat
}

/// returns a euler angle as a quaternion
pub fn to_quaternion(angle: Vector3) -> Quaternion {
    let (c3, c1, c2) = (
        (angle.0 / 2f32).cos(),
        (angle.1 / 2f32).cos(),
        (angle.2 / 2f32).cos(),
    );
    let (s3, s1, s2) = (
        (angle.0 / 2f32).sin(),
        (angle.1 / 2f32).sin(),
        (angle.2 / 2f32).sin(),
    );

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
pub fn build_persp_proj_mat(fov: f32, aspect: f32, znear: f32, zfar: f32) -> Matrix4 {
    let ymax = znear * (fov * (PI / 360.0)).tan();
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

    [
        [w, 0.0f32, 0.0f32, 0.0f32],
        [0.0f32, h, 0.0f32, 0.0f32],
        [0.0f32, 0.0f32, q, -1.0f32],
        [0.0f32, 0.0f32, qn, 0.0f32],
    ]
}

/// Returns the model view matrix for a first person view given cam position and rotation
pub fn build_fp_view_matrix(cam: &Camera) -> Matrix4 {

    let (sin_yaw, cos_yaw, sin_pitch, cos_pitch) = (
        cam.euler_rot.1.sin(),
        cam.euler_rot.1.cos(),
        cam.euler_rot.0.sin(),
        cam.euler_rot.0.cos(),
    );
    let xaxis = [cos_yaw, 0.0, -sin_yaw];
    let yaxis = [sin_yaw * sin_pitch, cos_pitch, cos_yaw * sin_pitch];
    let zaxis = [sin_yaw * cos_pitch, -sin_pitch, cos_pitch * cos_yaw];

    let cam_arr = [cam.pos.0, cam.pos.1, cam.pos.2];

    [
        [xaxis[0], yaxis[0], zaxis[0], 0.0],
        [xaxis[1], yaxis[1], zaxis[1], 0.0],
        [xaxis[2], yaxis[2], zaxis[2], 0.0],
        [
            -dotp(&xaxis, &cam_arr),
            -dotp(&yaxis, &cam_arr),
            -dotp(&zaxis, &cam_arr),
            1.0f32,
        ],
    ]
}


/// This method is where data transforms take place due to inputs
/// for a first person camera
pub fn handle_fp_inputs(input: &mut Input, cam: &mut Camera) {
    // some static vals to use the fp inputs
    const MOVE_SPEED: f32 = 0.2f32;
    const MOUSE_SPEED: f32 = 0.01f32;

    let mv_matrix = build_fp_view_matrix(cam);

    if input.keys_down.contains(&Key::S) {
        cam.pos.0 += mv_matrix[0][2] * MOVE_SPEED;
        cam.pos.1 += mv_matrix[1][2] * MOVE_SPEED;
        cam.pos.2 += mv_matrix[2][2] * MOVE_SPEED;
    }

    if input.keys_down.contains(&Key::W) {
        cam.pos.0 -= mv_matrix[0][2] * MOVE_SPEED;
        cam.pos.1 -= mv_matrix[1][2] * MOVE_SPEED;
        cam.pos.2 -= mv_matrix[2][2] * MOVE_SPEED;
    }

    if input.keys_down.contains(&Key::D) {
        cam.pos.0 += mv_matrix[0][0] * MOVE_SPEED;
        cam.pos.1 += mv_matrix[1][0] * MOVE_SPEED;
        cam.pos.2 += mv_matrix[2][0] * MOVE_SPEED;
    }

    if input.keys_down.contains(&Key::A) {
        cam.pos.0 -= mv_matrix[0][0] * MOVE_SPEED;
        cam.pos.1 -= mv_matrix[1][0] * MOVE_SPEED;
        cam.pos.2 -= mv_matrix[2][0] * MOVE_SPEED;
    }

    cam.euler_rot.0 += input.mouse_delta.1 * MOUSE_SPEED;
    cam.euler_rot.1 += input.mouse_delta.0 * MOUSE_SPEED;

    cam.euler_rot.0 = fix_rot(cam.euler_rot.0);
    cam.euler_rot.1 = fix_rot(cam.euler_rot.1);

    // make sure euler_rot always between 0 and 2PI
    fn fix_rot(num: f32) -> f32 {
        if num < 0f32 {
            return TWO_PI - num;
        }

        num % TWO_PI
    }
}


/// Test whether an object is in the view frustrum
pub fn frustrum_test(
    pos: &Vector3,
    radius: f32,
    frustrum_planes: &Vec<(f32, f32, f32, f32)>,
) -> bool {
    for plane in frustrum_planes {
        if dotp(&[pos.0, pos.1, pos.2], &[plane.0, plane.1, plane.2]) + plane.3 <= -radius {
            // sphere not in frustrum
            return false;
        }
    }

    true
}

/// Helper function that converts viewing matrix into frustum planes
pub fn get_frustum_planes(matrix: &Matrix4) -> Vec<(f32, f32, f32, f32)> {
    let mut planes = Vec::new();

    // column-major
    // Left clipping plane
    planes.push((
        matrix[3][0] + matrix[0][0],
        matrix[3][1] + matrix[0][1],
        matrix[3][2] + matrix[0][2],
        matrix[3][3] + matrix[0][3],
    ));
    // Right clipping plane
    planes.push((
        matrix[3][0] - matrix[0][0],
        matrix[3][1] - matrix[0][1],
        matrix[3][2] - matrix[0][2],
        matrix[3][3] - matrix[0][3],
    ));
    // Top clipping plane
    planes.push((
        matrix[3][0] - matrix[1][0],
        matrix[3][1] - matrix[1][1],
        matrix[3][2] - matrix[1][2],
        matrix[3][3] - matrix[1][3],
    ));
    // Bottom clipping plane
    planes.push((
        matrix[3][0] + matrix[1][0],
        matrix[3][1] + matrix[1][1],
        matrix[3][2] + matrix[1][2],
        matrix[3][3] + matrix[1][3],
    ));
    // Near clipping plane
    planes.push((
        matrix[3][0] + matrix[2][0],
        matrix[3][1] + matrix[2][1],
        matrix[3][2] + matrix[2][2],
        matrix[3][3] + matrix[2][3],
    ));
    // Far clipping plane
    planes.push((
        matrix[3][0] - matrix[2][0],
        matrix[3][1] - matrix[2][1],
        matrix[3][2] - matrix[2][2],
        matrix[3][3] - matrix[2][3],
    ));

    planes
}
