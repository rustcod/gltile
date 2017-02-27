use data;
use pixset;
use rgb;

use super::{Console, Pen};

pub struct WithParams<'a> {
    pub console: &'a mut Console,
    pub pen: Pen,
}

impl<'a> WithParams<'a> {
    #[allow(dead_code)]
    pub fn with_loc(&mut self, wl: data::WindowLoc) -> &mut Self {
        self.pen.cursor_loc = wl;
        self
    }

    #[allow(dead_code)]
    pub fn with_fg(&mut self, fg: rgb::RGB) -> &mut Self {
        self.pen.fg = fg;
        self
    }

    #[allow(dead_code)]
    pub fn with_bg(&mut self, bg: rgb::RGB) -> &mut Self {
        self.pen.bg = bg;
        self
    }

    pub fn set_pix(&mut self, pix: pixset::Pix) {
        self.console.set_with_pen(pix, self.pen);
    }
}
