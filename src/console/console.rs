use super::{Pen, Tile, Tiles, WithParams};
use colors;
use pixset;
use pixset::PixLike;
use std::default::Default;
use units;

pub struct Console<P: PixLike> {
    size: units::Size2D,
    tiles: Tiles<P>,
    pen: Pen,
}

impl<P> Console<P>
where
    P: pixset::PixLike,
{
    pub fn new(size: units::Size2D) -> Self {
        Console {
            size: size,
            tiles: Tiles::new(size),
            pen: Pen::new(),
        }
    }

    pub fn get_width(&self) -> i32 {
        self.size.width
    }

    pub fn get_height(&self) -> i32 {
        self.size.height
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Tile<P> {
        self.tiles.tiles[(self.size.width * y + x) as usize]
    }

    pub fn set_loc(&mut self, screen_loc: units::ScreenTile2D) -> &mut Self {
        self.pen.cursor_loc = screen_loc;
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
    pub fn with_loc(&mut self, screen_loc: units::ScreenTile2D) -> WithParams<P> {
        let fg = self.pen.fg;
        let bg = self.pen.bg;

        // TODO new
        WithParams {
            console: self,
            pen: Pen {
                cursor_loc: screen_loc,
                fg: fg,
                bg: bg,
            },
        }
    }

    #[allow(dead_code)]
    pub fn with_offset(&mut self, offset: units::ScreenTile2D) -> WithParams<P> {
        let fg = self.pen.fg;
        let bg = self.pen.bg;
        let screen_loc = units::ScreenTile2D::new(
            self.pen.cursor_loc.x + offset.x,
            self.pen.cursor_loc.y + offset.y,
        );

        // TODO new
        WithParams {
            console: self,
            pen: Pen {
                cursor_loc: screen_loc,
                fg: fg,
                bg: bg,
            },
        }
    }

    #[allow(dead_code)]
    pub fn with_fg(&mut self, fg: colors::Rgb) -> WithParams<P> {
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
    pub fn with_bg(&mut self, bg: colors::Rgb) -> WithParams<P> {
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
    pub fn set_pix<'a>(&mut self, pix: P) {
        let pen = self.pen.clone();
        self.set_with_pen(pix, pen);
    }

    pub fn set_with_pen<'a>(&mut self, pix: P, pen: Pen) {
        self.tiles.set(pen.cursor_loc, pix, pen.fg, pen.bg);
    }

    // +x is right
    // +y is down
    pub fn set_rect(&mut self, x: u32, y: u32) {
        for d_x in 0..x {
            for d_y in 0..y {
                let offset = units::ScreenTile2D::new(d_x as i32, d_y as i32);
                self.with_offset(offset).set_pix(Default::default());
            }
        }
    }
}
