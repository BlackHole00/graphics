use gl::types::*;
use crate::prelude::{gl_call, VaoObject, Bindable};

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