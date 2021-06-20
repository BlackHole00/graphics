pub use crate::prelude::{ShaderObject, gl_call};
use cgmath::{Vector3, Matrix4, Matrix};

pub trait ShaderUniform {
    fn set_uniform(&self, shader: &mut impl ShaderObject, uniform_name: &str);
}

impl ShaderUniform for i32 {
    fn set_uniform(&self, shader: &mut impl ShaderObject, uniform_name: &str) {
        let uniform_location = shader.get_uniform_location(uniform_name);
        gl_call!(gl::Uniform1i(uniform_location, *self));
    }
}

impl ShaderUniform for f32 {
    fn set_uniform(&self, shader: &mut impl ShaderObject, uniform_name: &str) {
        let uniform_location = shader.get_uniform_location(uniform_name);
        gl_call!(gl::Uniform1f(uniform_location, *self));
    }
}

impl ShaderUniform for Vector3<f32> {
    fn set_uniform(&self, shader: &mut impl ShaderObject, uniform_name: &str) {
        let uniform_location = shader.get_uniform_location(uniform_name);
        gl_call!(gl::Uniform3f(uniform_location, self[0], self[1], self[2]));
    }
}

impl ShaderUniform for Matrix4<f32> {
    fn set_uniform(&self, shader: &mut impl ShaderObject, uniform_name: &str) {
        let uniform_location = shader.get_uniform_location(uniform_name);
        gl_call!(gl::UniformMatrix4fv(
            uniform_location,
            1,
            gl::FALSE,
            self.as_ptr()
        ));
    }
}