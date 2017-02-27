use colors;
use pixset;

#[derive(Clone, Copy, Debug, Default)]
pub struct Tile {
    pub fg: colors::Srgb,
    pub bg: colors::Srgb,
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

    pub fn clear(&mut self) {
        // TODO gross
        let t = Tile::new();
        self.fg = t.fg;
        self.bg = t.bg;
        self.pix = t.pix;
    }
}
