extern crate gl;
use self::gl::types::*;

use log::warn;
use std::mem;

pub fn size_of_gl_type(gl_type: GLenum) -> Option<usize> {
    match gl_type {
        gl::FLOAT => Some(mem::size_of::<GLfloat>()),
        gl::INT => Some(mem::size_of::<GLint>()),
        gl::UNSIGNED_INT => Some(mem::size_of::<GLuint>()),
        gl::BYTE => Some(mem::size_of::<GLbyte>()),
        gl::UNSIGNED_BYTE => Some(mem::size_of::<GLubyte>()),
        gl::SHORT => Some(mem::size_of::<GLshort>()),
        gl::BOOL => Some(mem::size_of::<GLboolean>()),
        _ => {
            warn!("Could not find the size of gl type {}", gl_type);
            None
        }
    }
}
