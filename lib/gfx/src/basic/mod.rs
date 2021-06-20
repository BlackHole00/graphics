mod camera;
mod ebo;
mod shader;
mod texture;
mod vao;
mod vbo;

pub use camera::Camera;
pub use ebo::{Ebo, EboObject};
pub use shader::{get_uniform_location, Shader, ShaderObject, ShaderUniformable};
pub use texture::{TextureData, Texture, TextureBuilder, TextureObject};
pub use vao::{Vao, VaoLayout, VaoObject};
pub use vbo::{Vbo, VboObject};

pub trait Bindable {
    fn bind(&self);
    fn unbind(&self);
}

pub trait Drawable: Bindable {
    fn draw(&self);

    fn draw_bind(&self) {
        self.bind();
        self.draw();
    }
}

pub fn get_gl_error_string(error: u32) -> Option<&'static str> {
    match error {
        0 => None,
        gl::INVALID_ENUM => Some("GlError: INVALID ENUM"),
        gl::INVALID_VALUE => Some("GlError: INVALID VALUE"),
        gl::INVALID_OPERATION => Some("GlError: INVALID OPERATION"),
        gl::STACK_OVERFLOW => Some("GlError: STACK OVERFLOW"),
        gl::STACK_UNDERFLOW => Some("GlError: STACK UNDERFLOW"),
        gl::OUT_OF_MEMORY => Some("GlError: OUT OF MEMORY"),
        gl::INVALID_FRAMEBUFFER_OPERATION => Some("GlError: INVALID FRAMEBUFFER OPERATION"),
        _ => Some("GlError: Unknown glError"),
    }
}
