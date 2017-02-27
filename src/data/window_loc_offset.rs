#[derive(Clone, Copy, Debug, Default)]
pub struct WindowLocOffset {
    pub x: i32,
    pub y: i32,
}

impl WindowLocOffset {
    pub fn new(x: i32, y: i32) -> Self {
        WindowLocOffset { x: x, y: y }
    }
}
