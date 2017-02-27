use colors;
use data;
use pixset;

use super::Tile;

pub struct Tiles {
    size: data::Size,
    pub tiles: Vec<Tile>, // TODO impl Index
}

impl Tiles {
    pub fn new(size: data::Size) -> Self {
        let tiles = {
            let mut ts = Vec::with_capacity(size.len());
            for _ in 0..size.len() {
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

    pub fn clear(&mut self) {
        for t in self.tiles.iter_mut() {
            t.clear();
        }
    }

    // TODO WindowLoc really isn't anymore since consoles
    // can be located offscreen and `blitted` anywhere
    pub fn set(&mut self,
               cursor_loc: data::WindowLoc,
               pix: pixset::Pix,
               fg: colors::Srgb,
               bg: colors::Srgb) {
        // TODO asserts
        let idx = (self.size.width * cursor_loc.y + cursor_loc.x) as usize;
        self.tiles[idx].pix = pix;
        self.tiles[idx].fg = fg;
        self.tiles[idx].bg = bg;
    }
}
