/// tile based game location

use std;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Loc {
    pub x: i32,
    pub y: i32,
}

impl std::ops::Add for Loc {
    type Output = Loc;

    fn add(self, other: Loc) -> Loc {
        Loc {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Loc {
    type Output = Loc;

    fn sub(self, other: Loc) -> Loc {
        Loc {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
