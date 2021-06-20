use glutin::{
    dpi::PhysicalPosition,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
    ContextBuilder, GlRequest,
};
use log::info;
use simple_logger::SimpleLogger;
use std::time::Instant;
use winit_input_helper::WinitInputHelper;

use super::Application;

pub struct Window {
    windowed_context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    event_loop: EventLoop<()>,
    input_helper: WinitInputHelper,
}

impl Window {
    pub fn new(window_builder: WindowBuilder) -> Window {
        SimpleLogger::new().init().unwrap();
        info!("Started logger");

        info!("Creating window");
        let event_loop = EventLoop::new();

        info!("Starting input helper");
        let input_helper = WinitInputHelper::new();

        info!("Creating windowed context");
        let windowed_context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
            .build_windowed(window_builder, &event_loop)
            .unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };
        windowed_context.window().set_cursor_visible(false);

        info!("Window successfully crated");

        info!("Loading OpenGL");
        gl::load_with(|s| windowed_context.get_proc_address(s) as *const _);
        info!("Successfully loaded OpenGL");

        Window {
            windowed_context,
            event_loop,
            input_helper,
        }
    }

    pub fn run<T>(&mut self)
    where
        T: Application,
    {
        let mut now: Instant = Instant::now();
        let mut delta: f64 = 0.0;

        let windowed_context = &mut self.windowed_context;
        let input_helper = &mut self.input_helper;

        info!("Loading Init Function");
        let mut application = T::init();

        info!("Starting Application Main Loop");
        self.event_loop
            .run_return(move |event, _, mut control_flow| {
                *control_flow = ControlFlow::default();

                windowed_context
                    .window()
                    .set_cursor_position(PhysicalPosition::new(0.0, 0.0))
                    .unwrap();
                if input_helper.update(&event) {
                    application.logic(input_helper, &mut control_flow, delta);
                }

                delta = now.elapsed().as_secs_f64();
                now = Instant::now();

                application.draw();
                windowed_context.swap_buffers().unwrap();

                match event {
                    Event::LoopDestroyed => {
                        info!("Closing Application");
                        application.close();
                    }
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::Resized(physical_size) => {
                            windowed_context.resize(physical_size);
                            application
                                .resize(physical_size.width as i32, physical_size.height as i32);
                        }
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        _ => (),
                    },
                    Event::RedrawRequested(_) => {
                        windowed_context.swap_buffers().unwrap();
                    }
                    _ => (),
                }
            });
    }
}
