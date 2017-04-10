use glium::{ Surface, Program };
use glium::index::{ PrimitiveType, IndexBuffer };
use glium::vertex::VertexBuffer;
use glium::backend::{ Facade, Context };
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::{ Texture2d, DepthTexture2d, DepthFormat, MipmapsOption };

use std::cell::RefCell;
use std::rc::Rc;

use types::Vertex;

use time;

/// struct representing a post effect
pub struct PostEffect {
    context: Rc<Context>,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    target_color: RefCell<Option<Texture2d>>,
    target_depth: RefCell<Option<DepthTexture2d>>,
    /// The current shader being used for post processing
    pub current_shader: &'static str,
    start_time: f32,
    /// The scale factor that the scene will be rendered in relation to to the full window resolution
    pub downscale_factor: f32,
    /// The offset for the chromatic aberration
    pub chrom_offset: f32,
    /// The mix amount for the chromatic aberration
    pub chrom_amt: f32,
}

impl PostEffect {
    /// creates a new instance of a post effect
    pub fn new<F>(facade: &F) -> PostEffect where F: Facade + Clone {
        let vert_arr = [
            Vertex {
                position: [-1.0, -1.0, 0.0],
                normal: [0.0, 0.0, 0.0],
                texture: [0.0, 0.0]
            },
            Vertex {
                position: [-1.0,  1.0, 0.0],
                normal: [0.0, 0.0, 0.0],
                texture: [0.0, 1.0]
            },
            Vertex {
                position: [ 1.0,  1.0, 0.0],
                normal: [0.0, 0.0, 0.0],
                texture: [1.0, 1.0]
            },
            Vertex {
                position: [ 1.0, -1.0, 0.0],
                normal: [0.0, 0.0, 0.0],
                texture: [1.0, 0.0]
            }
        ];

        let ind_arr = [1 as u16, 2, 0, 3];

        PostEffect {
            context: facade.get_context().clone(),
            vertex_buffer: VertexBuffer::new(facade, &vert_arr).unwrap(),
            index_buffer: IndexBuffer::new(facade, PrimitiveType::TriangleStrip, &ind_arr).unwrap(),
            target_color: RefCell::new(None),
            target_depth: RefCell::new(None),
            current_shader: "default",
            start_time: time::precise_time_s() as f32,
            downscale_factor: 1.0f32,
            chrom_offset: 0.003f32,
            chrom_amt: 0.0f32,
        }
    }
}

/// Renders the post effect on to the scene rendered in the draw FnMut
pub fn render_post<T, F, R>(system: &PostEffect, shader: &Program, target: &mut T, mut draw: F)
    -> R where T: Surface, F: FnMut(&mut SimpleFrameBuffer) -> R {

        let target_dimensions = target.get_dimensions();

        let mut target_color = system.target_color.borrow_mut();
        let mut target_depth = system.target_depth.borrow_mut();

        // check whether the colour buffer needs clearing due to window size change
        let clear = if let &Some(ref tex) = &*target_color {
            tex.get_width() != target_dimensions.0 ||
                tex.get_height().unwrap() != target_dimensions.1
        } else {
            false
        };

        if clear || target_color.is_none() {
            let col_tex = Texture2d::empty(
                &system.context,
                (target_dimensions.0 as f32 * system.downscale_factor) as u32,
                (target_dimensions.1 as f32 * system.downscale_factor) as u32).unwrap();
            *target_color = Some(col_tex);

            let dep_tex = DepthTexture2d::empty_with_format(
                &system.context,
                DepthFormat::F32,
                MipmapsOption::NoMipmap,
                (target_dimensions.0 as f32 * system.downscale_factor) as u32,
                (target_dimensions.1 as f32 * system.downscale_factor) as u32).unwrap();
            *target_depth = Some(dep_tex);
        }

        let target_color = target_color.as_ref().unwrap();
        let target_depth = target_depth.as_ref().unwrap();

        // first pass draw the scene into a buffer
        let output = draw(&mut SimpleFrameBuffer::with_depth_buffer(&system.context,
                                                                    target_color,
                                                                    target_depth).unwrap());

        let uniforms = uniform! {
            tex: &*target_color,
            depth_buf: &*target_depth,
            resolution: (target_dimensions.0 as f32, target_dimensions.1 as f32),
            time: time::precise_time_s() as f32 - system.start_time,
            downscale_factor: system.downscale_factor,
            chrom_offset: system.chrom_offset,
            chrom_amt: system.chrom_amt,
        };

        // second pass draw the post effect
        target.draw(&system.vertex_buffer,
                    &system.index_buffer,
                    shader,
                    &uniforms,
                    &Default::default()).unwrap();

        output
    }
