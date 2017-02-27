/// a generic struct for something with width and height
/// can be any units i.e. not just "tiles"
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    pub fn new(width: i32, height: i32) -> Self {
        Size {
            width: width,
            height: height,
        }
    }

    #[allow(dead_code)]
    pub fn perimeter(&self) -> usize {
        (self.width * 2 + self.height * 2 - 4) as usize
    }

    pub fn len(&self) -> usize {
        (self.width * self.height) as usize
    }
}
