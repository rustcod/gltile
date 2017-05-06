use colors;
use pixset;

#[derive(Clone, Copy, Debug, Default)]
pub struct Tile {
    pub fg: colors::Rgb,
    pub bg: colors::Rgb,
    pub pix: pixset::Pix,
}

// TODO default?
impl Tile {
    pub fn new() -> Self {
        Tile {
            fg: *colors::BLACK,
            bg: *colors::BLACK,
            pix: pixset::Pix::Empty,
        }
    }

    pub fn make(fg: colors::Rgb, bg: colors::Rgb, pix: pixset::Pix) -> Self {
        Tile {
            fg: fg,
            bg: bg,
            pix: pix,
        }
    }

    pub fn clear(&mut self) {
        // TODO gross
        let t = Tile::new();
        self.fg = t.fg;
        self.bg = t.bg;
        self.pix = t.pix;
    }
}
