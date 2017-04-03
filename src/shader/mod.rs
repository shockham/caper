/// Default shaders
pub mod default;
/// Distance shaders for roll off to white in relation to the camera position
pub mod dist;
/// Height shaders for rolling of to white in relation to the y-axis
pub mod height;
/// Line shaders for rendering wireframes
pub mod line;
/// Shader for rendering with textures
pub mod texture;

use glium::{ Program, Display };
use glium::texture::compressed_srgb_texture2d::CompressedSrgbTexture2d;
use std::collections::HashMap;
use std::error::Error;


/// Contains all the shaders to be used
pub struct Shaders {
    /// Map containing all the shaders used to render the RenderItems
    pub shaders: HashMap<&'static str, Program>,
    /// Map containing all the shaders to be used for post processing
    pub post_shaders: HashMap<&'static str, Program>,
    /// Texture that can be passed to the shaders
    pub textures: HashMap<&'static str, CompressedSrgbTexture2d>,
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

        // the shader programs
        shader_map.insert("texture", program!(display,
                                    330 => {
                                        vertex: default::gl330::VERT,
                                        fragment: texture::gl330::FRAG,
                                        geometry: default::gl330::GEOM,
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

        let mut textures = HashMap::new();

        textures.insert("default", load_texture!("../resources/caper.png", display));
        textures.insert("default_normal", load_texture!("../resources/normal.png", display));

        Shaders {
            shaders: shader_map,
            post_shaders: post_shader_map,
            textures: textures,
        }
    }

    /// Add a new shader to the map that can used for rendering the RenderItems
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

    /// Add a new shader to the post_shaders map that can be used for rendering post processing
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
