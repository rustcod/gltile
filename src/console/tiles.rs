use super::Tile;
use colors;
use euclid;
use pixset;
use units;

pub struct Tiles {
    size: euclid::Size2D<i32>,
    pub tiles: Vec<Tile>, // TODO impl Index
}

impl Tiles {
    pub fn new(size: euclid::Size2D<i32>) -> Self {
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
        pt: units::ScreenPoint2D,
        pix: pixset::Pix,
        fg: colors::Srgb,
        bg: colors::Srgb
    ) {
        // TODO asserts
        let idx = (self.size.width * pt.y + pt.x) as usize;
        self.tiles[idx].pix = pix;
        self.tiles[idx].fg = fg;
        self.tiles[idx].bg = bg;
    }
}
