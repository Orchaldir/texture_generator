use crate::implementation::initialization::GliumInitialization;
use crate::implementation::input::{convert_key_code, convert_mouse_button};
use crate::interface::app::App;
use crate::interface::window::Window;
use crate::interface::Size2d;
use crate::logging::init_logging;
use core::cmp;
use glium::glutin::dpi::PhysicalPosition;
use glium::glutin::event::{ElementState, KeyboardInput, MouseButton};
use glium::{glutin, Display};
use std::cell::RefCell;
use std::ops::Sub;
use std::rc::Rc;

pub struct GliumWindow {
    title: &'static str,
    size: Size2d,
}

impl GliumWindow {
    pub fn new(title: &'static str, size: Size2d) -> GliumWindow {
        GliumWindow { title, size }
    }

    pub fn default_size(title: &'static str) -> GliumWindow {
        GliumWindow::new(title, (800, 600))
    }

    fn create_display(&self, event_loop: &glutin::event_loop::EventLoop<()>) -> Display {
        let size = glutin::dpi::LogicalSize::new(self.size.0, self.size.1);
        let wb = glutin::window::WindowBuilder::new()
            .with_title(self.title)
            .with_resizable(false)
            .with_inner_size(size);
        let cb = glutin::ContextBuilder::new();
        glium::Display::new(wb, cb, event_loop).unwrap()
    }
}

impl Window for GliumWindow {
    fn run(&mut self, app: Rc<RefCell<dyn App>>) -> ! {
        init_logging();

        let event_loop = glutin::event_loop::EventLoop::new();
        let display = self.create_display(&event_loop);
        let mut initialization = GliumInitialization::new(display);

        {
            let mut reference = app.borrow_mut();
            reference.init(&mut initialization);
        }

        let mut renderer = initialization.finish(self.size);
        let size = self.size;
        let mut last_rendering = std::time::Instant::now();
        let mut mouse_point = None;

        info!("Initialization finished");

        event_loop.run(move |event, _, control_flow| {
            *control_flow = run_with_frequency(60);

            match event {
                glutin::event::Event::NewEvents(event) => match event {
                    glutin::event::StartCause::ResumeTimeReached { .. } => {}
                    glutin::event::StartCause::WaitCancelled { .. } => {}
                    _ => return,
                },
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                        handle_keyboard_input(&app, input);
                        return;
                    }
                    glutin::event::WindowEvent::CursorMoved { position, .. } => {
                        mouse_point = calculate_mouse_point(size, position);
                        return;
                    }
                    glutin::event::WindowEvent::MouseInput { state, button, .. } => {
                        if let Some(point) = mouse_point {
                            handle_mouse_input(&app, point, state, button);
                        }
                        return;
                    }
                    _ => return,
                },
                glutin::event::Event::RedrawRequested(_) => (),
                _ => return,
            }

            let start = std::time::Instant::now();

            let mut reference = app.borrow_mut();
            reference.render(&mut renderer);

            analyze_performance(start, &mut last_rendering);
        });
    }
}

fn run_with_frequency(frequency: u32) -> glutin::event_loop::ControlFlow {
    let next_frame_time =
        std::time::Instant::now() + std::time::Duration::from_secs_f32(1.0 / frequency as f32);
    glutin::event_loop::ControlFlow::WaitUntil(next_frame_time)
}

fn handle_keyboard_input(app: &Rc<RefCell<dyn App>>, input: KeyboardInput) {
    if input.state == glutin::event::ElementState::Released {
        if let Some(glutin_key) = input.virtual_keycode {
            if let Some(key) = convert_key_code(glutin_key) {
                info!("Pressed key {:?}", key);
                let mut reference = app.borrow_mut();
                reference.on_key_released(key);
            } else {
                warn!("Unsupported key {:?}", glutin_key);
            }
        }
    }
}

fn handle_mouse_input(
    app: &Rc<RefCell<dyn App>>,
    mouse_point: (u32, u32),
    state: ElementState,
    button: MouseButton,
) {
    if state == glutin::event::ElementState::Released {
        if let Some(button) = convert_mouse_button(button) {
            info!("{:?} mouse click at {:?}", button, mouse_point);
            let mut reference = app.borrow_mut();
            reference.on_button_released(button, mouse_point);
        }
    }
}

fn calculate_mouse_point(size: Size2d, position: PhysicalPosition<f64>) -> Option<(u32, u32)> {
    let x = position.x as u32;
    let y = position.y as u32;

    if x > size.0 || y > size.1 {
        return None;
    }

    let y = cmp::max(size.1 - y, 1) - 1;
    Some((x, y))
}

fn analyze_performance(start: std::time::Instant, last_rendering: &mut std::time::Instant) {
    let duration_since_last = start.sub(*last_rendering);
    trace!("{:?} since last rendering", duration_since_last);
    let end = std::time::Instant::now();
    let duration = end.sub(start);
    trace!("Finished after {:?}", duration);
    *last_rendering = end;
}
