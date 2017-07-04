/// a 2D tile located on the screen
/// independent of game location
///
/// e.g. `ScreenTile2D::new(0, 0)` is always the upper left tile of the screen
/// regardless of what that game location that happens to be
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ScreenTile2D {
    pub x: i32,
    pub y: i32,
}

impl ScreenTile2D {
    pub fn new(x: i32, y: i32) -> Self {
        ScreenTile2D { x, y }
    }

    pub fn zero() -> Self {
        ScreenTile2D::new(0, 0)
    }
}

impl ::std::ops::Add for ScreenTile2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ScreenTile2D::new(self.x + other.x, self.y + other.y)
    }
}

impl ::std::ops::Sub for ScreenTile2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        ScreenTile2D::new(self.x - other.x, self.y - other.y)
    }
}

impl ::std::convert::From<(i32, i32)> for ScreenTile2D {
    fn from(other: (i32, i32)) -> Self {
        ScreenTile2D::new(other.0, other.1)
    }
}
