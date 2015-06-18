
use glutin::{ WindowBuilder };
use glutin::CursorState::{ Grab, Hide };

use glium::{ Display, DrawParameters, DisplayBuild, Surface, Program };
use glium::index::NoIndices;
use glium::index::PrimitiveType::TrianglesList;
use glium::DepthTest::IfLess;
use glium::vertex::VertexBuffer;

use utils::*;
use shader::*;
use std::default::Default;

pub struct Renderer {
    pub display: Display,
}

impl Renderer {
    pub fn new() -> Renderer {    

        Renderer {
            display: WindowBuilder::new()
                .with_depth_buffer(24)
                .with_multisampling(16)
                .with_title("caper".to_string())
                .build_glium()
                .unwrap(),
        }
    }

    pub fn setup(&self) {
        // get the window for various values
        let window = self.display.get_window().unwrap();
        window.set_cursor_state(Grab).ok();
        window.set_cursor_state(Hide).ok();
    }

    pub fn get_window_size(&self) -> (u32, u32) { 
        self.display.get_window()
            .unwrap()
            .get_inner_size()
            .unwrap_or((800, 600))

    }

    pub fn draw(&self, cam_pos: [f32; 3], cam_rot: [f32; 3], vertex_data: &mut Vec<Vertex>){
        // possibly set this to an event
        let (width, height) = self.get_window_size(); 

        // draw parameters
        let params = DrawParameters {
            depth_test: IfLess,
            depth_write: true,
            .. Default::default()
        };

        let uniforms = uniform! {
            projection_matrix: build_persp_proj_mat(60f32, width as f32/height as f32, 0.01f32, 1000f32),
            modelview_matrix: build_fp_view_matrix(cam_pos, cam_rot),
        };

        // building the vertex and index buffers
        let vertex_buffer = VertexBuffer::new(&self.display, vertex_data);

        // the shader programs
        let program = match Program::from_source(&self.display,
                                                        dist::vert(),
                                                        dist::frag(),
                                                        Some(dist::geom())
                                                       ){
            Ok(p) => p,
            Err(e) => panic!("glsl error: {}", e), 
        };

        // drawing a frame
        let mut target = self.display.draw();
        target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);
        target.draw(&vertex_buffer,
                    &NoIndices(TrianglesList),
                    &program,
                    &uniforms, 
                    &params).unwrap();
        target.finish();
    }
}
