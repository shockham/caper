/// Macro for including and loading a texture
#[macro_export]
macro_rules! load_texture (
    ($file:expr, $display:expr) => {{
        use $crate::glium::texture::compressed_srgb_texture2d::CompressedSrgbTexture2d;
        use $crate::glium::texture::RawImage2d;
        use std::io::Cursor;
        use $crate::image;

        let image = image::load(Cursor::new(&include_bytes!($file)[..]),
                                image::ImageFormat::Png).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(image.into_raw().as_slice(),
                                                       image_dimensions);

        CompressedSrgbTexture2d::new($display, image).unwrap()
    }};
);

/// Macro for dynamically loading a texture
#[macro_export]
macro_rules! load_texture_dynamic (
    ($file:expr, $display:expr) => {{
        use $crate::glium::texture::compressed_srgb_texture2d::CompressedSrgbTexture2d;
        use $crate::glium::texture::RawImage2d;
        use std::io::Cursor;
        use std::fs::File;
        use std::io::BufReader;
        use std::io::prelude::*;
        use $crate::image;


        let file = File::open($file).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = Vec::new();
        buf_reader.read_to_end(&mut contents).unwrap();

        let image = image::load(Cursor::new(&contents[..]),
                                image::ImageFormat::Png).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(image.into_raw().as_slice(),
                                                       image_dimensions);

        CompressedSrgbTexture2d::new($display, image).unwrap()
    }};
);
