pub mod default;
pub mod dist;
pub mod pbr;
pub mod height;
pub mod height_tess;
pub mod line;

use glium::{ Program, Display };

pub struct Shaders {
    pub shaders: Vec<Program>,
}

impl Shaders {
    /// Creates a new instance of Shaders
    pub fn new(display: &Display) -> Shaders {
        // the shader programs
        let program_dist = program!(display,
                                    330 => {
                                        vertex: default::gl330::VERT,
                                        fragment: dist::gl330::FRAG,
                                        geometry: default::gl330::GEOM,
                                        tessellation_control: default::gl330::TESS_CONTROL,
                                        tessellation_evaluation: default::gl330::TESS_EVAL
                                    },
                                    110 => {
                                        vertex: default::gl110::VERT,
                                        fragment: dist::gl110::FRAG
                                    }).unwrap();

        let program_pbr = program!(display,
                                   330 => {
                                       vertex: default::gl330::VERT,
                                       fragment: pbr::gl330::FRAG,
                                       geometry:default::gl330::GEOM,
                                       tessellation_control: default::gl330::TESS_CONTROL,
                                       tessellation_evaluation: default::gl330::TESS_EVAL
                                   },
                                   110 => {
                                       vertex: default::gl110::VERT,
                                       fragment: pbr::gl110::FRAG
                                   }).unwrap();

        let program_height = program!(display,
                                      330 => {
                                          vertex: default::gl330::VERT,
                                          fragment: height::gl330::FRAG,
                                          geometry: default::gl330::GEOM,
                                          tessellation_control: default::gl330::TESS_CONTROL,
                                          tessellation_evaluation: default::gl330::TESS_EVAL
                                      },
                                      110 => {
                                          vertex: default::gl110::VERT,
                                          fragment: height::gl110::FRAG
                                      }).unwrap();

        let program_height_tess = program!(display,
                                           330 => {
                                               vertex: default::gl330::VERT,
                                               fragment: height_tess::gl330::FRAG,
                                               geometry: default::gl330::GEOM,
                                               tessellation_control: height_tess::gl330::TESS_CONTROL,
                                               tessellation_evaluation: height_tess::gl330::TESS_EVAL
                                           },
                                           110 => {
                                               vertex: default::gl110::VERT,
                                               fragment: height_tess::gl110::FRAG
                                           }).unwrap();

        // the shader programs
        let program_line = program!(display,
                                    330 => {
                                        vertex: default::gl330::VERT,
                                        fragment: line::gl330::FRAG,
                                        geometry: line::gl330::GEOM,
                                        tessellation_control: height_tess::gl330::TESS_CONTROL,
                                        tessellation_evaluation: height_tess::gl330::TESS_EVAL
                                    },
                                    110 => {
                                        vertex: default::gl110::VERT,
                                        fragment: line::gl110::FRAG
                                    }).unwrap();

        Shaders {
            shaders: vec![program_dist, program_pbr, program_height, program_height_tess, program_line]
        }
    }
}
