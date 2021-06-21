extern crate gl;
use gl::types::*;

use super::TextureData;
use crate::{
    prelude::{TextureObject, ShaderObject, Bindable, gl_call},
    basic::ShaderUniform,
};
use image::GenericImageView;
use std::ffi::c_void;

#[derive(Clone, TextureObject)]
pub struct Texture {
    texture_id: GLuint,
    gl_type: GLenum,
    pub active_texture_number: GLenum,
}

impl Texture {
    pub fn new_empty(gl_type: GLenum) -> Texture {
        let mut texture_id = 1;
        gl_call!(gl::GenTextures(1, &mut texture_id));

        Texture {
            texture_id,
            gl_type,
            active_texture_number: 0,
        }
    } 

    pub fn from_raw(texture_id: GLuint, gl_type: GLenum, active_texture_number: GLenum) -> Texture {
        Texture {
            texture_id,
            gl_type,
            active_texture_number,
        }
    }

    pub fn set_image(&self, texture_data: TextureData, internal_format: GLenum, format: GLenum) {
        self.bind();

        match texture_data {
            TextureData::Raw {
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
            },
            TextureData::Image {
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
            },
            _ => {
                log::error!("Invalid image data");
            },
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

impl ShaderUniform for &Texture {
    fn set_uniform(&self, shader: &mut impl ShaderObject, uniform_name: &str) {
        let uniform_location = shader.get_uniform_location(uniform_name);
        gl_call!(gl::Uniform1i(
            uniform_location,
            (self.active_texture_number - gl::TEXTURE0) as i32
        ));
    }
}