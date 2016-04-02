use utils::{ calc_normal, Vertex };
use noise::{ perlin2, Seed };
use std::f32::consts::PI;

const DEF_NORMAL:[f32; 3] = [0f32, 0f32, 0f32];
const DEF_UV:[f32; 2] = [0f32, 0f32];
const PI2:f32 = PI * 2f32;

/// Generates a quad mesh with each side length 1
pub fn gen_quad() -> Vec<Vertex> {
    vec!(
        Vertex {
            position: [0f32, 0f32, 0f32],
            normal: DEF_NORMAL,
            texture: [0f32, 0f32]
        },
        Vertex {
            position: [0f32, 1f32, 0f32],
            normal: DEF_NORMAL,
            texture: [0f32, 1f32]
        },
        Vertex {
            position: [1f32, 1f32, 0f32],
            normal: DEF_NORMAL,
            texture: [1f32, 1f32]
        },
        Vertex {
            position: [0f32, 0f32, 0f32],
            normal: DEF_NORMAL,
            texture: [0f32, 0f32]
        },
        Vertex {
            position: [1f32, 1f32, 0f32],
            normal: DEF_NORMAL,
            texture: [1f32, 1f32]
        },
        Vertex {
            position: [1f32, 0f32, 0f32],
            normal: DEF_NORMAL,
            texture: [1f32, 0f32]
        }
    )
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
                    texture: DEF_UV
                });
            }
        } 
    } 

    vertices
}

/// Generates a perlin mesh from pseu_pos with each side of vert length map_size
pub fn gen_perlin_mesh(pseu_pos: (f32, f32), map_size: f32) -> Vec<Vertex> {
    fn get_pos_perlin(p:(f32, f32), seed: &Seed) -> f32 {
        perlin2(seed, &[p.0 / 15f32, p.1 / 15f32]).abs() * 6f32
    };

    gen_proc_mesh(pseu_pos, map_size, &Seed::new(0), get_pos_perlin)
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
