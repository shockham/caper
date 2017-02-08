pub mod default;
pub mod dist;
pub mod height;
pub mod line;

use glium::{ Program, Display };
use std::collections::HashMap;
use std::error::Error;

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

        shader_map.insert("height", program!(display,
                                      330 => {
                                          vertex: default::gl330::VERT,
                                          fragment: height::gl330::FRAG,
                                          geometry: default::gl330::GEOM,
                                          tessellation_control: default::gl330::TESS_CONTROL,
                                          tessellation_evaluation: default::gl330::TESS_EVAL
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

        post_shader_map.insert("default", program!(display,
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

    pub fn add_shader(&mut self, display: &Display,
                      name: &'static str, vert: &'static str,
                      frag: &'static str, geom: &'static str,
                      tess_cont: &'static str, tess_eval: &'static str) -> Result<&str, &str> {

        let shader_prog = match program!(display,
                                      330 => {
                                          vertex: vert,
                                          fragment: frag,
                                          geometry: geom,
                                          tessellation_control: tess_cont,
                                          tessellation_evaluation: tess_eval,
                                      }) {

            Ok(s) => s,
            Err(e) => {
                println!("{}", e.cause().unwrap());
                return Err("Could not create shader");
            },
        };

        self.shaders.insert(name, shader_prog);

        Ok("shader added")
    }

    pub fn add_post_shader(&mut self, display: &Display,
                           name: &'static str, vert: &'static str,
                           frag: &'static str) -> Result<&str, &str> {

        let post_shader_prog = match program!(display,
                             330 => {
                                 vertex: vert,
                                 fragment: frag
                             }) {
            Ok(s) => s,
            Err(e) => {
                println!("{}", e.cause().unwrap());
                return Err("Could not create post shader");
            },
        };

        self.post_shaders.insert(name, post_shader_prog);

        Ok("post shader added")
    }
}
