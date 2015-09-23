use glium::{ Display, DrawParameters, DisplayBuild, Surface };
use glium::index::NoIndices;
use glium::index::PrimitiveType::TrianglesList;
use glium::DepthTest::IfLess;
use glium::vertex::VertexBuffer;
use glium::glutin::WindowBuilder;
use glium::glutin::CursorState::Hide;//{ Grab, Hide };
use glium::draw_parameters::BackfaceCullingMode::CullClockWise;

use utils::*;
use shader::Shaders;
use std::default::Default;
use std::f32::consts::PI;

pub const FIXED_TIME_STAMP: u64 = 16666667;

pub struct RenderItem {
    pub vertices: Vec<Vertex>,
    pub shader_index: usize,
    pub world_position: (f32, f32, f32),
}

#[derive(Copy, Clone)]
pub struct CamState {
    pub cam_pos:[f32; 3],
    pub cam_rot:[f32; 3]
}

pub struct Renderer {
    pub display: Display,
}

impl Renderer {
    /// Creates new Renderer instance
    pub fn new() -> Renderer {    
        Renderer {
            display: WindowBuilder::new()
                .with_depth_buffer(24)
                .with_multisampling(16)
                .with_title("caper".to_string())
                .with_vsync()
                .build_glium()
                .unwrap(),
        }
    }

    /// Sets up the render window
    pub fn setup(&self) {
        // get the window for various values
        let window = self.display.get_window().unwrap();
        //window.set_cursor_state(Grab).ok();
        window.set_cursor_state(Hide).ok();

    } 

    /// Draws a frame
    pub fn draw(&self, cam_state: CamState, render_items: &Vec<RenderItem>, shaders: &Shaders){
        // possibly set this to an event
        let (width, height) = self.display.get_framebuffer_dimensions(); 

        // draw parameters
        let params = DrawParameters {
            depth_test: IfLess,
            depth_write: true,
            backface_culling: CullClockWise,
            .. Default::default()
        };

        let uniforms = uniform! {
            projection_matrix: Renderer::build_persp_proj_mat(60f32, width as f32/height as f32, 0.01f32, 1000f32),
            modelview_matrix: Renderer::build_fp_view_matrix(cam_state.cam_pos, cam_state.cam_rot),
        };




        // drawing a frame
        let mut target = self.display.draw();
        target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);

        for item in render_items.iter() { 
            // building the vertex and index buffers
            let vertex_buffer = VertexBuffer::new(&self.display, &item.vertices).unwrap();

            // add positions for instances 
            let per_instance = {
                #[derive(Copy, Clone)]
                struct Attr {
                    world_position: (f32, f32, f32),
                }

                implement_vertex!(Attr, world_position);

                let data = vec![
                    Attr {
                        world_position: item.world_position,
                    }
                ];

                VertexBuffer::dynamic(&self.display, &data).unwrap()
            };

            target.draw((&vertex_buffer, per_instance.per_instance().unwrap()),
            &NoIndices(TrianglesList),
            &shaders.shaders[item.shader_index],
            &uniforms, 
            &params).unwrap();
        }

        match target.finish() {
            Ok(_) => {},
            Err(e) => println!("{:?}", e),
        };
    }

    /// Returns perspective matrix given fov, aspect ratio, z near and far
    pub fn build_persp_proj_mat(fov:f32,aspect:f32,znear:f32,zfar:f32) -> [[f32; 4]; 4] {
        let ymax = znear * (fov * (PI/360.0)).tan();
        let ymin = -ymax;
        let xmax = ymax * aspect;
        let xmin = ymin * aspect;

        let width = xmax - xmin;
        let height = ymax - ymin;

        let depth = zfar - znear;
        let q = -(zfar + znear) / depth;
        let qn = -2.0 * (zfar * znear) / depth;

        let w = 2.0 * znear / width;
        let h = 2.0 * znear / height;

        let mut m:[[f32; 4]; 4] = [[0.0f32; 4]; 4];
        m[0]  = [w, 0.0f32, 0.0f32, 0.0f32];
        m[1]  = [0.0f32, h, 0.0f32, 0.0f32];
        m[2]  = [0.0f32, 0.0f32, q, -1.0f32];
        m[3] = [0.0f32, 0.0f32, qn, 0.0f32];

        return m;
    }

    /// Returns the model view matrix for a first person view given cam position and rotation
    pub fn build_fp_view_matrix(cam_pos: [f32; 3], cam_rot: [f32; 3]) -> [[f32; 4]; 4] {

        let (sin_yaw, cos_yaw, sin_pitch, cos_pitch) = (cam_rot[1].sin(), cam_rot[1].cos(), cam_rot[0].sin(), cam_rot[0].cos());
        let xaxis = [cos_yaw, 0.0, -sin_yaw];
        let yaxis = [sin_yaw * sin_pitch, cos_pitch, cos_yaw * sin_pitch];
        let zaxis = [sin_yaw * cos_pitch, -sin_pitch, cos_pitch * cos_yaw];

        [
            [ xaxis[0], yaxis[0], zaxis[0], 0.0],
            [ xaxis[1], yaxis[1], zaxis[1], 0.0],
            [ xaxis[2], yaxis[2], zaxis[2], 0.0],
                [ dotp(&xaxis, &cam_pos), dotp(&yaxis, &cam_pos), dotp(&zaxis, &cam_pos), 1.0f32]
        ]

    }
}
