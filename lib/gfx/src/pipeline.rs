use gl::types::*;
use crate::{basic::{Bindable, Shader, ShaderObject, ShaderUniform, Vao, VaoObject}, derives::VaoObject, gl_call};
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

    pub fn update_states(&self) {
        match self.states.depth_test {
            Some(mode) => {
                gl_call!(gl::Enable(gl::DEPTH_TEST));
                gl_call!(gl::DepthFunc(mode));
            },
            None => {
                gl_call!(gl::Disable(gl::DEPTH_TEST));
            },
        }

        match self.states.blend {
            Some((sfactor, dfactor)) => {
                gl_call!(gl::Enable(gl::BLEND));
                gl_call!(gl::BlendFunc(sfactor, dfactor));
            },
            None => {
                gl_call!(gl::Disable(gl::BLEND));
            }
        }

        match self.states.cull_face {
            Some((face, mode)) => {
                gl_call!(gl::Enable(gl::CULL_FACE));
                gl_call!(gl::CullFace(face));
                gl_call!(gl::FrontFace(mode));
            },
            None => {
                gl_call!(gl::Disable(gl::CULL_FACE));
            },
        }

        gl_call!(gl::PolygonMode(gl::FRONT_AND_BACK, self.states.polygon_mode));
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
        self.update_states();
        self.vao.bind();
        self.shader.bind();
    }

    fn unbind(&self) {
        self.vao.unbind();
        self.shader.unbind();
    }
}

impl ShaderObject for Pipeline {
    #[inline]
    fn set_uniform<T: ShaderUniform>(&mut self, uniform_name: &str, value: T) {
        self.shader.set_uniform(uniform_name, value);
    }

    #[inline]
    fn get_uniform_location(&mut self, uniform_name: &str) -> GLint {
        self.shader.get_uniform_location(uniform_name)
    }
}

pub struct PipelineStates {
    pub depth_test: Option<GLenum>,
    pub blend: Option<(GLenum, GLenum)>,
    pub cull_face: Option<(GLenum, GLenum)>,
    pub polygon_mode: GLenum,

    //  There will be more...
}

impl Default for PipelineStates {
    fn default() -> Self {
        PipelineStates {
            depth_test: None,
            blend: None,
            cull_face: None,
            polygon_mode: gl::FILL,
        }
    }
}
