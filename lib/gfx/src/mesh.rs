use crate::{
    basic::{Bindable, Drawable, Ebo, Vbo},
    gl_call,
};

pub struct Mesh {
    vertices: Vbo,
    indices: Ebo,
    indices_len: usize,
}

impl Mesh {
    pub fn new(vertices: Vbo, indices: Ebo, indices_len: usize) -> Mesh {
        Mesh {
            vertices,
            indices,
            indices_len,
        }
    }

    pub fn from_data<T>(vertices: Vec<T>, indices: Vec<u32>) -> Mesh {
        let vbo = Vbo::new();
        vbo.add_data(vertices.as_slice(), gl::STATIC_DRAW);
        //vbo.add_raw_data(vertices.as_ptr() as *const std::ffi::c_void, vertices.len() * std::mem::size_of::<T>(), gl::STATIC_DRAW);

        let ebo = Ebo::new();
        //ebo.add_raw_data(indices.as_ptr() as *const std::ffi::c_void, indices.len() * std::mem::size_of::<T>(), gl::STATIC_DRAW);
        ebo.add_data(indices.as_slice(), gl::STATIC_DRAW);

        Mesh {
            vertices: vbo,
            indices: ebo,
            indices_len: indices.len(),
        }
    }
}

impl Bindable for Mesh {
    fn bind(&self) {
        self.vertices.bind();
        self.indices.bind();
    }

    fn unbind(&self) {
        self.vertices.unbind();
        self.indices.unbind();
    }
}

impl Drawable for Mesh {
    fn draw(&self) {
        gl_call!(gl::DrawElements(
            gl::TRIANGLES,
            self.indices_len as i32,
            gl::UNSIGNED_INT,
            std::ptr::null()
        ));
    }
}
