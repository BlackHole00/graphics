use gl::types::GLint;
use crate::prelude::{Bindable};
use super::ShaderUniform;


pub trait ShaderObject: Bindable {
    fn set_uniform<T: ShaderUniform>(&mut self, uniform_name: &str, value: T);
    fn get_uniform_location(&mut self, uniform_name: &str) -> GLint;
}