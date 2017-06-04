use colors;
use pixset;
use pixset::PixLike;
use std::default::Default;

#[derive(Clone, Copy, Debug, Default)]
pub struct Tile<P: PixLike> {
    pub fg: colors::Rgb,
    pub bg: colors::Rgb,
    pub pix: P,
}

// TODO default?
impl<P> Tile<P>
where
    P: pixset::PixLike,
{
    pub fn new() -> Self {
        Tile {
            fg: *colors::BLACK,
            bg: *colors::BLACK,
            pix: Default::default(),
        }
    }

    pub fn make(fg: colors::Rgb, bg: colors::Rgb, pix: P) -> Self {
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
