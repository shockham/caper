pub mod dist;
pub mod pbr;
pub mod height;
pub mod height_tess;

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
                                        vertex: dist::gl330::VERT,
                                        fragment: dist::gl330::FRAG,
                                        geometry: dist::gl330::GEOM
                                    },
                                    110 => {
                                        vertex: dist::gl110::VERT,
                                        fragment: dist::gl110::FRAG
                                    }) {
            Ok(p) => p,
            Err(e) => panic!("glsl error: {}", e), 
        };

        let program_pbr = match program!(display,
                                    330 => {
                                        vertex: pbr::gl330::VERT,
                                        fragment: pbr::gl330::FRAG,
                                        geometry: pbr::gl330::GEOM
                                    },
                                    110 => {
                                        vertex: pbr::gl110::VERT,
                                        fragment: pbr::gl110::FRAG
                                    }) {
            Ok(p) => p,
            Err(e) => panic!("glsl error: {}", e), 
        };
        
        let program_height = match program!(display,
                                    330 => {
                                        vertex: height::gl330::VERT,
                                        fragment: height::gl330::FRAG,
                                        geometry: height::gl330::GEOM,
                                    },
                                    110 => {
                                        vertex: height::gl110::VERT,
                                        fragment: height::gl110::FRAG
                                    }) {
            Ok(p) => p,
            Err(e) => panic!("glsl error: {}", e), 
        };
        
        let program_height_tess = match program!(display,
                                    330 => {
                                        vertex: height_tess::gl330::VERT,
                                        fragment: height_tess::gl330::FRAG,
                                        geometry: height_tess::gl330::GEOM,
                                        tessellation_control: height_tess::gl330::TESS_CONTROL,
                                        tessellation_evaluation: height_tess::gl330::TESS_EVAL
                                    },
                                    110 => {
                                        vertex: height_tess::gl110::VERT,
                                        fragment: height_tess::gl110::FRAG
                                    }) {
            Ok(p) => p,
            Err(e) => panic!("glsl error: {}", e), 
        };
       
        Shaders {
            shaders: vec![program_dist, program_pbr, program_height, program_height_tess]
        }
    }
}
