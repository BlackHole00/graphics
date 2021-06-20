extern crate gl;
use std::ffi::c_void;

use self::gl::types::*;

use super::Bindable;
use crate::{derives::VaoObject, gl_call};
use utilis::size_of_gl_type;

pub trait VaoObject: Bindable {}

#[derive(Clone, VaoObject)]
pub struct Vao(GLuint);

impl Vao {
    pub fn new() -> Vao {
        let mut vao_id = 0;
        gl_call!(gl::GenVertexArrays(1, &mut vao_id));

        Vao(vao_id)
    }
}

impl Bindable for Vao {
    #[inline]
    fn bind(&self) {
        gl_call!(gl::BindVertexArray(self.0));
    }

    #[inline]
    fn unbind(&self) {
        gl_call!(gl::BindVertexArray(0));
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        gl_call!(gl::DeleteVertexArrays(1, &self.0));
    }
}

struct VaoLayoutElement {
    count: u8,
    gl_type: GLenum,
    normalized: GLboolean,
}

pub struct VaoLayout {
    layout: Vec<VaoLayoutElement>,
    stride: usize,
}

#[allow(dead_code)]
impl VaoLayout {
    pub fn new() -> VaoLayout {
        VaoLayout {
            layout: Vec::<VaoLayoutElement>::new(),
            stride: 0,
        }
    }

    pub fn push_element(&mut self, count: u8, gl_type: GLenum, normalized: bool) {
        self.layout.push(VaoLayoutElement {
            count,
            gl_type,
            normalized: normalized as GLboolean,
        });
        self.stride += count as usize * size_of_gl_type(gl_type).unwrap();
    }

    pub fn apply_layout<T>(&self, vao: &T)
    where
        T: VaoObject,
    {
        vao.bind();

        apply_layout(self);
    }

    pub fn apply_layout_raw(&self) {
        apply_layout(self);
    }
}

fn apply_layout(layout: &VaoLayout) {
    let mut offset = layout.stride;
    let size = layout.layout.len();

    for (index, elem) in layout.layout.iter().rev().enumerate() {
        let index = size - index - 1;

        offset -= size_of_gl_type(elem.gl_type).unwrap() * elem.count as usize;
        gl_call!(gl::VertexAttribPointer(
            index as u32,
            elem.count as i32,
            elem.gl_type,
            elem.normalized,
            layout.stride as i32,
            offset as *const c_void,
        ));
        gl_call!(gl::EnableVertexAttribArray(index as u32));
    }
}
