extern crate gl;
use std::{ffi::c_void, mem};

use self::gl::types::*;
use super::{Bindable, Vao};
use crate::{derives::EboObject, gl_call};

pub trait EboObject: Bindable {}

#[derive(Clone, EboObject)]
pub struct Ebo(GLuint);

#[allow(dead_code)]
impl Ebo {
    pub fn new() -> Ebo {
        let mut ebo_id = 0;
        gl_call!(gl::CreateBuffers(1, &mut ebo_id));

        Ebo(ebo_id)
    }

    pub fn add_data<T>(&self, data: &T, draw_type: GLenum)
    where
        T: ?Sized,
    {
        self.bind();

        gl_call!(gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            mem::size_of_val(data) as isize,
            data as *const T as *const c_void,
            draw_type,
        ));
    }

    #[inline]
    pub fn add_raw_data(&self, data: *const c_void, size: usize, draw_type: GLenum) {
        self.bind();
        gl_call!(gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            size as isize,
            data,
            draw_type
        ));
    }

    #[inline]
    pub fn bind_to_vao(&self, vao: &Vao) {
        vao.bind();
        self.bind();
    }
}

impl Bindable for Ebo {
    #[inline]
    fn bind(&self) {
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.0));
    }

    #[inline]
    fn unbind(&self) {
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0));
    }
}

impl Drop for Ebo {
    fn drop(&mut self) {
        gl_call!(gl::DeleteBuffers(1, &mut self.0));
    }
}
