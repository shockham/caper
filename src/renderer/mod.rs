/// Module for utility functions for textures
#[macro_use]
pub mod texture;
/// Module for the lighting system
pub mod lighting;
/// Rendering post processing effects
pub mod posteffect;
/// Module for dealing with shaders
pub mod shader;

use glium::backend::Facade;
use glium::draw_parameters::{BackfaceCullingMode, DepthClamp};
use glium::glutin::{
    event_loop::EventLoop,
    window::{Fullscreen, WindowBuilder},
    Api, ContextBuilder, GlRequest,
};
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::RawImage2d;
use glium::vertex::VertexBuffer;
use glium::DepthTest::IfLess;
use glium::Frame;
use glium::{Blend, Depth, Display, DrawParameters, Surface};

use glium_text;
use glium_text::{FontTexture, TextDisplay, TextSystem};

use fps_counter::FPSCounter;
use std::default::Default;
use time;

use imgui::*;
use imgui_glium_renderer::Renderer as ImGuiRenderer;

use gif;
use gif::SetParameter;
use image;

use rayon::prelude::*;

use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use input::{Input, MouseButton};
use lighting::Lighting;
use posteffect::{render_to_texture, PostEffect};
use shader::Shaders;
use types::{Camera, RenderItem, ShaderIn, TextItem};
#[cfg(feature = "nphysics")]
use types::PhysicsType;
use utils::{
    build_fp_view_matrix, build_persp_proj_mat, frustrum_test, get_frustum_planes, mul_mat4,
};

/// struct for abstracting the render state
pub struct Renderer {
    /// The glium display used for rendering
    pub display: Display,
    /// The glium_text system used for rendering TextItem
    text_system: Arc<Mutex<TextSystem>>,
    /// Fefault font that the text renderer will use
    default_font: Arc<Mutex<FontTexture>>,
    /// Main imgui system
    pub imgui: imgui::Context,
    /// The sub renderer for imgui
    imgui_rend: ImGuiRenderer,
    /// Instance of PostEffect used for rendering post processing
    pub post_effect: PostEffect,
    /// The shaders that can be used for rendering
    pub shaders: Shaders,
    /// The lighting system
    pub lighting: Lighting,
    /// Info on the current gif being written to
    gif_info: Option<GifInfo>,
    /// stuct to track the fps
    fps_counter: FPSCounter,
    /// The render/engine start time
    pub start_time: f64,
    /// The current frames per second the Renderer is drawing at
    pub fps: f32,
    /// The number items rendered in the last drawn frame
    pub render_count: usize,
    /// Whether to display the engine editor window
    pub show_editor: bool,
}

struct GifInfo {
    /// The encoder for the current gif
    encoder: Arc<Mutex<gif::Encoder<File>>>,
    /// The path of the current gif
    path: &'static str,
}

impl Renderer {
    /// Creates new Renderer instance
    pub fn new(title: String, event_loop: &EventLoop<()>) -> Renderer {
        let window_builder = WindowBuilder::new()
            .with_title(title)
            .with_fullscreen(Some(Fullscreen::Borderless(event_loop.primary_monitor())));
        let ctx_builder = ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(true)
            .with_gl(GlRequest::Specific(Api::OpenGl, (4, 0)));
        let display = Display::new(window_builder, ctx_builder, &event_loop).unwrap();

        // create a text system instance and font
        let text_system = TextSystem::new(&display);
        let scale_factor = display.gl_window().window().scale_factor();

        let font = FontTexture::new(
            &display,
            &include_bytes!("./resources/font.ttf")[..],
            (100 as f64 * scale_factor) as u32,
            glium_text::FontTexture::ascii_character_list(),
        )
        .unwrap();

        let mut imgui = imgui::Context::create();
        {
            // Set style for caper editor windows
            let imgui_style = imgui.style_mut();
            // TitleBg
            imgui_style.colors[10] = [0.05f32, 0.05f32, 0.05f32, 0.95f32];
            // TitleBgActive
            imgui_style.colors[11] = [0.01f32, 0.01f32, 0.01f32, 0.95f32];
            // TitleBgCollapsed
            imgui_style.colors[12] = [0.1f32, 0.1f32, 0.1f32, 0.95f32];
            // ScrollbarBg
            imgui_style.colors[14] = [0f32, 0f32, 0f32, 0.8f32];
            // ScrollbarGrab
            imgui_style.colors[15] = [0.98f32, 0.98f32, 0.98f32, 0.8f32];
            // ScrollbarGrabHovered
            imgui_style.colors[16] = [0.95f32, 0.95f32, 0.95f32, 0.8f32];
            // ScrollBarGrabActive
            imgui_style.colors[17] = [0.9f32, 0.9f32, 0.9f32, 0.8f32];
            // Button
            imgui_style.colors[22] = [0.05f32, 0.05f32, 0.05f32, 0.8f32];
            // ButtonHovered
            imgui_style.colors[23] = [0.01f32, 0.01f32, 0.01f32, 0.8f32];
            // ButtonActive
            imgui_style.colors[24] = [0.1f32, 0.1f32, 0.1f32, 0.8f32];
            // Header
            imgui_style.colors[25] = [0.05f32, 0.05f32, 0.05f32, 0.7f32];
            // HeaderHovered
            imgui_style.colors[26] = [0.01f32, 0.01f32, 0.01f32, 0.7f32];
            // HeaderActive
            imgui_style.colors[27] = [0.1f32, 0.1f32, 0.1f32, 0.7f32];
            // CloseButton
            imgui_style.colors[34] = [0.05f32, 0.05f32, 0.05f32, 0.6f32];
            // CloseButtonHovered
            imgui_style.colors[35] = [0.01f32, 0.01f32, 0.01f32, 0.6f32];
            // CloseButtonActive
            imgui_style.colors[36] = [0.1f32, 0.1f32, 0.1f32, 0.6f32];
            //TextSelectedBg
            imgui_style.colors[41] = [0f32, 0f32, 0f32, 0.9f32];
        }

        imgui.fonts().add_font(&[
            imgui::FontSource::TtfData {
                size_pixels: 13. * scale_factor as f32,
                data: include_bytes!("./resources/font.ttf"),
                config: None,
            },
        ]);
 
        let imgui_rend = ImGuiRenderer::init(&mut imgui, &display).unwrap();

        let shaders = Shaders::new(&display);
        let post_effect = PostEffect::new(&display);
        let lighting = Lighting::new(&display);

        let fps_counter = FPSCounter::new();

        let renderer = Renderer {
            display,
            text_system: Arc::new(Mutex::new(text_system)),
            default_font: Arc::new(Mutex::new(font)),
            imgui,
            imgui_rend,
            post_effect,
            start_time: time::precise_time_s(),
            shaders,
            fps_counter,
            fps: 0f32,
            gif_info: None,
            lighting,
            render_count: 0usize,
            show_editor: false,
        };

        {
            let gl_window = renderer.display.gl_window();
            let window = gl_window.window();
            window.set_cursor_visible(true);
        }

        renderer
    }

    /// Update imgui's interal input state
    pub fn update_imgui_input(&mut self, input: &Input) {
        let mut imgui_io = self.imgui.io_mut();

        // set the framebuffer size for imgui
        // moved to here rather than Renderer::new as it was the wrong size
        let (width, height) = self.display.get_framebuffer_dimensions();
        imgui_io.display_size = [width as f32, height as f32];

        imgui_io.mouse_pos = [input.mouse_pos.0, input.mouse_pos.1];
        imgui_io.mouse_down = [
            input.mouse_btns_down.contains(&MouseButton::Left),
            input.mouse_btns_down.contains(&MouseButton::Right),
            input.mouse_btns_down.contains(&MouseButton::Middle),
            false,
            false,
        ];
        for ch in &input.characters_down {
            imgui_io.add_input_character(*ch);
        }
    }

    /// Saves out a screenshot from in-game
    pub fn save_screenshot(&self) {
        // reading the front buffer into an image
        let image: RawImage2d<u8> = self.display.read_front_buffer().unwrap();

        thread::spawn(move || {
            let image =
                image::ImageBuffer::from_raw(image.width, image.height, image.data.into_owned())
                    .unwrap();
            let image = image::DynamicImage::ImageRgba8(image).flipv();
            let path_string = format!("./screenshot_{}.png", time::precise_time_s());
            let path = Path::new(&path_string);
            image.save(path).unwrap();
        });
    }

    /// When called with the same path adds a frame to a gif at the path
    pub fn save_add_to_gif(&mut self, path: &'static str) {
        // reading the front buffer into a gif frame
        let image: RawImage2d<u8> = self.display.read_front_buffer().unwrap();

        let (w, h) = (image.width, image.height);

        // if there is no encoder present create one
        let new_file = {
            match self.gif_info.as_ref() {
                Some(gi_ref) => gi_ref.path != path,
                None => false,
            }
        };
        if self.gif_info.is_none() || new_file {
            let output = OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)
                .unwrap();
            let mut encoder = gif::Encoder::new(output, w as u16, h as u16, &[]).unwrap();
            encoder.set(gif::Repeat::Infinite).unwrap();

            let info = GifInfo {
                encoder: Arc::new(Mutex::new(encoder)),
                path,
            };

            self.gif_info = Some(info);
        }

        if let Some(ref mut info) = self.gif_info {
            let encoder_mutex = info.encoder.clone();
            thread::spawn(move || {
                let mut image = {
                    let image_buf =
                        image::ImageBuffer::from_raw(w, h, image.data.into_owned()).unwrap();
                    let dy_image = image::DynamicImage::ImageRgba8(image_buf).flipv();
                    let fin_image = dy_image.as_rgba8().unwrap();
                    fin_image.clone().into_raw()
                };
                let frame = gif::Frame::from_rgba(w as u16, h as u16, image.as_mut_slice());

                let mut encoder = encoder_mutex.lock().unwrap();
                // Write frame to file
                encoder.write_frame(&frame).unwrap();
            });
        }
    }
}

/// Trait for drawing to screen
pub trait Draw {
    /// Draws a frame
    fn draw<F: FnMut(&Ui), T: Default>(
        &mut self,
        cams: &mut Vec<Camera>,
        render_items: &mut Vec<RenderItem<T>>,
        text_items: &mut Vec<TextItem>,
        f: F,
    );
    /// Draws render_items
    fn draw_render_items<T: Default>(
        &mut self,
        target: Arc<Mutex<Frame>>,
        cams: &mut Vec<Camera>,
        render_items: &mut Vec<RenderItem<T>>,
    );
    /// Draws the text_items
    fn draw_text_items(&mut self, target: Arc<Mutex<Frame>>, text_items: &mut Vec<TextItem>);
    /// Draws the ui
    fn draw_ui<F: FnMut(&Ui), T: Default>(
        &mut self,
        target: Arc<Mutex<Frame>>,
        cams: &mut Vec<Camera>,
        render_items: &mut Vec<RenderItem<T>>,
        text_items: &mut Vec<TextItem>,
        f: F,
    );
}

impl Draw for Renderer {
    /// Draws a frame
    fn draw<F: FnMut(&Ui), T: Default>(
        &mut self,
        cams: &mut Vec<Camera>,
        render_items: &mut Vec<RenderItem<T>>,
        text_items: &mut Vec<TextItem>,
        f: F,
    ) {
        let target = Arc::new(Mutex::new(self.display.draw()));

        self.draw_render_items(Arc::clone(&target), cams, render_items);
        self.draw_text_items(Arc::clone(&target), text_items);
        self.draw_ui(Arc::clone(&target), cams, render_items, text_items, f);

        let mut target = target.lock().unwrap();
        match target.set_finish() {
            Ok(_) => {
                self.fps = self.fps_counter.tick() as f32;
            }
            Err(e) => println!("{:?}", e),
        };
    }

    /// Draw render_items
    fn draw_render_items<T: Default>(
        &mut self,
        target: Arc<Mutex<Frame>>,
        cams: &mut Vec<Camera>,
        render_items: &mut Vec<RenderItem<T>>,
    ) {
        // draw parameters
        let params = DrawParameters {
            depth: Depth {
                test: IfLess,
                write: true,
                clamp: DepthClamp::Clamp,
                ..Default::default()
            },
            blend: Blend::alpha_blending(),
            backface_culling: BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        // drawing a frame
        let context = self.display.get_context().clone();
        let (width, height) = self.display.get_framebuffer_dimensions();
        let mut render_count = 0usize;
        let mut cols = Vec::new();
        let mut depths = Vec::new();
        let mut p_mat = None;
        let mut mv_mat = None;

        cams.iter_mut().for_each(|cam| {
            // uniforms passed to the shaders
            let projection_matrix =
                build_persp_proj_mat(60f32, width as f32 / height as f32, 0.01f32, 1000f32);
            if p_mat.is_none() {
                p_mat = Some(projection_matrix);
            }
            let modelview_matrix = build_fp_view_matrix(&cam);
            if mv_mat.is_none() {
                mv_mat = Some(modelview_matrix);
            }
            let cam_pos = cam.pos;
            let time = (time::precise_time_s() - self.start_time) as f32;

            // calc frustum places for culling
            let combo_matrix = mul_mat4(projection_matrix, modelview_matrix);
            let frustum_planes = get_frustum_planes(&combo_matrix);

            // render to texture/depth
            let mut target = target.lock().unwrap();
            let (target_color, target_depth) =
                render_to_texture(&self.post_effect, &context, &mut *target, |target| {
                    // clear the colour and depth buffers
                    target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);

                    // drawing the render items (with more than one instance)
                    render_items
                        .iter()
                        .filter(|r| r.active && !r.instance_transforms.is_empty())
                        .for_each(|item| {
                            // building the vertex and index buffers
                            let vertex_buffer =
                                VertexBuffer::new(&self.display, &item.vertices).unwrap();

                            // add positions for instances
                            let per_instance = {
                                let data = item
                                    .instance_transforms
                                    .par_iter()
                                    .filter(|t| {
                                        (!t.cull
                                            || frustrum_test(
                                                &t.pos,
                                                t.scale.0.max(t.scale.1.max(t.scale.2)) * 2.5f32,
                                                &frustum_planes,
                                            ))
                                            && t.active
                                    })
                                    .map(|t| ShaderIn {
                                        world_position: t.pos,
                                        world_rotation: t.rot,
                                        world_scale: t.scale,
                                    })
                                    .collect::<Vec<_>>();

                                // if there are no active transforms skip ri
                                if data.is_empty() {
                                    return;
                                }

                                // add instances to render_count
                                render_count += data.len();

                                VertexBuffer::dynamic(&self.display, &data).unwrap()
                            };

                            let tex_name = item
                                .material
                                .texture_name
                                .clone()
                                .unwrap_or_else(|| "default".to_string());
                            let normal_tex_name = item
                                .material
                                .normal_texture_name
                                .clone()
                                .unwrap_or_else(|| "default_normal".to_string());

                            let dir_lights = self.lighting.directional_tex.borrow();

                            let uniforms = uniform! {
                                projection_matrix: projection_matrix,
                                modelview_matrix: modelview_matrix,
                                cam_pos: cam_pos,
                                viewport: (width as f32, height as f32),
                                time: time,
                                tex: &self.shaders.textures[tex_name.as_str()],
                                normal_tex: &self.shaders.textures[normal_tex_name.as_str()],
                                dir_lights: &*dir_lights,
                            };

                            target
                                .draw(
                                    (&vertex_buffer, per_instance.per_instance().unwrap()),
                                    &NoIndices(PrimitiveType::Patches {
                                        vertices_per_patch: 3,
                                    }),
                                    &self.shaders.shaders[item.material.shader_name.as_str()],
                                    &uniforms,
                                    &params,
                                )
                                .unwrap();
                        });
                });

            cols.push(target_color);
            depths.push(target_depth);
        });

        //let texs_arr = Texture2dArray::new(&self.post_effect.context, cols).unwrap();
        //let depths_arr = DepthTexture2dArray::new(&self.post_effect.context, depths).unwrap();

        // second pass draw the post effect and composition
        let uniforms = uniform! {
            // general uniforms
            tex: &cols[0],
            depth_buf: &depths[0],
            resolution: (width as f32, height as f32),
            time: time::precise_time_s() as f32 - self.post_effect.start_time,
            cam_pos: cams[0].pos,
            projection_matrix: p_mat.unwrap(),
            modelview_matrix: mv_mat.unwrap(),
            downscale_factor: self.post_effect.downscale_factor,
            // post effect param uniforms
            chrom_offset: self.post_effect.post_shader_options.chrom_offset,
            chrom_amt: self.post_effect.post_shader_options.chrom_amt,
            blur: self.post_effect.post_shader_options.blur,
            blur_amt: self.post_effect.post_shader_options.blur_amt,
            blur_radius: self.post_effect.post_shader_options.blur_radius,
            blur_weight: self.post_effect.post_shader_options.blur_weight,
            bokeh: self.post_effect.post_shader_options.bokeh,
            bokeh_focal_depth: self.post_effect.post_shader_options.bokeh_focal_depth,
            bokeh_focal_width: self.post_effect.post_shader_options.bokeh_focal_width,
            color_offset: self.post_effect.post_shader_options.color_offset,
            greyscale: self.post_effect.post_shader_options.greyscale,
            noise: self.post_effect.post_shader_options.noise,
            scanline: self.post_effect.post_shader_options.scanline,
            scanline_count: self.post_effect.post_shader_options.scanline_count,
        };

        let uniforms = if cols.len() > 1 {
            uniforms
                .add("tex_1", &cols[1])
                .add("depth_buf_1", &depths[1])
        } else {
            uniforms
                .add("tex_1", &cols[0])
                .add("depth_buf_1", &depths[0])
        };

        let uniforms = if cols.len() > 2 {
            uniforms
                .add("tex_2", &cols[2])
                .add("depth_buf_2", &depths[2])
        } else {
            uniforms
                .add("tex_2", &cols[0])
                .add("depth_buf_2", &depths[0])
        };

        let uniforms = if cols.len() > 3 {
            uniforms
                .add("tex_3", &cols[3])
                .add("depth_buf_3", &depths[3])
        } else {
            uniforms
                .add("tex_3", &cols[0])
                .add("depth_buf_3", &depths[0])
        };

        let uniforms = if cols.len() > 4 {
            uniforms
                .add("tex_4", &cols[4])
                .add("depth_buf_4", &depths[4])
        } else {
            uniforms
                .add("tex_4", &cols[0])
                .add("depth_buf_4", &depths[0])
        };

        let uniforms = if cols.len() > 5 {
            uniforms
                .add("tex_5", &cols[5])
                .add("depth_buf_5", &depths[5])
        } else {
            uniforms
                .add("tex_5", &cols[0])
                .add("depth_buf_5", &depths[0])
        };

        {
            let mut target = target.lock().unwrap();
            target
                .draw(
                    &self.post_effect.vertex_buffer,
                    &self.post_effect.index_buffer,
                    &self.shaders.post_shaders[self.post_effect.current_shader],
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
        }

        self.render_count = render_count;
    }

    fn draw_text_items(&mut self, target: Arc<Mutex<Frame>>, text_items: &mut Vec<TextItem>) {
        let (width, height) = self.display.get_framebuffer_dimensions();
        let renderer = Arc::new(Mutex::new(self));

        // drawing the text items
        text_items
            .iter()
            .filter(|r| r.active)
            .for_each(|text_item| {
                // create the matrix for the text
                let matrix = [
                    [0.02 * text_item.scale.0, 0.0, 0.0, 0.0],
                    [
                        0.0,
                        0.02 * text_item.scale.1 * (width as f32) / (height as f32),
                        0.0,
                        0.0,
                    ],
                    [0.0, 0.0, 0.02 * text_item.scale.2, 0.0],
                    [text_item.pos.0, text_item.pos.1, text_item.pos.2, 1.0f32],
                ];

                // lock required members
                let renderer = renderer.lock().unwrap();
                let text_system = renderer.text_system.lock().unwrap();
                let default_font = renderer.default_font.lock().unwrap();

                // create TextDisplay for item
                let text = TextDisplay::new(&*text_system, &*default_font, text_item.text.as_str());

                // draw the text
                let mut target = target.lock().unwrap();
                let _ =
                    glium_text::draw(&text, &*text_system, &mut *target, matrix, text_item.color);
            });
    }

    fn draw_ui<F: FnMut(&Ui), T: Default>(
        &mut self,
        target: Arc<Mutex<Frame>>,
        cams: &mut Vec<Camera>,
        render_items: &mut Vec<RenderItem<T>>,
        text_items: &mut Vec<TextItem>,
        mut f: F,
    ) {
        let renderer = self;

        // imgui elements
        let ui = renderer.imgui.frame();
        f(&ui);

        // create the engine editor
        if renderer.show_editor {
            let fps = renderer.fps;
            // create the editor window
            Window::new(im_str!("caper editor"))
                .size([300f32, 200f32], Condition::FirstUseEver)
                .position([0f32, 0f32], Condition::FirstUseEver)
                .collapsible(true)
                .build(&ui, || {
                    // fps
                    ui.text(im_str!("fps: {:?}", fps));
                    // camera state editor
                    if ui.collapsing_header(im_str!("Camera")).build() {
                        for cam in cams {
                            ui.tree_node(im_str!("cam")).build(|| {
                                // camera position
                                if ui.collapsing_header(im_str!("position")).build() {
                                    ui.input_float(im_str!("x"), &mut cam.pos.0)
                                        .step(0.1)
                                        .step_fast(1.0)
                                        .build();
                                    ui.input_float(im_str!("y"), &mut cam.pos.1)
                                        .step(0.1)
                                        .step_fast(1.0)
                                        .build();
                                    ui.input_float(im_str!("z"), &mut cam.pos.2)
                                        .step(0.1)
                                        .step_fast(1.0)
                                        .build();
                                }
                                // camera rotation
                                if ui.collapsing_header(im_str!("rotation")).build() {
                                    ui.input_float(im_str!("x"), &mut cam.euler_rot.0)
                                        .step(0.1)
                                        .step_fast(1.0)
                                        .build();
                                    ui.input_float(im_str!("y"), &mut cam.euler_rot.1)
                                        .step(0.1)
                                        .step_fast(1.0)
                                        .build();
                                    ui.input_float(im_str!("z"), &mut cam.euler_rot.2)
                                        .step(0.1)
                                        .step_fast(1.0)
                                        .build();
                                }
                            });
                        }
                    }
                    // render items editor
                    if ui.collapsing_header(im_str!("Render items")).build() {
                        // create node for each item
                        for render_item in render_items {
                            ui.tree_node(&im_str!("name:{}", render_item.name))
                                .build(|| {
                                    ui.checkbox(im_str!("active"), &mut render_item.active);
                                    // physics type TODO make sure this is propagated
                                    #[cfg(feature = "nphysics")]
                                    {
                                        let mut physics_type = match render_item.physics_type {
                                            PhysicsType::Static => 0,
                                            PhysicsType::Dynamic => 1,
                                            PhysicsType::None => 2,
                                        };
                                        ComboBox::new(im_str!("physics")).build_simple_string(
                                            &ui,
                                            &mut physics_type,
                                            &[im_str!("Static"), im_str!("Dynamic"), im_str!("None")],
                                        );
                                        render_item.physics_type = match physics_type {
                                            0 => PhysicsType::Static,
                                            1 => PhysicsType::Dynamic,
                                            _ => PhysicsType::None,
                                        };
                                    }
                                    // TODO add mutability for these items
                                    ui.text(im_str!(
                                        "instance_count:{}",
                                        render_item.instance_transforms.len()
                                    ));
                                    ui.text(im_str!("vert_count:{}", render_item.vertices.len()));
                                });
                        }
                    }
                    // text items editor
                    if ui.collapsing_header(im_str!("Text items")).build() {
                        for text_item in text_items {
                            ui.tree_node(&im_str!("name:{}", text_item.name)).build(|| {
                                // TODO add mutability
                                //ui.input_text(im_str!("text"), &mut text_item.text).build();
                                // text item color
                                if ui.collapsing_header(im_str!("color")).build() {
                                    ui.input_float(im_str!("r"), &mut text_item.color.0)
                                        .step(0.01)
                                        .step_fast(1.0)
                                        .build();
                                    ui.input_float(im_str!("g"), &mut text_item.color.1)
                                        .step(0.01)
                                        .step_fast(1.0)
                                        .build();
                                    ui.input_float(im_str!("b"), &mut text_item.color.2)
                                        .step(0.01)
                                        .step_fast(1.0)
                                        .build();
                                    ui.input_float(im_str!("a"), &mut text_item.color.3)
                                        .step(0.01)
                                        .step_fast(1.0)
                                        .build();
                                }
                                // text item position
                                if ui.collapsing_header(im_str!("position")).build() {
                                    ui.input_float(im_str!("x"), &mut text_item.pos.0)
                                        .step(0.01)
                                        .step_fast(1.0)
                                        .build();
                                    ui.input_float(im_str!("y"), &mut text_item.pos.1)
                                        .step(0.01)
                                        .step_fast(1.0)
                                        .build();
                                    ui.input_float(im_str!("z"), &mut text_item.pos.2)
                                        .step(0.01)
                                        .step_fast(1.0)
                                        .build();
                                }
                                // text item scale
                                if ui.collapsing_header(im_str!("scale")).build() {
                                    ui.input_float(im_str!("x"), &mut text_item.scale.0)
                                        .step(0.01)
                                        .step_fast(1.0)
                                        .build();
                                    ui.input_float(im_str!("y"), &mut text_item.scale.1)
                                        .step(0.01)
                                        .step_fast(1.0)
                                        .build();
                                    ui.input_float(im_str!("z"), &mut text_item.scale.2)
                                        .step(0.01)
                                        .step_fast(1.0)
                                        .build();
                                }
                                ui.checkbox(im_str!("active"), &mut text_item.active);
                            });
                        }
                    }
                });
        }

        // render imgui items
        let mut target = target.lock().unwrap();
        let draw_data = ui.render();
        renderer.imgui_rend.render(&mut *target, draw_data).unwrap();
    }
}
