extern crate gl;

use self::gl::types::*;
use super::Bindable;
use crate::gl_call;

use cgmath::{Matrix, Matrix4, Vector3};
use log::{error, warn};
use std::{ffi::CString, fs, path::Path, ptr};
use utilis::str_to_c_str_ptr;

pub trait ShaderObject: Bindable {
    fn set_uniform<T: ShaderUniformable>(&self, uniform_name: &str, value: T);
}

#[derive(Clone)]
pub struct Shader(GLuint);

#[allow(dead_code)]
impl Shader {
    pub fn new(vertex_source: &Path, fragment_source: &Path) -> Shader {
        let vertex_code = read_shader_source(vertex_source.as_os_str().to_str().unwrap()).unwrap();
        let fragment_code =
            read_shader_source(fragment_source.as_os_str().to_str().unwrap()).unwrap();
        let vertex_code = CString::new(vertex_code.as_bytes()).unwrap();
        let fragment_code = CString::new(fragment_code.as_bytes()).unwrap();

        let shader_id = {
            let shader_id;
            let vertex_shader;
            let fragment_shader;

            gl_call!(shader_id = gl::CreateProgram());

            gl_call!(vertex_shader = gl::CreateShader(gl::VERTEX_SHADER));
            gl_call!(fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER));

            gl_call!(gl::ShaderSource(
                vertex_shader,
                1,
                &vertex_code.as_ptr(),
                ptr::null()
            ));
            gl_call!(gl::ShaderSource(
                fragment_shader,
                1,
                &fragment_code.as_ptr(),
                ptr::null()
            ));

            gl_call!(gl::CompileShader(vertex_shader));
            check_shader_error(vertex_shader, &vertex_source);

            gl_call!(gl::CompileShader(fragment_shader));
            check_shader_error(fragment_shader, &fragment_source);

            gl_call!(gl::AttachShader(shader_id, vertex_shader));
            gl_call!(gl::AttachShader(shader_id, fragment_shader));
            gl_call!(gl::LinkProgram(shader_id));
            check_program_error(shader_id, &vertex_source, &fragment_source);

            gl_call!(gl::DeleteShader(vertex_shader));
            gl_call!(gl::DeleteShader(fragment_shader));

            shader_id
        };

        Shader(shader_id)
    }
}

impl ShaderObject for Shader {
    fn set_uniform<T: ShaderUniformable>(&self, uniform_name: &str, value: T) {
        self.bind();
        value.set_uniform(self.0, uniform_name);
    }
}

impl Bindable for Shader {
    #[inline]
    fn bind(&self) {
        gl_call!(gl::UseProgram(self.0));
    }

    #[inline]
    fn unbind(&self) {
        gl_call!(gl::UseProgram(0));
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        gl_call!(gl::DeleteProgram(self.0));
    }
}

fn read_shader_source(source: &str) -> Option<String> {
    let code = fs::read_to_string(source);

    match code {
        Ok(src) => Some(src),
        Err(_) => {
            error!("Could not open shader source at {}", source);
            None
        }
    }
}

fn check_shader_error(shader_id: GLuint, shader_path: &Path) {
    let mut result = 0;
    gl_call!(gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut result));
    if result == 0 {
        let mut info_log: Vec<u8> = Vec::with_capacity(1024);
        let mut info_len = 0;
        unsafe {
            info_log.set_len(1024);
        }

        gl_call!(gl::GetShaderInfoLog(
            shader_id,
            1023,
            &mut info_len,
            info_log.as_mut_ptr() as *mut GLchar,
        ));
        unsafe {
            info_log.set_len(info_len as usize);
        }

        warn!(
            "Could not compile shader {}:\n{}",
            shader_path.as_os_str().to_str().unwrap(),
            String::from_utf8_lossy(info_log.as_slice())
        );
    }
}

fn check_program_error(program_id: GLuint, vertex_path: &Path, fragment_path: &Path) {
    let mut result = 0;
    gl_call!(gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut result));
    if result == 0 {
        let mut info_log: Vec<u8> = Vec::with_capacity(1024);
        let mut info_len = 0;
        unsafe {
            info_log.set_len(1024);
        }

        gl_call!(gl::GetProgramInfoLog(
            program_id,
            1023,
            &mut info_len,
            info_log.as_mut_ptr() as *mut GLchar,
        ));
        unsafe {
            info_log.set_len(info_len as usize);
        }

        warn!(
            "Could not link shaders {} and {}:\n{}",
            vertex_path.as_os_str().to_str().unwrap(),
            fragment_path.as_os_str().to_str().unwrap(),
            String::from_utf8_lossy(info_log.as_slice())
        );
    }
}

pub fn get_uniform_location(shader_id: GLuint, uniform_name: &str) -> GLint {
    let uniform_location;

    gl_call!(uniform_location = gl::GetUniformLocation(shader_id, str_to_c_str_ptr!(uniform_name)));
    if uniform_location == -1 {
        log::info!(
            "Could not find uniform {} in shader {}",
            uniform_name,
            shader_id
        );
    }

    uniform_location
}

pub trait ShaderUniformable {
    fn set_uniform(&self, shader_id: GLuint, uniform_name: &str);
}

impl ShaderUniformable for i32 {
    fn set_uniform(&self, shader_id: GLuint, uniform_name: &str) {
        let uniform_location = get_uniform_location(shader_id, uniform_name);
        gl_call!(gl::Uniform1i(uniform_location, *self));
    }
}

impl ShaderUniformable for f32 {
    fn set_uniform(&self, shader_id: GLuint, uniform_name: &str) {
        let uniform_location = get_uniform_location(shader_id, uniform_name);
        gl_call!(gl::Uniform1f(uniform_location, *self));
    }
}

impl ShaderUniformable for Vector3<f32> {
    fn set_uniform(&self, shader_id: GLuint, uniform_name: &str) {
        let uniform_location = get_uniform_location(shader_id, uniform_name);
        gl_call!(gl::Uniform3f(uniform_location, self[0], self[1], self[2]));
    }
}

impl ShaderUniformable for Matrix4<f32> {
    fn set_uniform(&self, shader_id: GLuint, uniform_name: &str) {
        let uniform_location = get_uniform_location(shader_id, uniform_name);
        gl_call!(gl::UniformMatrix4fv(
            uniform_location,
            1,
            gl::FALSE,
            self.as_ptr()
        ));
    }
}
