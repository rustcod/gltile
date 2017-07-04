/// a 2D pixel located on the screen
///
/// e.g. `ScreenPixel2D::new(0, 0)` is always the upper left pixel of the screen
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ScreenPixel2D {
    pub x: i32,
    pub y: i32,
}

impl ScreenPixel2D {
    pub fn new(x: i32, y: i32) -> Self {
        ScreenPixel2D { x, y }
    }

    pub fn zero() -> Self {
        ScreenPixel2D::new(0, 0)
    }
}

impl ::std::ops::Add for ScreenPixel2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ScreenPixel2D::new(self.x + other.x, self.y + other.y)
    }
}

impl ::std::ops::Sub for ScreenPixel2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        ScreenPixel2D::new(self.x - other.x, self.y - other.y)
    }
}

impl ::std::convert::From<(i32, i32)> for ScreenPixel2D {
    fn from(other: (i32, i32)) -> Self {
        ScreenPixel2D::new(other.0, other.1)
    }
}
