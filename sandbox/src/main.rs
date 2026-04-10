use vertex::core::{app::App, engine::Engine};

struct SandboxApp;

impl App for SandboxApp {
    fn init(&mut self) {
        
    }

    fn exit(&mut self) {
        
    }

    fn load(&mut self) {
        
    }

    fn unload(&mut self) {
        
    }

    fn update(&mut self, delta_time: f32) {
        println!("dt: {delta_time}");
    }

    fn render(&mut self) {
        
    }
}

fn main() {
    Engine::new(SandboxApp)
        .with_title("Vertex Sandbox")
        .with_size(800, 600)
        .run();
}
