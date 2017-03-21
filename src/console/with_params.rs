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
    pub fn with_loc(&mut self, loc: units::ScreenTile2D) -> &mut Self {
        self.pen.cursor_loc = loc;
        self
    }

    #[allow(dead_code)]
    pub fn with_fg(&mut self, fg: colors::Rgb) -> &mut Self {
        self.pen.fg = fg;
        self
    }

    #[allow(dead_code)]
    pub fn with_bg(&mut self, bg: colors::Rgb) -> &mut Self {
        self.pen.bg = bg;
        self
    }

    pub fn set_pix(&mut self, pix: pixset::Pix) {
        self.console.set_with_pen(pix, self.pen);
    }
}
