/// a 2D Size consisting of a width and a height.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Size2D {
    pub width: i32,
    pub height: i32,
}

impl Size2D {
    pub fn new(width: i32, height: i32) -> Self {
        Size2D { width, height }
    }

    pub fn zero() -> Self {
        Size2D::new(0, 0)
    }
}

impl ::std::ops::Add for Size2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Size2D::new(self.width + other.width, self.height + other.height)
    }
}

impl ::std::ops::Sub for Size2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Size2D::new(self.width - other.width, self.height - other.height)
    }
}

impl ::std::convert::From<(i32, i32)> for Size2D {
    fn from(other: (i32, i32)) -> Self {
        Size2D::new(other.0, other.1)
    }
}
