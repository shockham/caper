use utils::{ calc_normal, Vertex };
use noise::{ perlin2, Seed };

/// Generates a quad with each side length 1
pub fn gen_quad() -> Vec<Vertex> {
    let def_normal = [0f32, 0f32, 0f32];
    
    vec!(
        Vertex {
            position: [0f32, 0f32, 0f32],
            normal: def_normal,
            texture: [0f32, 0f32]
        },
        Vertex {
            position: [0f32, 1f32, 0f32],
            normal: def_normal,
            texture: [0f32, 1f32]
        },
        Vertex {
            position: [1f32, 1f32, 0f32],
            normal: def_normal,
            texture: [1f32, 1f32]
        },
        Vertex {
            position: [0f32, 0f32, 0f32],
            normal: def_normal,
            texture: [0f32, 0f32]
        },
        Vertex {
            position: [1f32, 1f32, 0f32],
            normal: def_normal,
            texture: [1f32, 1f32]
        },
        Vertex {
            position: [1f32, 0f32, 0f32],
            normal: def_normal,
            texture: [1f32, 0f32]
        }
    )
}

/// Generates a perlin mesh from pseu_pos with each side of vert length map_size
pub fn gen_perlin_mesh(pseu_pos: (f32, f32), map_size: f32) -> Vec<Vertex> {
    // generate the instance positions 
    let mut vertices = Vec::new();

    let point_total = (map_size * map_size) as i32;
    let seed = Seed::new(0);

    // get all heights for first chunk 
    let mut size_00 = perlin2(&seed, &[0f32, 0f32]).abs() * 8f32;
    let mut size_10;
    let mut size_01 = perlin2(&seed, &[0f32, 0f32]).abs() * 8f32;
    let mut size_11;

    //let def_normal = [0f32, 0f32, 0f32];
    let def_uv = [0f32, 0f32];

    for i in 0 .. point_total {
        let pos = ((i as f32 % map_size), (i / map_size as i32) as f32);
        let p_pos = (pos.0 + pseu_pos.0, pos.1 + pseu_pos.1);

        // get the heights of the next two corners
        size_10 = perlin2(&seed, &[(p_pos.0 + 1f32) / 10f32, p_pos.1 / 10f32]).abs() * 8f32;
        size_11 = perlin2(&seed, 
                          &[(p_pos.0 + 1f32) / 10f32, (p_pos.1 + 1f32) / 10f32]).abs() * 8f32;

        // create the two tris for this chunk
        let verts = vec!(
            [pos.0 + 1f32, size_10, pos.1],
            [pos.0, size_00, pos.1],
            [pos.0 + 1f32, size_11, pos.1 + 1f32],
            [pos.0, size_00, pos.1],
            [pos.0 , size_01, pos.1 + 1f32],
            [pos.0 + 1f32, size_11, pos.1 + 1f32]);

        let calc_normal = calc_normal(verts[0], verts[1], verts[2]);
        
        // create each Vertex from the verts vec
        for v in verts {
            vertices.push(Vertex {
                position: v,
                normal: calc_normal,
                texture: def_uv
            });
        }

        // reuse calculated heights for efficiency
        size_00 = size_10;
        size_01 = size_11;
    }

    vertices
}
