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
        let program_dist = match program!(display,
                                    330 => {
                                        vertex: dist::gl330::vert(),
                                        fragment: dist::gl330::frag(),
                                        geometry: dist::gl330::geom()
                                    },
                                    110 => {
                                        vertex: dist::gl110::vert(),
                                        fragment: dist::gl110::frag()
                                    }) {
            Ok(p) => p,
            Err(e) => panic!("glsl error: {}", e), 
        };

        let program_pbr = match program!(display,
                                    330 => {
                                        vertex: pbr::gl330::vert(),
                                        fragment: pbr::gl330::frag(),
                                        geometry: pbr::gl330::geom()
                                    },
                                    110 => {
                                        vertex: pbr::gl110::vert(),
                                        fragment: pbr::gl110::frag()
                                    }) {
            Ok(p) => p,
            Err(e) => panic!("glsl error: {}", e), 
        };
       
        Shaders {
            shaders: vec![program_dist, program_pbr]
        }
    }
}
