use glutin::event_loop::ControlFlow;
use winit_input_helper::WinitInputHelper;

pub trait Application {
    fn init() -> Self;
    fn draw(&mut self);
    fn logic(&mut self, input: &mut WinitInputHelper, control_flow: &mut ControlFlow, delta: f64);
    fn resize(&mut self, width: i32, height: i32);
    fn close(&mut self);
}
