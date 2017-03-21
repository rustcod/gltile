use super::Tile;
use colors;
use pixset;
use units;

pub struct Tiles {
    size: units::Size2D,
    pub tiles: Vec<Tile>, // TODO impl Index
}

impl Tiles {
    pub fn new(size: units::Size2D) -> Self {
        let tiles = {
            // TODO area
            let length = (size.width * size.height) as usize;
            let mut ts = Vec::with_capacity(length);
            for _ in 0..length {
                let t = Tile::new();
                ts.push(t);
            }
            ts
        };

        Tiles {
            size: size,
            tiles: tiles,
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        for t in self.tiles.iter_mut() {
            t.clear();
        }
    }

    pub fn set(
        &mut self,
        loc: units::ScreenTile2D,
        pix: pixset::Pix,
        fg: colors::Rgb,
        bg: colors::Rgb
    ) {
        // TODO asserts
        let idx = (self.size.width * loc.y + loc.x) as usize;
        self.tiles[idx].pix = pix;
        self.tiles[idx].fg = fg;
        self.tiles[idx].bg = bg;
    }
}
