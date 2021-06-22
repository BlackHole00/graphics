use gl::types::*;
use utils::size_of_gl_type;
use crate::prelude::{gl_call, VaoObject};
use std::ffi::c_void;

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
