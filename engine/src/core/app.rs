pub trait App {
    fn init(&mut self) {}
    fn exit(&mut self) {}
    fn load(&mut self) {}
    fn unload(&mut self) {}
    fn update(&mut self, _delta_time: f32) {}
    fn render(&mut self) {}
}
