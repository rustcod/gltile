use super::{Pen, Tile, Tiles, WithParams};
use colors;
use pixset;
use units;

pub struct Console {
    pub size: units::Size2D,
    tiles: Tiles,
    pen: Pen,
}

impl Console {
    pub fn new(size: units::Size2D) -> Self {
        Console {
            size: size,
            tiles: Tiles::new(size),
            pen: Pen::new(),
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        // TODO make a getter or impl Index, in either case remove pub on tiles
        self.tiles.tiles[(self.size.width * y + x) as usize]
    }

    pub fn set_pt(&mut self, screen_point: units::ScreenPoint2D) -> &mut Self {
        self.pen.cursor_pt = screen_point;
        self
    }

    pub fn set_fg(&mut self, color: colors::Rgb) -> &mut Self {
        self.pen.fg = color;
        self
    }

    pub fn set_bg(&mut self, color: colors::Rgb) -> &mut Self {
        self.pen.bg = color;
        self
    }

    #[allow(dead_code)]
    pub fn with_pt(&mut self, screen_point: units::ScreenPoint2D) -> WithParams {
        let fg = self.pen.fg;
        let bg = self.pen.bg;

        // TODO new
        WithParams {
            console: self,
            pen: Pen {
                cursor_pt: screen_point,
                fg: fg,
                bg: bg,
            },
        }
    }

    #[allow(dead_code)]
    pub fn with_offset(&mut self, offset: units::ScreenPoint2D) -> WithParams {
        let fg = self.pen.fg;
        let bg = self.pen.bg;
        let screen_point = self.pen.cursor_pt + offset;

        // TODO new
        WithParams {
            console: self,
            pen: Pen {
                cursor_pt: screen_point,
                fg: fg,
                bg: bg,
            },
        }
    }

    #[allow(dead_code)]
    pub fn with_fg(&mut self, fg: colors::Rgb) -> WithParams {
        let cursor_pt = self.pen.cursor_pt;
        let bg = self.pen.bg;

        // TODO new
        WithParams {
            console: self,
            pen: Pen {
                cursor_pt: cursor_pt,
                fg: fg,
                bg: bg,
            },
        }
    }

    #[allow(dead_code)]
    pub fn with_bg(&mut self, bg: colors::Rgb) -> WithParams {
        let cursor_pt = self.pen.cursor_pt;
        let fg = self.pen.fg;

        // TODO new
        WithParams {
            console: self,
            pen: Pen {
                cursor_pt: cursor_pt,
                fg: fg,
                bg: bg,
            },
        }
    }

    #[allow(dead_code)]
    pub fn set_pix<'a>(&mut self, pix: pixset::Pix) {
        let pen = self.pen.clone();
        self.set_with_pen(pix, pen);
    }

    pub fn set_with_pen<'a>(&mut self, pix: pixset::Pix, pen: Pen) {
        self.tiles.set(pen.cursor_pt, pix, pen.fg, pen.bg);
    }

    // +x is right
    // +y is down
    pub fn set_rect(&mut self, x: u32, y: u32) {
        for d_x in 0..x {
            for d_y in 0..y {
                let offset = units::ScreenPoint2D::new(d_x as i32, d_y as i32);
                self.with_offset(offset).set_pix(pixset::Pix::Empty);
            }
        }
    }
}
