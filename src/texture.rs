/// Macro for including and loading a texture
#[macro_export]
macro_rules! load_texture (
    ($file:expr, $display:ident) => {{
        use glium::texture::compressed_srgb_texture2d::CompressedSrgbTexture2d;
        use std::io::Cursor;
        use glium;
        use image;

        let image = image::load(Cursor::new(&include_bytes!($file)[..]), 
                                image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);

        CompressedSrgbTexture2d::new($display, image).unwrap()
    }};
);
