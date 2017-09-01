use glium::{Surface, Program};
use glium::index::{PrimitiveType, IndexBuffer};
use glium::vertex::VertexBuffer;
use glium::backend::{Facade, Context};
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::{Texture2d, DepthTexture2d, DepthFormat, MipmapsOption};

use std::cell::RefCell;
use std::rc::Rc;

use types::Vertex;

use time;

/// struct representing a post effect
pub struct PostEffect {
    /// Ref to the rendering context
    context: Rc<Context>,
    /// The vertex buffer to render
    vertex_buffer: VertexBuffer<Vertex>,
    /// The index buffer to render
    index_buffer: IndexBuffer<u16>,
    /// Wrapped color texture
    target_color: RefCell<Option<Texture2d>>,
    /// Wrapped depth texture
    target_depth: RefCell<Option<DepthTexture2d>>,
    /// The current shader being used for post processing
    pub current_shader: &'static str,
    /// The time the post effect was initialised
    start_time: f32,
    /// The scale factor that the scene will be rendered
    /// in relation to to the full window resolution
    pub downscale_factor: f32,
    /// Options for the default post shader
    pub post_shader_options: PostShaderOptions,
}

impl PostEffect {
    /// creates a new instance of a post effect
    pub fn new<F>(facade: &F) -> PostEffect
    where
        F: Facade + Clone,
    {
        let vert_arr = [
            Vertex {
                position: [-1.0, -1.0, 0.0],
                normal: [0.0, 0.0, 0.0],
                texture: [0.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0, 0.0],
                normal: [0.0, 0.0, 0.0],
                texture: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, 0.0],
                normal: [0.0, 0.0, 0.0],
                texture: [1.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                normal: [0.0, 0.0, 0.0],
                texture: [1.0, 0.0],
            },
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
            post_shader_options: PostShaderOptionsBuilder::default().build().unwrap(),
        }
    }
}

/// Contains all the options for the default post shader
#[derive(Builder, Clone, PartialEq)]
pub struct PostShaderOptions {
    /// The offset for the chromatic aberration
    #[builder(default = "0.003f32")]
    pub chrom_offset: f32,
    /// The mix amount for the chromatic aberration, if this is at 0.0 chromatic aberration is off
    #[builder(default = "0.0f32")]
    pub chrom_amt: f32,
    /// Whether blur is on
    #[builder(default = "false")]
    pub blur: bool,
    /// The amount of blur
    #[builder(default = "1.0f32")]
    pub blur_amt: f32,
    /// The radius of the blur
    #[builder(default = "1.0f32")]
    pub blur_radius: f32,
    /// The weight of the blur
    #[builder(default = "1.0f32")]
    pub blur_weight: f32,
    /// Whether bokeh is on
    #[builder(default = "false")]
    pub bokeh: bool,
    /// Bokeh focal depth
    #[builder(default = "0.2f32")]
    pub bokeh_focal_depth: f32,
    /// Bokeh focal width
    #[builder(default = "0.2f32")]
    pub bokeh_focal_width: f32,
    /// Colour grading
    #[builder(default = "(1f32, 1f32, 1f32, 1f32)")]
    pub color_offset: (f32, f32, f32, f32),
    /// Greyscale
    #[builder(default = "false")]
    pub greyscale: bool,
}

/// Renders the post effect on to the scene rendered in the draw FnMut
pub fn render_post<T, F, R>(system: &PostEffect, shader: &Program, target: &mut T, mut draw: F) -> R
where
    T: Surface,
    F: FnMut(&mut SimpleFrameBuffer) -> R,
{

    let target_dimensions = target.get_dimensions();

    let mut target_color = system.target_color.borrow_mut();
    let mut target_depth = system.target_depth.borrow_mut();

    // check whether the colour buffer needs clearing due to window size change
    let clear = if let &Some(ref tex) = &*target_color {
        tex.get_width() != target_dimensions.0 || tex.get_height().unwrap() != target_dimensions.1
    } else {
        false
    };

    if clear || target_color.is_none() {
        let col_tex = Texture2d::empty(
            &system.context,
            (target_dimensions.0 as f32 * system.downscale_factor) as u32,
            (target_dimensions.1 as f32 * system.downscale_factor) as u32,
        ).unwrap();
        *target_color = Some(col_tex);

        let dep_tex = DepthTexture2d::empty_with_format(
            &system.context,
            DepthFormat::F32,
            MipmapsOption::NoMipmap,
            (target_dimensions.0 as f32 * system.downscale_factor) as u32,
            (target_dimensions.1 as f32 * system.downscale_factor) as u32,
        ).unwrap();
        *target_depth = Some(dep_tex);
    }

    let target_color = target_color.as_ref().unwrap();
    let target_depth = target_depth.as_ref().unwrap();

    // first pass draw the scene into a buffer
    let output = draw(&mut SimpleFrameBuffer::with_depth_buffer(
        &system.context,
        target_color,
        target_depth,
    ).unwrap());

    let uniforms =
        uniform! {
            // general uniforms
            tex: &*target_color,
            depth_buf: &*target_depth,
            resolution: (target_dimensions.0 as f32, target_dimensions.1 as f32),
            time: time::precise_time_s() as f32 - system.start_time,
            downscale_factor: system.downscale_factor,
            // post effect param uniforms
            chrom_offset: system.post_shader_options.chrom_offset,
            chrom_amt: system.post_shader_options.chrom_amt,
            blur: system.post_shader_options.blur,
            blur_amt: system.post_shader_options.blur_amt,
            blur_radius: system.post_shader_options.blur_radius,
            blur_weight: system.post_shader_options.blur_weight,
            bokeh: system.post_shader_options.bokeh,
            bokeh_focal_depth: system.post_shader_options.bokeh_focal_depth,
            bokeh_focal_width: system.post_shader_options.bokeh_focal_width,
            color_offset: system.post_shader_options.color_offset,
            greyscale: system.post_shader_options.greyscale,
        };

    // second pass draw the post effect
    target
        .draw(
            &system.vertex_buffer,
            &system.index_buffer,
            shader,
            &uniforms,
            &Default::default(),
        )
        .unwrap();

    output
}
