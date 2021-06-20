use super::ShaderObject;
use cgmath::{point3, prelude::*, vec3, Matrix4, Point3, SquareMatrix, Vector3};
use logic::transform::{Movable, Rotable};

pub struct Camera {
    screen_rateo: f32,

    position: Point3<f32>,
    rotation: Vector3<f32>,

    front: Vector3<f32>,

    fov: f32,
    render_distance: f32,

    consider_max_angle: bool,

    view_matrix: Matrix4<f32>,
    proj_matrix: Matrix4<f32>,
}

impl Camera {
    pub fn new(screen_width: i32, screen_height: i32, render_distance: f32) -> Camera {
        let mut camera = Camera {
            screen_rateo: screen_width as f32 / screen_height as f32,

            position: point3(0.0, 0.0, 0.0),
            rotation: vec3(-90.0, 0.0, 0.0),

            front: vec3(0.0, 0.0, -1.0),

            fov: 110.0,
            render_distance: render_distance,

            consider_max_angle: true,

            //  temporary values
            view_matrix: Matrix4::identity(),
            proj_matrix: Matrix4::identity(),
        };

        camera.apply_rotation();
        camera.update_proj_matrix();
        camera.update_view_matrix();

        camera
    }

    fn update_view_matrix(&mut self) {
        self.view_matrix = Matrix4::look_at_rh(
            self.position,
            self.position + self.front,
            vec3(0.0, 1.0, 0.0),
        );
    }

    fn update_proj_matrix(&mut self) {
        self.proj_matrix = cgmath::perspective(
            cgmath::Deg(self.fov),
            self.screen_rateo,
            0.1,
            self.render_distance,
        );
    }

    //  TODO: z rotation
    fn apply_rotation(&mut self) {
        {
            let rotation_y = &mut self.rotation[1];

            if self.consider_max_angle {
                if *rotation_y > 89.9 {
                    *rotation_y = 89.0;
                } else if *rotation_y < -89.0 {
                    *rotation_y = -89.0;
                }
            }
        }

        {
            let rotation_x = &mut self.rotation[0];
            //  Overflow avoiding...
            if *rotation_x > 360.0 {
                *rotation_x -= 360.0;
            } else if *rotation_x < -360.0 {
                *rotation_x += 360.0;
            }
        }

        self.front[0] = self.rotation[0].to_radians().cos() * self.rotation[1].to_radians().cos();
        self.front[1] = self.rotation[1].to_radians().sin();
        self.front[2] = self.rotation[0].to_radians().sin() * self.rotation[1].to_radians().cos();
    }

    pub fn bind_to_shader<T>(&mut self, shader: &mut T)
    where
        T: ShaderObject,
    {
        self.update_proj_matrix();
        self.update_view_matrix();

        shader.set_uniform("view", self.view_matrix);
        shader.set_uniform("proj", self.proj_matrix);
    }

    #[inline]
    pub fn move_forw(&mut self, amount: f32) {
        self.position += self.front * amount;
    }

    #[inline]
    pub fn move_back(&mut self, amount: f32) {
        self.position -= self.front * amount;
    }

    #[inline]
    pub fn move_left(&mut self, amount: f32) {
        self.position -= self.front.cross(vec3(0.0, 1.0, 0.0)).normalize() * amount;
    }

    #[inline]
    pub fn move_right(&mut self, amount: f32) {
        self.position += self.front.cross(vec3(0.0, 1.0, 0.0)).normalize() * amount;
    }
}

impl Movable for Camera {
    #[inline]
    fn get_position(&mut self) -> &mut Point3<f32> {
        &mut self.position
    }

    fn move_forward(&mut self, _direction: Vector3<f32>, _amount: f32) {
        todo!()

        /*let direction = direction.normalize();
        let mut movement = vec3(0.0, 0.0, 0.0);

        movement *= amount;

        log::info!("{:?}", movement);

        self.position -= movement;*/
    }

    #[inline]
    fn simple_move(&mut self, direction: Vector3<f32>, amount: f32) {
        self.position += direction.normalize() * amount;
    }

    #[inline]
    fn set_position(&mut self, position: Point3<f32>) {
        self.position = position;
    }
}

impl Rotable for Camera {
    #[inline]
    fn get_rotation(&mut self) -> &mut Vector3<f32> {
        &mut self.rotation
    }

    #[inline]
    fn rotate(&mut self, rotation: Vector3<f32>, amount: f32) {
        self.rotation += rotation.normalize() * -amount;
        self.apply_rotation();
    }

    #[inline]
    fn set_rotation(&mut self, rotation: Vector3<f32>) {
        self.rotation = rotation;
        self.apply_rotation();
    }
}
