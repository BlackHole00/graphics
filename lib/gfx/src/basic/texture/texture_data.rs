use image::DynamicImage;

pub enum TextureData {
    Raw {
        data: Vec<u8>,
        width: u32,
        height: u32,
    },
    Image {
        image: DynamicImage,
        flip_v: bool,
        flip_h: bool,
    },
    None,
}