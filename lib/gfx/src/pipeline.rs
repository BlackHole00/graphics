use crate::{
    basic::{Bindable, Shader, ShaderObject, ShaderUniformable, Vao, VaoObject},
    derives::VaoObject,
};
use std::path::Path;

#[derive(VaoObject)]
pub struct Pipeline {
    vao: Vao,
    shader: Shader,
    pub states: PipelineStates,
}

impl Pipeline {
    pub fn new(shader_vertex_source: &Path, shader_fragment_source: &Path) -> Pipeline {
        Pipeline {
            vao: Vao::new(),
            shader: Shader::new(shader_vertex_source, shader_fragment_source),
            states: PipelineStates::default(),
        }
    }

    pub fn from_existing(shader: Shader, vao: Vao) -> Pipeline {
        Pipeline {
            vao,
            shader,
            states: PipelineStates::default(),
        }
    }

    pub fn vao(&self) -> &Vao {
        &self.vao
    }

    pub fn shader(&self) -> &Shader {
        &self.shader
    }
}

impl Bindable for Pipeline {
    fn bind(&self) {
        self.vao.bind();
        self.shader.bind();

        //  TODO: States
    }

    fn unbind(&self) {
        self.vao.unbind();
        self.shader.unbind();
    }
}

impl ShaderObject for Pipeline {
    #[inline]
    fn set_uniform<T: ShaderUniformable>(&self, uniform_name: &str, value: T) {
        self.shader.bind();
        self.shader.set_uniform(uniform_name, value);
    }
}

pub struct PipelineStates {}

impl Default for PipelineStates {
    fn default() -> Self {
        PipelineStates {}
    }
}
