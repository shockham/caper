use glium::{ Display, DrawParameters, DisplayBuild, Surface, Depth, Blend };
use glium::index::{ NoIndices, PrimitiveType };
use glium::DepthTest::IfLess;
use glium::vertex::VertexBuffer;
use glium::glutin::{ WindowBuilder, get_primary_monitor };
use glium::glutin::CursorState::Hide;//{ Grab, Hide };
use glium::draw_parameters::BackfaceCullingMode::CullClockwise;
use glium::texture::RawImage2d;

use glium_text;
use glium_text::{ TextSystem, FontTexture, TextDisplay };

use time;
use std::default::Default;
use fps_counter::FPSCounter;

use imgui::{ ImGui, Ui };
use imgui::glium_renderer::Renderer as ImGuiRenderer;

use image;
use gif;
use gif::SetParameter;
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;


use shader::Shaders;
use utils::*;
use posteffect::*;
use types::*;

pub const FIXED_TIME_STAMP: u64 = 16666667;

/// struct for abstracting the render state
pub struct Renderer {
    pub display: Display,
    pub text_system: TextSystem,
    default_font: FontTexture,
    imgui: ImGui,
    imgui_rend: ImGuiRenderer,
    pub post_effect: PostEffect,
    pub start_time: f64,
    pub shaders: Shaders,
    fps_counter: FPSCounter,
    pub fps: f32,
}

impl Renderer {
    /// Creates new Renderer instance
    pub fn new(title:String) -> Renderer {
        // create a diplay instance
        let display = WindowBuilder::new()
            .with_depth_buffer(24)
            .with_title(title)
            .with_vsync()
            .with_fullscreen(get_primary_monitor())
            .build_glium()
            .unwrap();

        // create a text system instance and font
        let text_system = TextSystem::new(&display);
        let font = FontTexture::new(&display, &include_bytes!("./resources/font.otf")[..], 100).unwrap();

        let mut imgui = ImGui::init();
        let imgui_rend = ImGuiRenderer::init(&mut imgui, &display).unwrap();

        let shaders = Shaders::new(&display);
        let post_fx = PostEffect::new(&display);

        let fps_counter = FPSCounter::new(); 

        let renderer = Renderer {
            display: display,
            text_system: text_system,
            default_font: font,
            imgui: imgui,
            imgui_rend: imgui_rend,
            post_effect: post_fx,
            start_time: time::precise_time_s(),
            shaders: shaders,
            fps_counter: fps_counter,
            fps: 0f32,
        };

        renderer.setup();

        renderer
    }

    /// Sets up the render window
    pub fn setup(&self) {
        // get the window for various values
        let window = self.display.get_window().unwrap();
        window.set_cursor_state(Hide).ok();
    }

    pub fn update_imgui_input(&mut self, pos: (i32, i32), btns: (bool, bool, bool)) {
        self.imgui.set_mouse_pos(pos.0 as f32, pos.1 as f32);
        self.imgui.set_mouse_down(&[btns.0, btns.1, btns.2, false, false]);
        //self.imgui.set_mouse_wheel(self.mouse_wheel);
    }

    /// Draws a frame
    pub fn draw<F: FnMut(&Ui)>(&mut self,
                               cam_state: &CamState,
                               render_items: &Vec<RenderItem>,
                               text_items: &Vec<TextItem>,
                               mut f: F) {
        // get display dimensions
        let (width, height) = self.display.get_framebuffer_dimensions();

        // draw parameters
        let params = DrawParameters {
            depth: Depth {
                test: IfLess,
                write: true,
                .. Default::default()
            },
            blend: Blend::alpha_blending(),
            backface_culling: CullClockwise,
            .. Default::default()
        };

        // uniforms passed to the shaders
        let uniforms = uniform! {
            projection_matrix: build_persp_proj_mat(60f32, width as f32/height as f32, 0.01f32, 1000f32),
            modelview_matrix: build_fp_view_matrix(cam_state),
            cam_pos: cam_state.cam_pos,
            time: (time::precise_time_s() - self.start_time) as f32,
        };

        // drawing a frame
        let mut target = self.display.draw();

        render_post(&self.post_effect,
                    &self.shaders.post_shaders.get(self.post_effect.current_shader).unwrap(),
                    &mut target,
                    |target| {
                        // clear the colour and depth buffers
                        target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);

                        // drawing the render items
                        for item in render_items.iter().filter(|r| r.active) {
                            // building the vertex and index buffers
                            let vertex_buffer = VertexBuffer::new(&self.display, &item.vertices).unwrap();

                            // add positions for instances
                            let per_instance = {
                                let data = item.instance_transforms.iter().filter(|t| t.active).map(|t| {
                                    Attr {
                                        world_position: t.pos,
                                        world_rotation: t.rot,
                                        world_scale: t.scale
                                    }
                                }).collect::<Vec<_>>();

                                VertexBuffer::dynamic(&self.display, &data).unwrap()
                            };

                            target.draw(
                                (&vertex_buffer, per_instance.per_instance().unwrap()),
                                &NoIndices(PrimitiveType::Patches { vertices_per_patch: 3 }),
                                &self.shaders.shaders.get(item.shader_name.as_str()).unwrap(),
                                &uniforms,
                                &params).unwrap();
                        }
                    });

        // drawing the text items
        for text_item in text_items.iter().filter(|r| r.active) {
            // create the matrix for the text
            let matrix = [[0.02 * text_item.scale.0, 0.0, 0.0, 0.0], 
            [0.0, 0.02 * text_item.scale.1 * (width as f32) / (height as f32), 0.0, 0.0],
            [0.0, 0.0, 0.02 * text_item.scale.2, 0.0],
            [text_item.pos.0, text_item.pos.1, text_item.pos.2, 1.0f32]];

            // create TextDisplay for item, TODO change this to not be done every frame
            let text = TextDisplay::new(&self.text_system, &self.default_font,
                                        text_item.text.as_str());

            // draw the text
            glium_text::draw(&text,
                             &self.text_system,
                             &mut target,
                             matrix,
                             text_item.color);
        }

        // imgui elements
        let ui = self.imgui.frame((width, height), (width, height), 0.1);
        f(&ui);
        self.imgui_rend.render(&mut target, ui).unwrap();

        match target.finish() {
            Ok(_) => { self.fps = self.fps_counter.tick() as f32; },
            Err(e) => println!("{:?}", e),
        };
    }

    /// Saves out a screenshot from in-game
    pub fn save_screenshot(&self) {
        // reading the front buffer into an image
        let image: RawImage2d<u8> = self.display.read_front_buffer();
        let image = image::ImageBuffer::from_raw(image.width, image.height, image.data.into_owned()).unwrap();
        let image = image::DynamicImage::ImageRgba8(image).flipv();
        let mut output = File::create(&Path::new(format!("./screenshot_{}.png", 
                                                                  time::precise_time_s()).as_str())).unwrap();
        image.save(&mut output, image::ImageFormat::PNG).unwrap();
    }

    /// When called with the same path adds a frame to a gif at the path
    pub fn save_add_to_gif(&self, path:&'static str) {
        // reading the front buffer into an image
        let mut image: RawImage2d<u8> = self.display.read_front_buffer();
        let (w, h) = (image.width, image.height);
        
        let mut output = OpenOptions::new().write(true).create(true).open(path).unwrap(); 
        let mut encoder = gif::Encoder::new(&mut output, w as u16, h as u16, &[]).unwrap();
        encoder.set(gif::Repeat::Infinite).unwrap();

        let frame = gif::Frame::from_rgba(w as u16, h as u16, image.data.to_mut());
        // Write frame to file
        encoder.write_frame(&frame).unwrap();
    }
}
