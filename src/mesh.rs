use noise::{NoiseFn, Perlin, Seedable};
use std::f32::consts::PI;
use types::Vertex;
use utils::calc_normal;

/// The default normal to give a mesh vertex
pub const DEF_NORMAL: [f32; 3] = [0f32, 0f32, 0f32];
/// The default uv coordinate to give a mesh vertex
pub const DEF_UV: [f32; 2] = [0f32, 0f32];
const PI2: f32 = PI * 2f32;
/// The default seed base for creating a perlin mesh
pub const DEF_SEED_BASE: u32 = 0;

lazy_static! {
    /// static ref to generate perlin noise from
    static ref PERLIN:Perlin = {
        Perlin::new().set_seed(DEF_SEED_BASE)
    };
}

/// Generates a quad mesh with each side length 1
pub fn gen_quad() -> Vec<Vertex> {
    let p_00 = [-0.5f32, -0.5f32, -0.5f32];
    let p_01 = [-0.5f32, 0.5f32, -0.5f32];
    let p_10 = [0.5f32, -0.5f32, -0.5f32];
    let p_11 = [0.5f32, 0.5f32, -0.5f32];

    vec![
        Vertex {
            position: p_00,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32],
        },
        Vertex {
            position: p_01,
            normal: DEF_NORMAL,
            texture: [1f32, 1f32],
        },
        Vertex {
            position: p_11,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32],
        },
        Vertex {
            position: p_00,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32],
        },
        Vertex {
            position: p_11,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32],
        },
        Vertex {
            position: p_10,
            normal: DEF_NORMAL,
            texture: [0f32, 0f32],
        },
    ]
}

///Generates a cube mesh
pub fn gen_cube() -> Vec<Vertex> {
    let p_000 = [-0.5f32, -0.5f32, -0.5f32];
    let p_010 = [-0.5f32, 0.5f32, -0.5f32];
    let p_100 = [0.5f32, -0.5f32, -0.5f32];
    let p_110 = [0.5f32, 0.5f32, -0.5f32];
    let p_001 = [-0.5f32, -0.5f32, 0.5f32];
    let p_011 = [-0.5f32, 0.5f32, 0.5f32];
    let p_101 = [0.5f32, -0.5f32, 0.5f32];
    let p_111 = [0.5f32, 0.5f32, 0.5f32];

    let mut cube_verts = gen_quad();

    cube_verts.append(&mut vec![
        // back face TODO fix line rendering
        Vertex {
            position: p_101,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32],
        },
        Vertex {
            position: p_011,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32],
        },
        Vertex {
            position: p_001,
            normal: DEF_NORMAL,
            texture: [0f32, 0f32],
        },
        Vertex {
            position: p_101,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32],
        },
        Vertex {
            position: p_111,
            normal: DEF_NORMAL,
            texture: [1f32, 1f32],
        },
        Vertex {
            position: p_011,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32],
        },
        // bottom face
        Vertex {
            position: p_000,
            normal: DEF_NORMAL,
            texture: [0f32, 0f32],
        },
        Vertex {
            position: p_100,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32],
        },
        Vertex {
            position: p_001,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32],
        },
        Vertex {
            position: p_100,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32],
        },
        Vertex {
            position: p_101,
            normal: DEF_NORMAL,
            texture: [1f32, 1f32],
        },
        Vertex {
            position: p_001,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32],
        },
        // top face
        Vertex {
            position: p_110,
            normal: DEF_NORMAL,
            texture: [1f32, 1f32],
        },
        Vertex {
            position: p_010,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32],
        },
        Vertex {
            position: p_011,
            normal: DEF_NORMAL,
            texture: [0f32, 0f32],
        },
        Vertex {
            position: p_111,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32],
        },
        Vertex {
            position: p_110,
            normal: DEF_NORMAL,
            texture: [1f32, 1f32],
        },
        Vertex {
            position: p_011,
            normal: DEF_NORMAL,
            texture: [0f32, 0f32],
        },
        // left face
        Vertex {
            position: p_010,
            normal: DEF_NORMAL,
            texture: [1f32, 1f32],
        },
        Vertex {
            position: p_000,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32],
        },
        Vertex {
            position: p_001,
            normal: DEF_NORMAL,
            texture: [0f32, 0f32],
        },
        Vertex {
            position: p_011,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32],
        },
        Vertex {
            position: p_010,
            normal: DEF_NORMAL,
            texture: [1f32, 1f32],
        },
        Vertex {
            position: p_001,
            normal: DEF_NORMAL,
            texture: [0f32, 0f32],
        },
        // right face
        Vertex {
            position: p_100,
            normal: DEF_NORMAL,
            texture: [0f32, 0f32],
        },
        Vertex {
            position: p_110,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32],
        },
        Vertex {
            position: p_101,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32],
        },
        Vertex {
            position: p_110,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32],
        },
        Vertex {
            position: p_111,
            normal: DEF_NORMAL,
            texture: [1f32, 1f32],
        },
        Vertex {
            position: p_101,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32],
        },
    ]);

    cube_verts
}

/// Generates a sphere mesh
pub fn gen_sphere() -> Vec<Vertex> {
    gen_sphere_segments(10f32, 10f32)
}

/// Generates a sphere mesh with segments and rings specified
pub fn gen_sphere_segments(segs: f32, rings: f32) -> Vec<Vertex> {
    assert!(
        segs > 1.0 && rings > 1.0,
        "segs and rings needs to be greater than 1"
    );

    let mut vertices = Vec::new();

    for m in 0..rings as i32 {
        for n in 0..segs as i32 {
            let r = m as f32;
            let s = n as f32;
            let r_plus = r + 1.0;
            let s_plus = s + 1.0;

            let mut verts: Vec<[f32; 3]> = Vec::new();

            // skip first triangle on first ring
            if r > 0.0 {
                verts.push([
                    (PI * r / rings).sin() * (PI2 * s_plus / segs).cos(),
                    (PI * r / rings).cos(),
                    (PI * r / rings).sin() * (PI2 * s_plus / segs).sin(),
                ]);
                verts.push([
                    (PI * r_plus / rings).sin() * (PI2 * s / segs).cos(),
                    (PI * r_plus / rings).cos(),
                    (PI * r_plus / rings).sin() * (PI2 * s / segs).sin(),
                ]);
                verts.push([
                    (PI * r / rings).sin() * (PI2 * s / segs).cos(),
                    (PI * r / rings).cos(),
                    (PI * r / rings).sin() * (PI2 * s / segs).sin(),
                ]);
            }

            // skip last triangle on last ring
            if r < rings - 1.0 {
                verts.push([
                    (PI * r_plus / rings).sin() * (PI2 * s_plus / segs).cos(),
                    (PI * r_plus / rings).cos(),
                    (PI * r_plus / rings).sin() * (PI2 * s_plus / segs).sin(),
                ]);
                verts.push([
                    (PI * r_plus / rings).sin() * (PI2 * s / segs).cos(),
                    (PI * r_plus / rings).cos(),
                    (PI * r_plus / rings).sin() * (PI2 * s / segs).sin(),
                ]);
                verts.push([
                    (PI * r / rings).sin() * (PI2 * s_plus / segs).cos(),
                    (PI * r / rings).cos(),
                    (PI * r / rings).sin() * (PI2 * s_plus / segs).sin(),
                ]);
            }

            let normal = calc_normal(verts[0], verts[1], verts[2]);

            // create each Vertex from the verts vec
            for v in verts {
                vertices.push(Vertex {
                    position: v,
                    normal,
                    texture: [(v[0] + 1f32) / 2f32, (v[1] + 1f32) / 2f32],
                });
            }
        }
    }

    vertices
}

/// Set the seed for perlin generation
pub fn set_perlin_seed(seed: u32) {
    PERLIN.set_seed(seed);
}

/// Get a height for a pos p using perlin noise
pub fn get_pos_perlin(p: (f32, f32)) -> f32 {
    PERLIN
        .get([f64::from(p.0) / 15f64, f64::from(p.1) / 15f64])
        .abs() as f32 * 6f32
}

/// Generates a perlin mesh from pseu_pos with each side of vert length map_size
pub fn gen_perlin_mesh(pseu_pos: (f32, f32), map_size: f32) -> Vec<Vertex> {
    gen_proc_mesh(pseu_pos, map_size, get_pos_perlin)
}

/// Generates a perlin mesh from pseu_pos with each side of vert length map_size using seed
pub fn gen_seed_perlin_mesh(pseu_pos: (f32, f32), map_size: f32) -> Vec<Vertex> {
    gen_proc_mesh(pseu_pos, map_size, get_pos_perlin)
}

/// Macro to speed up gen_proc_mesh
macro_rules! push_vertices {
    ( $vec:ident, $( $x:expr, )* ) => {
        {
            $(
                $vec.push(Vertex {
                    position: $x,
                    normal: DEF_NORMAL,
                    texture: DEF_UV,
                });
            )*
        }
    };
}

/// Generate a procedural function used to calculate a vertex
pub fn gen_proc_mesh(
    pseu_pos: (f32, f32),
    map_size: f32,
    gen_fn: fn((f32, f32)) -> f32,
) -> Vec<Vertex> {
    // generate the instance positions
    let mut vertices = Vec::new();

    let point_total = (map_size * map_size) as i32;

    // get all heights for first chunk
    let mut size_00 = gen_fn((0f32, 0f32));
    let mut size_10;
    let mut size_01 = gen_fn((0f32, 0f32));
    let mut size_11;

    for i in 0..point_total {
        let pos = ((i as f32 % map_size), (i / map_size as i32) as f32);
        let p_pos = (pos.0 + pseu_pos.0, pos.1 + pseu_pos.1);

        // get the heights of the next two corners
        size_10 = gen_fn((p_pos.0 + 1f32, p_pos.1));
        size_11 = gen_fn((p_pos.0 + 1f32, p_pos.1 + 1f32));

        // create the two tris for this chunk
        push_vertices![
            vertices,
            [pos.0 + 1f32, size_10, pos.1],
            [pos.0, size_00, pos.1],
            [pos.0 + 1f32, size_11, pos.1 + 1f32],
            [pos.0, size_00, pos.1],
            [pos.0, size_01, pos.1 + 1f32],
            [pos.0 + 1f32, size_11, pos.1 + 1f32],
        ];

        // reuse calculated heights for efficiency
        size_00 = size_10;
        size_01 = size_11;
    }

    vertices
}
