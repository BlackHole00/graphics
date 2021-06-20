use cgmath::{Point3, Vector3};

pub trait Movable {
    fn get_position(&mut self) -> &mut Point3<f32>;
    fn move_forward(&mut self, direction: Vector3<f32>, amount: f32);
    fn simple_move(&mut self, direction: Vector3<f32>, amount: f32);
    fn set_position(&mut self, position: Point3<f32>);
}

pub trait Rotable {
    fn get_rotation(&mut self) -> &mut Vector3<f32>;
    fn rotate(&mut self, rotation: Vector3<f32>, amount: f32);
    fn set_rotation(&mut self, rotation: Vector3<f32>);
}
