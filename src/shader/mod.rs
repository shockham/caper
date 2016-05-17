pub mod default;
pub mod dist;
pub mod pbr;
pub mod height;
pub mod height_tess;
pub mod line;

use glium::{ Program, Display };
use std::collections::HashMap;

pub struct Shaders {
    pub shaders: HashMap<&'static str, Program>,
    pub post_shaders: HashMap<&'static str, Program>,
}

impl Shaders {
    /// Creates a new instance of Shaders
    pub fn new(display: &Display) -> Shaders {

        let mut shader_map = HashMap::new();

        // the shader programs
        shader_map.insert("dist", program!(display,
                                    330 => {
                                        vertex: default::gl330::VERT,
                                        fragment: dist::gl330::FRAG,
                                        geometry: default::gl330::GEOM,
                                        tessellation_control: default::gl330::TESS_CONTROL,
                                        tessellation_evaluation: default::gl330::TESS_EVAL
                                    }).unwrap());

        shader_map.insert("pbr", program!(display,
                                   330 => {
                                       vertex: default::gl330::VERT,
                                       fragment: pbr::gl330::FRAG,
                                       geometry:default::gl330::GEOM,
                                       tessellation_control: default::gl330::TESS_CONTROL,
                                       tessellation_evaluation: default::gl330::TESS_EVAL
                                   }).unwrap());

        shader_map.insert("height", program!(display,
                                      330 => {
                                          vertex: default::gl330::VERT,
                                          fragment: height::gl330::FRAG,
                                          geometry: default::gl330::GEOM,
                                          tessellation_control: default::gl330::TESS_CONTROL,
                                          tessellation_evaluation: default::gl330::TESS_EVAL
                                      }).unwrap());

        shader_map.insert("height_tess", program!(display,
                                           330 => {
                                               vertex: default::gl330::VERT,
                                               fragment: height_tess::gl330::FRAG,
                                               geometry: height_tess::gl330::GEOM,
                                               tessellation_control: height_tess::gl330::TESS_CONTROL,
                                               tessellation_evaluation: height_tess::gl330::TESS_EVAL
                                           }).unwrap());

        shader_map.insert("line", program!(display,
                                    330 => {
                                        vertex: default::gl330::VERT,
                                        fragment: line::gl330::FRAG,
                                        geometry: line::gl330::GEOM,
                                        tessellation_control: default::gl330::TESS_CONTROL,
                                        tessellation_evaluation: default::gl330::TESS_EVAL
                                    }).unwrap());

        let mut post_shader_map = HashMap::new();
            
        post_shader_map.insert("chrom", program!(display,
                             330 => {
                                 vertex: r"
                            #version 330

                            layout(location = 0) in vec3 position;
                            layout(location = 1) in vec2 texture;

                            out vec2 v_tex_coords;

                            void main() {
                                gl_Position = vec4(position, 1.0);
                                v_tex_coords = texture;
                            }
                        ",
                        fragment: r"
                            #version 330

                            uniform vec2 resolution;
                            uniform sampler2D tex;

                            in vec2 v_tex_coords;

                            out vec4 frag_output;

                            void main() {
                                vec4 color = texture(tex, v_tex_coords);
                                //ivec2 tex_size = textureSize(tex, 0);
                                vec2 tex_size = vec2(0.997);

                                color.r = texture(tex, vec2(min(v_tex_coords.x + 0.003, tex_size.x), v_tex_coords.y)).r;
                                color.b = texture(tex, vec2(v_tex_coords.x, min(v_tex_coords.y + 0.003, tex_size.y))).b;

                                frag_output = color;
                            }
                        "
                             }
            ).unwrap());

        Shaders {
            shaders: shader_map,
            post_shaders: post_shader_map,
        }
    }
}
