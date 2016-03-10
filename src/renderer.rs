use glium::{ Display, DrawParameters, DisplayBuild, Surface, Depth };
use glium::index::NoIndices;
use glium::index::PrimitiveType;
use glium::DepthTest::IfLess;
use glium::vertex::VertexBuffer;
use glium::glutin::{ WindowBuilder, get_primary_monitor };
use glium::glutin::CursorState::Hide;//{ Grab, Hide };
use glium::draw_parameters::BackfaceCullingMode::CullClockwise;

use glium_text;
use glium_text::{ TextSystem, FontTexture, TextDisplay };

use time;
use utils::*;
use shader::Shaders;
use std::default::Default;
use std::f32::consts::PI;

pub const FIXED_TIME_STAMP: u64 = 16666667;

/// type definition for a Vector3
pub type Vector3 = (f32, f32, f32);

/// type definition for a Quaternion
pub type Quaternion = (f32, f32, f32, f32);

/// struct for handling transform data
pub struct Transform {
    pub pos: Vector3,
    pub rot: Quaternion, 
    pub scale: Vector3,
    pub update_fn: Vec<fn(t:&mut Transform) -> ()>,
}

/// struct for abstracting items to be sent to render
pub struct RenderItem {
    pub vertices: Vec<Vertex>,
    pub shader_index: usize,
    pub instance_transforms: Vec<Transform>,
}

/// struct for abstacting text items to be rendered
pub struct TextItem {
    pub text: String,
    pub color: (f32, f32, f32, f32),
    pub pos: Vector3,
    pub update_fn: Vec<fn(ti:&mut TextItem) -> ()>,
}

/// trait for updateable entities
pub trait Entity {
    /// ran every frame
    fn update(&mut self) -> ();
}

/// implementation of Entity for Transform
impl Entity for Transform {
    fn update(&mut self) {
        for i in 0..self.update_fn.len() {
            self.update_fn[i](self);
        }
    }
}

/// implementation of Entity for TextItem
impl Entity for TextItem {
    fn update(&mut self) {
        for i in 0..self.update_fn.len() {
            self.update_fn[i](self);
        }
    }
}

/// struct for abstracting the camera state
#[derive(Copy, Clone)]
pub struct CamState {
    pub cam_pos:Vector3,
    pub cam_rot:Vector3
}

/// struct for shader attributes
#[derive(Copy, Clone)]
struct Attr {
    world_position: Vector3,
    world_rotation: Quaternion,
    world_scale: Vector3
}

/// struct for abstracting the render state
pub struct Renderer {
    pub display: Display,
    pub text_system: TextSystem,
    default_font: FontTexture,
    pub start_time: f64,
}

impl Renderer {
    /// Creates new Renderer instance
    pub fn new(title:String) -> Renderer {    
        // create a diplay instance
        let display = WindowBuilder::new()
            .with_depth_buffer(24)
            //.with_multisampling(16) // multisampling doesn't work on chromebook
            .with_title(title)
            .with_vsync()
            .with_fullscreen(get_primary_monitor())
            .build_glium()
            .unwrap();

        // create a text system instance and font
        let text_system = TextSystem::new(&display);
        let font = FontTexture::new(&display, &include_bytes!("./resources/font.otf")[..], 100).unwrap();

        let renderer = Renderer {
            display: display,
            text_system: text_system,
            default_font: font,
            start_time: time::precise_time_s(),
        };

        renderer.setup();

        renderer
    }

    /// Sets up the render window
    pub fn setup(&self) {
        // get the window for various values
        let window = self.display.get_window().unwrap();
        //window.set_cursor_state(Grab).ok();
        window.set_cursor_state(Hide).ok();
    } 

    /// Draws a frame
    pub fn draw(&self, cam_state: CamState, render_items: &Vec<RenderItem>, text_items: &Vec<TextItem>, shaders: &Shaders){
        // possibly set this to an event
        let (width, height) = self.display.get_framebuffer_dimensions(); 

        // draw parameters
        let params = DrawParameters {
            depth: Depth {
                test: IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: CullClockwise,
            .. Default::default()
        };

        let uniforms = uniform! {
            projection_matrix: Renderer::build_persp_proj_mat(60f32, width as f32/height as f32, 0.01f32, 1000f32),
            modelview_matrix: Renderer::build_fp_view_matrix(cam_state),
            cam_pos: cam_state.cam_pos,
            time: (time::precise_time_s() - self.start_time) as f32,
        };

        // drawing a frame
        let mut target = self.display.draw();
        target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);

        // drawing the render items TODO batching
        for item in render_items.iter() { 
            // building the vertex and index buffers
            let vertex_buffer = VertexBuffer::new(&self.display, &item.vertices).unwrap();

            // add positions for instances 
            let per_instance = {
                implement_vertex!(Attr, world_position, world_rotation, world_scale);

                let data = item.instance_transforms.iter().map(|t| {
                    Attr {
                        world_position: t.pos,
                        world_rotation: t.rot,
                        world_scale: t.scale
                    }
                }).collect::<Vec<_>>();

                VertexBuffer::dynamic(&self.display, &data).unwrap()
            };

            target.draw((&vertex_buffer, per_instance.per_instance().unwrap()),
                &NoIndices(PrimitiveType::Patches { vertices_per_patch: 3 }),
                //&NoIndices(TrianglesList),
                &shaders.shaders[item.shader_index],
                &uniforms, 
                &params).unwrap();
        }

        // drawing the text items
        for text_item in text_items.iter() {
            // create the matrix for the text
            let matrix = [
                [0.02, 0.0, 0.0, 0.0],
                [0.0, 0.02 * (width as f32) / (height as f32), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [text_item.pos.0, text_item.pos.1, text_item.pos.2, 1.0f32]
            ];

            // create TextDisplay for item, TODO change this to not be done every frame
            let text = TextDisplay::new(&self.text_system, &self.default_font, text_item.text.as_str());

            glium_text::draw(&text,
                             &self.text_system,
                             &mut target,
                             matrix,
                             text_item.color);
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

        [
            [w, 0.0f32, 0.0f32, 0.0f32],
            [0.0f32, h, 0.0f32, 0.0f32],
            [0.0f32, 0.0f32, q, -1.0f32],
            [0.0f32, 0.0f32, qn, 0.0f32]
        ]
    }

    /// Returns the model view matrix for a first person view given cam position and rotation
    pub fn build_fp_view_matrix(cam_state: CamState) -> [[f32; 4]; 4] {

        let (sin_yaw, cos_yaw, sin_pitch, cos_pitch) = (
            cam_state.cam_rot.1.sin(),
            cam_state.cam_rot.1.cos(),
            cam_state.cam_rot.0.sin(),
            cam_state.cam_rot.0.cos());
        let xaxis = [cos_yaw, 0.0, -sin_yaw];
        let yaxis = [sin_yaw * sin_pitch, cos_pitch, cos_yaw * sin_pitch];
        let zaxis = [sin_yaw * cos_pitch, -sin_pitch, cos_pitch * cos_yaw];

        let cam_arr = [cam_state.cam_pos.0, cam_state.cam_pos.1, cam_state.cam_pos.2];

        [
            [ xaxis[0], yaxis[0], zaxis[0], 0.0],
            [ xaxis[1], yaxis[1], zaxis[1], 0.0],
            [ xaxis[2], yaxis[2], zaxis[2], 0.0],
            [ dotp(&xaxis, &cam_arr), dotp(&yaxis, &cam_arr), dotp(&zaxis, &cam_arr), 1.0f32]
        ]
    }
}
