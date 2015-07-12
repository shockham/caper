pub mod dist;
pub mod pbr;

use glium::{ Program, Display };

pub struct Shaders {
    pub shaders: Vec<Program>,
}

impl Shaders {
    /// Creates a new instance of Shaders
    pub fn new(display: &Display) -> Shaders {
        // the shader programs
        let program_dist = match Program::from_source(display, dist::vert(), dist::frag(), Some(dist::geom())) {
            Ok(p) => p,
            Err(e) => panic!("glsl error: {}", e), 
        };

        let program_pbr = match Program::from_source(display, pbr::vert(), pbr::frag(), Some(pbr::geom())) {
            Ok(p) => p,
            Err(e) => panic!("glsl error: {}", e), 
        };
       
        Shaders {
            shaders: vec![program_dist, program_pbr]
        }
    }
}
