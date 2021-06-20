extern crate gl;
use std::{ffi::c_void, mem};

use super::Bindable;
use crate::{derives::VboObject, gl_call};
use gl::types::*;

pub trait VboObject: Bindable {}

#[derive(Clone, VboObject)]
pub struct Vbo(GLuint);

impl Vbo {
    pub fn new() -> Vbo {
        let mut vbo_id = 0;
        gl_call!(gl::CreateBuffers(1, &mut vbo_id));

        Vbo(vbo_id)
    }

    pub fn add_data<T>(&self, data: &T, draw_type: GLenum)
    where
        T: ?Sized,
    {
        self.bind();

        gl_call!(gl::BufferData(
            gl::ARRAY_BUFFER,
            mem::size_of_val(data) as isize,
            data as *const T as *const c_void,
            draw_type,
        ));
    }

    #[inline]
    pub fn add_raw_data(&self, data: *const c_void, size: usize, draw_type: GLenum) {
        self.bind();

        gl_call!(gl::BufferData(
            gl::ARRAY_BUFFER,
            size as isize,
            data,
            draw_type
        ));
    }
}

impl Bindable for Vbo {
    #[inline]
    fn bind(&self) {
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, self.0));
    }

    #[inline]
    fn unbind(&self) {
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, 0));
    }
}

impl Drop for Vbo {
    fn drop(&mut self) {
        gl_call!(gl::DeleteBuffers(1, &mut self.0));
    }
}
