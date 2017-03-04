use super::{Console, Pen};
use colors;
use pixset;
use units;

pub struct WithParams<'a> {
    pub console: &'a mut Console,
    pub pen: Pen,
}

impl<'a> WithParams<'a> {
    #[allow(dead_code)]
    pub fn with_pt(&mut self, pt: units::ScreenPoint2D) -> &mut Self {
        self.pen.cursor_pt = pt;
        self
    }

    #[allow(dead_code)]
    pub fn with_fg(&mut self, fg: colors::Srgb) -> &mut Self {
        self.pen.fg = fg;
        self
    }

    #[allow(dead_code)]
    pub fn with_bg(&mut self, bg: colors::Srgb) -> &mut Self {
        self.pen.bg = bg;
        self
    }

    pub fn set_pix(&mut self, pix: pixset::Pix) {
        self.console.set_with_pen(pix, self.pen);
    }
}
