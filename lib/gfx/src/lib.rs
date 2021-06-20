pub mod basic;
pub mod derives;
pub mod prelude;

mod mesh;
mod mesh_builder;
mod pipeline;

pub use mesh::Mesh;
pub use mesh_builder::MeshBuilder;
pub use pipeline::Pipeline;

#[macro_export]
macro_rules! gl_call {
    ($l:expr) => {
        unsafe {
            use $crate::basic::get_gl_error_string;

            $l;

            while let Some(message) = get_gl_error_string(gl::GetError()) {
                log::warn!("{}", message);
            }
        }
    };
}
