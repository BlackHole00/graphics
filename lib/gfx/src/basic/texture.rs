extern crate gl;
use gl::types::*;

use super::{
    shader::{get_uniform_location, ShaderUniformable},
    Bindable,
};
use crate::{derives::TextureObject, gl_call};

use image::{DynamicImage, GenericImageView};
use std::{ffi::c_void, path::Path};

pub trait TextureObject: Bindable {}

#[derive(Clone, TextureObject)]
pub struct Texture {
    texture_id: GLuint,
    gl_type: GLenum,
    active_texture_number: GLenum,
}

impl Texture {
    pub fn set_image(&self, image_data: ImageData, internal_format: GLenum, format: GLenum) {
        self.bind();

        match image_data {
            ImageData::Raw {
                data,
                width,
                height,
            } => {
                gl_call!(gl::TexImage2D(
                    self.gl_type,
                    0,
                    internal_format as i32,
                    width as i32,
                    height as i32,
                    0,
                    format,
                    gl::UNSIGNED_BYTE,
                    data.as_ptr() as *const c_void
                ));
            }
            ImageData::Image {
                image,
                flip_h,
                flip_v,
            } => {
                let mut image = image;

                if flip_h {
                    image = image.fliph();
                }
                if flip_v {
                    image = image.flipv();
                }

                gl_call!(gl::TexImage2D(
                    self.gl_type,
                    0,
                    internal_format as i32,
                    image.width() as i32,
                    image.height() as i32,
                    0,
                    format,
                    gl::UNSIGNED_BYTE,
                    &image.as_bytes()[0] as *const u8 as *const c_void
                ));
            }
            _ => {
                log::error!("Invalid image data");
            }
        }
    }
}

impl Bindable for Texture {
    fn bind(&self) {
        gl_call!(gl::ActiveTexture(self.active_texture_number));
        gl_call!(gl::BindTexture(self.gl_type, self.texture_id));
    }

    fn unbind(&self) {
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, 0));
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        gl_call!(gl::DeleteTextures(1, &self.texture_id));
    }
}

impl ShaderUniformable for &Texture {
    fn set_uniform(&self, shader_id: GLuint, uniform_name: &str) {
        let uniform_location = get_uniform_location(shader_id, uniform_name);
        gl_call!(gl::Uniform1i(
            uniform_location,
            (self.active_texture_number - gl::TEXTURE0) as i32
        ));
    }
}

pub enum ImageData {
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

pub struct TextureBuilder {
    gl_type: GLenum,
    image_data: ImageData,

    format: GLenum,
    internal_format: GLenum,
    active_texture_number: GLuint,
    texture_wrap_s: GLenum,
    texture_wrap_t: GLenum,
    texture_min_filter: GLenum,
    texture_mag_filter: GLenum,

    use_mipmaps: bool,
}

impl TextureBuilder {
    pub fn new() -> TextureBuilder {
        TextureBuilder::default()
    }

    pub fn from_file(image_path: &Path, flip_h: bool, flip_v: bool) -> Option<TextureBuilder> {
        let image = image::open(image_path);
        match image {
            Err(_) => None,
            Ok(image) => {
                let format = match image.color() {
                    image::ColorType::Rgb8 => gl::RGB,
                    image::ColorType::Rgba8 => gl::RGBA,
                    _ => gl::RGB,
                };

                Some(
                    TextureBuilder::default()
                        .image_data(ImageData::Image {
                            image,
                            flip_h,
                            flip_v,
                        })
                        .format(format)
                        .internal_format(format),
                )
            }
        }
    }

    pub fn from_raw_data(data: &[u8], width: u32, height: u32) -> TextureBuilder {
        TextureBuilder::default().image_data(ImageData::Raw {
            data: data.to_owned(),
            width,
            height,
        })
    }

    pub fn gl_type(mut self, gl_type: GLenum) -> TextureBuilder {
        self.gl_type = gl_type;
        self
    }

    pub fn format(mut self, format: GLenum) -> TextureBuilder {
        self.format = format;
        self
    }

    pub fn internal_format(mut self, internal_format: GLenum) -> TextureBuilder {
        self.internal_format = internal_format;
        self
    }

    pub fn active_texture_number(mut self, active_texture_number: GLenum) -> TextureBuilder {
        self.active_texture_number = active_texture_number;
        self
    }

    pub fn texture_wrap_s(mut self, texture_wrap_s: GLenum) -> TextureBuilder {
        self.texture_wrap_s = texture_wrap_s;
        self
    }

    pub fn texture_wrap_t(mut self, texture_wrap_t: GLenum) -> TextureBuilder {
        self.texture_wrap_t = texture_wrap_t;
        self
    }

    pub fn texture_min_filter(mut self, texture_min_filter: GLenum) -> TextureBuilder {
        self.texture_min_filter = texture_min_filter;
        self
    }

    pub fn texture_mag_filter(mut self, texture_mag_filter: GLenum) -> TextureBuilder {
        self.texture_mag_filter = texture_mag_filter;
        self
    }

    pub fn use_mipmaps(mut self, use_mipmaps: bool) -> TextureBuilder {
        self.use_mipmaps = use_mipmaps;
        self
    }

    pub fn image_data(mut self, image_data: ImageData) -> TextureBuilder {
        self.image_data = image_data;
        self
    }

    pub fn build(self) -> Texture {
        let mut texture_id = 1;
        gl_call!(gl::GenTextures(1, &mut texture_id));

        gl_call!(gl::ActiveTexture(self.active_texture_number));
        gl_call!(gl::BindTexture(self.gl_type, texture_id));

        gl_call!(gl::TexParameteri(
            self.gl_type,
            gl::TEXTURE_WRAP_S,
            self.texture_wrap_s as i32
        ));
        gl_call!(gl::TexParameteri(
            self.gl_type,
            gl::TEXTURE_WRAP_T,
            self.texture_wrap_t as i32
        ));
        gl_call!(gl::TexParameteri(
            self.gl_type,
            gl::TEXTURE_MIN_FILTER,
            self.texture_min_filter as i32
        ));
        gl_call!(gl::TexParameteri(
            self.gl_type,
            gl::TEXTURE_MAG_FILTER,
            self.texture_mag_filter as i32
        ));

        match self.image_data {
            ImageData::Raw {
                data,
                width,
                height,
            } => {
                gl_call!(gl::TexImage2D(
                    self.gl_type,
                    0,
                    self.internal_format as i32,
                    width as i32,
                    height as i32,
                    0,
                    self.format,
                    gl::UNSIGNED_BYTE,
                    data.as_ptr() as *const c_void
                ));
            }
            ImageData::Image {
                image,
                flip_h,
                flip_v,
            } => {
                let mut image = image;

                if flip_h {
                    image = image.fliph();
                }
                if flip_v {
                    image = image.flipv();
                }

                gl_call!(gl::TexImage2D(
                    self.gl_type,
                    0,
                    self.internal_format as i32,
                    image.width() as i32,
                    image.height() as i32,
                    0,
                    self.format,
                    gl::UNSIGNED_BYTE,
                    &image.as_bytes()[0] as *const u8 as *const c_void
                ));
            }
            ImageData::None => {}
        }

        if self.use_mipmaps {
            gl_call!(gl::GenerateMipmap(self.gl_type))
        };

        Texture {
            texture_id,
            gl_type: self.gl_type,
            active_texture_number: self.active_texture_number,
        }
    }
}

impl Default for TextureBuilder {
    fn default() -> Self {
        TextureBuilder {
            gl_type: gl::TEXTURE_2D,
            image_data: ImageData::None,

            format: gl::RGB,
            internal_format: gl::RGB,
            active_texture_number: gl::TEXTURE0,
            texture_wrap_s: gl::CLAMP_TO_BORDER,
            texture_wrap_t: gl::CLAMP_TO_BORDER,
            texture_min_filter: gl::NEAREST_MIPMAP_LINEAR,
            texture_mag_filter: gl::NEAREST,

            use_mipmaps: true,
        }
    }
}
