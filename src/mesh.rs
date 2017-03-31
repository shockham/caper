use utils::calc_normal;
use types::Vertex;
use noise::{ perlin2, Seed };
use std::f32::consts::PI;

/// The default normal to give a mesh vertex
pub const DEF_NORMAL:[f32; 3] = [0f32, 0f32, 0f32];
/// The default uv coordinate to give a mesh vertex
pub const DEF_UV:[f32; 2] = [0f32, 0f32];
const PI2:f32 = PI * 2f32;
/// The default seed base for creating a perlin mesh
pub const DEF_SEED_BASE:u32 = 0;

/// Generates a quad mesh with each side length 1
pub fn gen_quad() -> Vec<Vertex> {
    let p_00 = [-0.5f32, -0.5f32, -0.5f32];
    let p_01 = [-0.5f32, 0.5f32, -0.5f32];
    let p_10 = [0.5f32, -0.5f32, -0.5f32];
    let p_11 = [0.5f32, 0.5f32, -0.5f32];

    vec!(
        Vertex {
            position: p_00,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32]
        },
        Vertex {
            position: p_01,
            normal: DEF_NORMAL,
            texture: [1f32, 1f32]
        },
        Vertex {
            position: p_11,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32]
        },
        Vertex {
            position: p_00,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32]
        },
        Vertex {
            position: p_11,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32]
        },
        Vertex {
            position: p_10,
            normal: DEF_NORMAL,
            texture: [0f32, 0f32]
        }
    )
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

    cube_verts.append(&mut vec!(
        // back face TODO fix line rendering
        Vertex {
            position: p_101,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32]
        },
         Vertex {
            position: p_011,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32]
        },
        Vertex {
            position: p_001,
            normal: DEF_NORMAL,
            texture: [0f32, 0f32]
        },
        Vertex {
            position: p_101,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32]
        },
        Vertex {
            position: p_111,
            normal: DEF_NORMAL,
            texture: [1f32, 1f32]
        },
        Vertex {
            position: p_011,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32]
        },
        // bottom face
        Vertex {
            position: p_000,
            normal: DEF_NORMAL,
            texture: [0f32, 0f32]
        },
        Vertex {
            position: p_100,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32]
        },
        Vertex {
            position: p_001,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32]
        },
        Vertex {
            position: p_100,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32]
        },
        Vertex {
            position: p_101,
            normal: DEF_NORMAL,
            texture: [1f32, 1f32]
        },
        Vertex {
            position: p_001,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32]
        },
        // top face
        Vertex {
            position: p_110,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32]
        },
        Vertex {
            position: p_010,
            normal: DEF_NORMAL,
            texture: [0f32, 0f32]
        },
        Vertex {
            position: p_011,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32]
        },
        Vertex {
            position: p_111,
            normal: DEF_NORMAL,
            texture: [1f32, 1f32]
        },
        Vertex {
            position: p_110,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32]
        },
        Vertex {
            position: p_011,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32]
        },
        // left face
        Vertex {
            position: p_010,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32]
        },
        Vertex {
            position: p_000,
            normal: DEF_NORMAL,
            texture: [0f32, 0f32]
        },
        Vertex {
            position: p_001,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32]
        },
        Vertex {
            position: p_011,
            normal: DEF_NORMAL,
            texture: [1f32, 1f32]
        },
        Vertex {
            position: p_010,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32]
        },
        Vertex {
            position: p_001,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32]
        },
        // right face
        Vertex {
            position: p_100,
            normal: DEF_NORMAL,
            texture: [0f32, 0f32]
        },
        Vertex {
            position: p_110,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32]
        },
        Vertex {
            position: p_101,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32]
        },
        Vertex {
            position: p_110,
            normal: DEF_NORMAL,
            texture: [1f32, 0f32]
        },
        Vertex {
            position: p_111,
            normal: DEF_NORMAL,
            texture: [1f32, 1f32]
        },
        Vertex {
            position: p_101,
            normal: DEF_NORMAL,
            texture: [0f32, 1f32]
        },
    ));

    cube_verts
}

/// Generates a sphere mesh
pub fn gen_sphere() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let segs = 10f32;

    for m in 0 .. segs as i32 {
        for n in 0 .. segs as i32 {
            let n_plus = n + 1;
            let m_plus = m + 1;
            let verts = vec!(
                [(PI * m as f32/segs).sin() * (PI2 * n as f32/segs).cos(),
                (PI * m as f32/segs).sin() * (PI2 * n as f32/segs).sin(),
                (PI * m as f32/segs).cos()],
                [(PI * m_plus as f32/segs).sin() * (PI2 * n as f32/segs).cos(),
                (PI * m_plus as f32/segs).sin() * (PI2 * n as f32/segs).sin(),
                (PI * m_plus as f32/segs).cos()],
                [(PI * m as f32/segs).sin() * (PI2 * n_plus as f32/segs).cos(),
                (PI * m as f32/segs).sin() * (PI2 * n_plus as f32/segs).sin(),
                (PI * m as f32/segs).cos()],

                [(PI * m as f32/segs).sin() * (PI2 * n_plus as f32/segs).cos(),
                (PI * m as f32/segs).sin() * (PI2 * n_plus as f32/segs).sin(),
                (PI * m as f32/segs).cos()],
                [(PI * m_plus as f32/segs).sin() * (PI2 * n as f32/segs).cos(),
                (PI * m_plus as f32/segs).sin() * (PI2 * n as f32/segs).sin(),
                (PI * m_plus as f32/segs).cos()],
                [(PI * m_plus as f32/segs).sin() * (PI2 * n_plus as f32/segs).cos(),
                (PI * m_plus as f32/segs).sin() * (PI2 * n_plus as f32/segs).sin(),
                (PI * m_plus as f32/segs).cos()]);

            let normal = calc_normal(verts[0], verts[1], verts[2]);

            // create each Vertex from the verts vec
            for v in verts {
                vertices.push(Vertex {
                    position: v,
                    normal: normal,
                    texture: [v[0], v[1]],
                });
            }
        }
    }

    vertices
}

/// Get a height for a pos p using perlin noise
pub fn get_pos_perlin(p:(f32, f32), seed: &Seed) -> f32 {
    perlin2(seed, &[p.0 / 15f32, p.1 / 15f32]).abs() * 6f32
}

/// Generates a perlin mesh from pseu_pos with each side of vert length map_size
pub fn gen_perlin_mesh(pseu_pos: (f32, f32), map_size: f32) -> Vec<Vertex> {
    gen_proc_mesh(pseu_pos, map_size, &Seed::new(DEF_SEED_BASE), get_pos_perlin)
}

/// Generates a perlin mesh from pseu_pos with each side of vert length map_size using seed
pub fn gen_seed_perlin_mesh(pseu_pos: (f32, f32), map_size: f32, seed: &Seed) -> Vec<Vertex> {
    gen_proc_mesh(pseu_pos, map_size, seed, get_pos_perlin)
}

/// Generate a procedural function used to calculate a vertex
pub fn gen_proc_mesh(pseu_pos: (f32, f32), map_size: f32,
                     seed: &Seed, gen_fn: fn((f32, f32), &Seed) -> f32) -> Vec<Vertex> {
    // generate the instance positions
    let mut vertices = Vec::new();

    let point_total = (map_size * map_size) as i32;

    // get all heights for first chunk
    let mut size_00 = gen_fn((0f32, 0f32), seed);
    let mut size_10;
    let mut size_01 = gen_fn((0f32, 0f32), seed);
    let mut size_11;

    for i in 0 .. point_total {
        let pos = ((i as f32 % map_size), (i / map_size as i32) as f32);
        let p_pos = (pos.0 + pseu_pos.0, pos.1 + pseu_pos.1);

        // get the heights of the next two corners
        size_10 = gen_fn((p_pos.0 + 1f32, p_pos.1), seed);
        size_11 = gen_fn((p_pos.0 + 1f32, p_pos.1 + 1f32), seed);

        // create the two tris for this chunk
        let verts = vec!(
            [pos.0 + 1f32, size_10, pos.1],
            [pos.0, size_00, pos.1],
            [pos.0 + 1f32, size_11, pos.1 + 1f32],
            [pos.0, size_00, pos.1],
            [pos.0 , size_01, pos.1 + 1f32],
            [pos.0 + 1f32, size_11, pos.1 + 1f32]);

        //let calc_normal = calc_normal(verts[0], verts[1], verts[2]);

        // create each Vertex from the verts vec
        for v in verts {
            vertices.push(Vertex {
                position: v,
                normal: DEF_NORMAL,
                texture: DEF_UV
            });
        }

        // reuse calculated heights for efficiency
        size_00 = size_10;
        size_01 = size_11;
    }

    vertices
}
