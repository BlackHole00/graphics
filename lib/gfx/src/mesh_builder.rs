use std::fmt::Debug;

use crate::Mesh;

pub struct MeshBuilder<T>
where
    T: Sized + Copy + Debug,
{
    vertices: Vec<T>,
    indices: Vec<u32>,
    indices_used: u32,
}

impl<T> MeshBuilder<T>
where
    T: Sized + Copy + Debug,
{
    pub fn new() -> MeshBuilder<T> {
        MeshBuilder {
            vertices: Vec::new(),
            indices: Vec::new(),
            indices_used: 0,
        }
    }

    pub fn push_triangle(&mut self, vertices: &[T; 3]) {
        vertices.into_iter().for_each(|x| self.vertices.push(*x));

        self.indices.push(self.indices_used);
        self.indices.push(self.indices_used + 1);
        self.indices.push(self.indices_used + 2);
        self.indices_used += 3;
    }

    pub fn push_quad(&mut self, vertices: &[T; 4]) {
        vertices.into_iter().for_each(|x| self.vertices.push(*x));

        self.indices.push(self.indices_used);
        self.indices.push(self.indices_used + 1);
        self.indices.push(self.indices_used + 2);
        self.indices.push(self.indices_used + 2);
        self.indices.push(self.indices_used + 3);
        self.indices.push(self.indices_used);
        self.indices_used += 4;
    }

    pub fn build(self) -> Mesh {
        //log::info!("{:?}\n{:?}", self.vertices, self.indices);

        Mesh::from_data(self.vertices, self.indices)
    }
}
