use colors;
use units;

#[derive(Clone, Copy, Debug)]
pub struct Pen {
    pub cursor_loc: units::ScreenTile2D,
    pub fg: colors::Rgb,
    pub bg: colors::Rgb,
}

impl Pen {
    pub fn new() -> Self {
        Pen {
            cursor_loc: units::ScreenTile2D::zero(),
            fg: [0.0, 0.0, 0.0],
            bg: [0.0, 0.0, 0.0],
        }
    }

    #[allow(dead_code)]
    pub fn offset(self, offset: units::ScreenTile2D) -> Self {
        Pen {
            cursor_loc: self.cursor_loc + offset,
            ..self
        }
    }
}
