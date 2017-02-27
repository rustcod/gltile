/// tile based window location
/// independent of game location
///
/// e.g. `WindowLoc { x: 0, y: 0 }` is always the upper left tile of the screen
/// regardless of what game location that happens to be

use std;

use super::WindowLocOffset;

#[derive(Clone, Copy, Debug, Default)]
pub struct WindowLoc {
    pub x: i32,
    pub y: i32,
}

impl WindowLoc {
    pub fn new(x: i32, y: i32) -> Self {
        WindowLoc { x: x, y: y }
    }

    pub fn offset(self, other: WindowLocOffset) -> WindowLoc {
        self +
        WindowLoc {
            x: other.x,
            y: other.y,
        }
    }

    pub fn index(&self) -> usize {
        (self.x * self.y + self.x) as usize
    }
}

impl std::ops::Add for WindowLoc {
    type Output = WindowLoc;

    fn add(self, other: WindowLoc) -> WindowLoc {
        WindowLoc {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for WindowLoc {
    type Output = WindowLoc;

    fn sub(self, other: WindowLoc) -> WindowLoc {
        WindowLoc {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
