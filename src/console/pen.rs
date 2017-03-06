use colors;
use units;

#[derive(Clone, Copy, Debug)]
pub struct Pen {
    pub cursor_pt: units::ScreenPoint2D,
    pub fg: colors::Rgb,
    pub bg: colors::Rgb,
}

impl Pen {
    pub fn new() -> Self {
        Pen {
            cursor_pt: units::ScreenPoint2D::zero(),
            fg: [0.0, 0.0, 0.0],
            bg: [0.0, 0.0, 0.0],
        }
    }

    #[allow(dead_code)]
    pub fn offset(self, offset: units::ScreenPoint2D) -> Self {
        Pen { cursor_pt: self.cursor_pt + offset, ..self }
    }
}
