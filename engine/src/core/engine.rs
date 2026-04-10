use std::time::Instant;

use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop,
    window::WindowAttributes,
};

use crate::{core::app::App, platform::window::Window};

pub struct Engine<A: App> {
    window: Window,
    app: A,

    last_frame: Instant,
}

impl<A> Engine<A>
where
    A: App,
{
    pub fn new(app: A) -> Engine<A> {
        Engine {
            window: Window {
                inner: None,
                title: "Vertex".to_string(),
                width: 800,
                height: 600,
            },
            app,
            last_frame: Instant::now(),
        }
    }

    pub fn run(&mut self) {
        println!("Running Vertex application...");

        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(self).unwrap();
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.window.title = title.to_string();
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.window.width = width;
        self.window.height = height;
        self
    }
}

impl<A> ApplicationHandler for Engine<A>
where
    A: App
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title(self.window.title.clone())
                    .with_inner_size(winit::dpi::LogicalSize::new(
                        self.window.width,
                        self.window.height,
                    )),
            )
            .unwrap();

        self.window.inner = Some(window);

        self.app.init();
        self.app.load();
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;

        self.app.update(dt);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                self.app.render();
            }
            WindowEvent::CloseRequested => {
                self.app.unload();
                self.app.exit();
                event_loop.exit();
            }
            _ => {}
        }
    }
}
