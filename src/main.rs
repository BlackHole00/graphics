use cgmath::{vec3, Matrix4};
use gfx::{
    basic::{Camera, Texture, TextureBuilder, VaoLayout},
    gl_call,
    prelude::*,
    Mesh, MeshBuilder, Pipeline,
};
use glutin::{event::VirtualKeyCode, event_loop::ControlFlow};
use logic::transform::{Movable, Rotable};
use std::path::Path;
use window::{main_app, Application};
use winit_input_helper::WinitInputHelper;

#[allow(dead_code)]
#[main_app(with_title("OpenGL"), with_inner_size(LogicalSize::new(800, 600)))]
struct App {
    pipeline: Pipeline,
    mesh: Mesh,
    texture1: Texture,
    texture2: Texture,

    color: f32,
    mode: bool,

    camera: Camera,
}

impl Application for App {
    fn init() -> App {
        gl_call!(gl::Enable(gl::DEPTH_TEST));
        gl_call!(gl::Enable(gl::BLEND));

        let mut pipeline = Pipeline::new(
            Path::new("res/shaders/basic.vs"),
            Path::new("res/shaders/basic.fs"),
        );

        pipeline.bind();

        let mut meshbuilder: MeshBuilder<[f32; 5]> = MeshBuilder::new();
        meshbuilder.push_quad(&[
            [-0.5, -0.5, -0.5, 0.0, 0.0],
            [ 0.5, -0.5, -0.5, 1.0, 0.0],
            [ 0.5,  0.5, -0.5, 1.0, 1.0],
            [-0.5,  0.5, -0.5, 0.0, 1.0],
        ]);
        meshbuilder.push_quad(&[
            [-0.5, -0.5, 0.5, 0.0, 0.0],
            [ 0.5, -0.5, 0.5, 1.0, 0.0],
            [ 0.5,  0.5, 0.5, 1.0, 1.0],
            [-0.5,  0.5, 0.5, 0.0, 1.0],
        ]);
        let mesh = meshbuilder.build();

        let mut vao_layout = VaoLayout::new();
        vao_layout.push_element(3, gl::FLOAT, false);
        vao_layout.push_element(2, gl::FLOAT, false);
        vao_layout.apply_layout(&pipeline);

        let texture1 =
            TextureBuilder::from_file(&Path::new("res/textures/container.jpg"), false, false)
                .build();
        texture1.bind();
        pipeline.set_uniform("texture1", &texture1);

        let texture2 = TextureBuilder::from_file(&Path::new("res/textures/wall.jpg"), false, true)
            .active_texture_number(gl::TEXTURE1)
            .build();
        texture2.bind();
        pipeline.set_uniform("texture2", &texture2);

        let model_mat: Matrix4<f32> = Matrix4::from_translation(vec3(0.0, 0.0, -2.0));
        pipeline.set_uniform("model", model_mat);

        let mut camera = Camera::new(800, 600, 100.0);
        camera.bind_to_shader(&mut pipeline);

        pipeline.set_uniform("mixValue", 0.5);

        App {
            pipeline,
            mesh,
            texture1,
            texture2,
            color: 0.0,
            mode: false,
            camera,
        }
    }

    fn draw(&mut self) {
        self.mesh.bind();

        gl_call!(gl::ClearColor(self.color, self.color, self.color, 1.0));
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));
        self.pipeline
            .set_uniform("mixValue", (90.0 * self.color).to_radians().sin());

        self.mesh.draw();
    }

    fn logic(&mut self, input: &mut WinitInputHelper, control_flow: &mut ControlFlow, delta: f64) {
        check_camera_inputs(&mut self.camera, &mut self.pipeline, input, delta);

        if input.key_held(VirtualKeyCode::Z) {
            gl_call!(gl::PolygonMode(gl::FRONT_AND_BACK , gl::LINE));
        } else {
            gl_call!(gl::PolygonMode(gl::FRONT_AND_BACK , gl::FILL))
        }

        if input.key_pressed(VirtualKeyCode::Escape) {
            *control_flow = ControlFlow::Exit;
        }

        let mut offset = (delta / 2.0) as f32;
        if self.mode {
            offset *= -1.0;
        }
        self.color += offset;
        if self.color > 1.0 || self.color < 0.0 {
            self.mode = !self.mode;
        }
    }

    fn resize(&mut self, width: i32, height: i32) {
        unsafe { gl::Viewport(0, 0, width as i32, height as i32) }
    }

    fn close(&mut self) {}
}

fn check_camera_inputs(
    camera: &mut Camera,
    pipeline: &mut Pipeline,
    input: &mut WinitInputHelper,
    delta: f64,
) {
    if input.key_held(VirtualKeyCode::W) {
        camera.move_forw(3.0 * delta as f32);
        camera.bind_to_shader(pipeline);
    } else if input.key_held(VirtualKeyCode::S) {
        camera.move_back(3.0 * delta as f32);
        camera.bind_to_shader(pipeline);
    }
    if input.key_held(VirtualKeyCode::D) {
        camera.move_right(3.0 * delta as f32);
        camera.bind_to_shader(pipeline);
    } else if input.key_held(VirtualKeyCode::A) {
        camera.move_left(3.0 * delta as f32);
        camera.bind_to_shader(pipeline);
    }
    if input.key_held(VirtualKeyCode::Space) {
        camera.simple_move(vec3(0.0, 1.0, 0.0), 3.0 * delta as f32);
        camera.bind_to_shader(pipeline);
    } else if input.key_held(VirtualKeyCode::LShift) {
        camera.simple_move(vec3(0.0, -1.0, 0.0), 3.0 * delta as f32);
        camera.bind_to_shader(pipeline);
    }

    if let Some((pos_x, pos_y)) = input.mouse() {
        const SENSIBILITY: f32 = 0.15;

        camera.rotate(vec3(-1.0, 0.0, 0.0), pos_x * SENSIBILITY);
        camera.rotate(vec3(0.0, 1.0, 0.0), pos_y * SENSIBILITY);

        camera.bind_to_shader(pipeline);
    }
}
