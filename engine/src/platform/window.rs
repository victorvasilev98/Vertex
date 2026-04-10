use winit::{
    window::Window as WinitWindow
};

pub struct Window {
    pub inner: Option<WinitWindow>,
    
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Window {
    
}