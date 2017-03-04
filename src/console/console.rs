use super::{Pen, Tile, Tiles, WithParams};
use colors;
use data;
use pixset;

pub struct Console {
    pub size: data::Size,
    tiles: Tiles,
    pen: Pen,
}

impl Console {
    pub fn new(size: data::Size) -> Self {
        Console {
            size: size,
            tiles: Tiles::new(size),
            pen: Default::default(),
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        // TODO make a getter or impl Index, in either case remove pub on tiles
        self.tiles.tiles[(self.size.width * y + x) as usize]
    }

    pub fn set_loc(&mut self, window_loc: data::WindowLoc) -> &mut Self {
        self.pen.cursor_loc = window_loc;
        self
    }

    pub fn set_fg(&mut self, color: colors::Srgb) -> &mut Self {
        self.pen.fg = color;
        self
    }

    pub fn set_bg(&mut self, color: colors::Srgb) -> &mut Self {
        self.pen.bg = color;
        self
    }

    #[allow(dead_code)]
    pub fn with_loc(&mut self, window_loc: data::WindowLoc) -> WithParams {
        let fg = self.pen.fg;
        let bg = self.pen.bg;

        // TODO new
        WithParams {
            console: self,
            pen: Pen {
                cursor_loc: window_loc,
                fg: fg,
                bg: bg,
            },
        }
    }

    #[allow(dead_code)]
    pub fn with_loc_offset(&mut self, offset: data::WindowLocOffset) -> WithParams {
        let fg = self.pen.fg;
        let bg = self.pen.bg;
        let window_loc = self.pen.cursor_loc.offset(offset);

        // TODO new
        WithParams {
            console: self,
            pen: Pen {
                cursor_loc: window_loc,
                fg: fg,
                bg: bg,
            },
        }
    }

    #[allow(dead_code)]
    pub fn with_fg(&mut self, fg: colors::Srgb) -> WithParams {
        let cursor_loc = self.pen.cursor_loc;
        let bg = self.pen.bg;

        // TODO new
        WithParams {
            console: self,
            pen: Pen {
                cursor_loc: cursor_loc,
                fg: fg,
                bg: bg,
            },
        }
    }

    #[allow(dead_code)]
    pub fn with_bg(&mut self, bg: colors::Srgb) -> WithParams {
        let cursor_loc = self.pen.cursor_loc;
        let fg = self.pen.fg;

        // TODO new
        WithParams {
            console: self,
            pen: Pen {
                cursor_loc: cursor_loc,
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
        self.tiles.set(pen.cursor_loc, pix, pen.fg, pen.bg);
    }

    // +x is right
    // +y is down
    pub fn set_rect(&mut self, x: u32, y: u32) {
        for d_x in 0..x {
            for d_y in 0..y {
                let offset = data::WindowLocOffset::new(d_x as i32, d_y as i32);
                self.with_loc_offset(offset)
                    .set_pix(pixset::Pix::Empty);
            }
        }
    }
}
