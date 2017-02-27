use std;
use std::collections::HashMap;

pub type TexCoords = [[f32; 2]; 4];

lazy_static! {
    pub static ref PIXSET: Pixset  = Pixset::new(100);
}

lazy_static! {
    static ref PIX_ORDER: Vec<Pix> =
        vec![
        Pix::A,
        Pix::B,
        Pix::C,
        Pix::D,
        Pix::E,
        Pix::F,
        Pix::G,
        Pix::H,
        Pix::I,
        Pix::J,
        Pix::K,
        Pix::L,
        Pix::M,
        Pix::N,
        Pix::O,
        Pix::P,
        Pix::Q,
        Pix::R,
        Pix::S,
        Pix::T,
        Pix::U,
        Pix::V,
        Pix::W,
        Pix::X,
        Pix::Y,
        Pix::Z,
        Pix::DownArrow,
        Pix::LeftArrow,
        Pix::Dood,
        Pix::Food,
        Pix::UpArrow,
        Pix::RightArrow,
        Pix::Hash,
        Pix::Period,
        Pix::Comma,
        Pix::Quotes,
        Pix::Apostrophe,
        Pix::Colon,
        Pix::SemiColon,
        Pix::Empty,
        Pix::LeftTopCorner,
        Pix::RightTopCorner,
        Pix::LeftBottomCorner,
        Pix::RightBottomCorner,
        Pix::LeftStraight,
        Pix::RightStraight,
        Pix::TopStraight,
        Pix::BottomStraight
    ];
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Pix {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Dood,
    Food,
    UpArrow,
    DownArrow,
    RightArrow,
    LeftArrow,
    Hash,
    Period,
    Comma,
    Quotes,
    Apostrophe,
    Colon,
    SemiColon,
    Empty,
    LeftTopCorner,
    RightTopCorner,
    LeftBottomCorner,
    RightBottomCorner,
    LeftStraight,
    RightStraight,
    TopStraight,
    BottomStraight,
}

impl std::default::Default for Pix {
    fn default() -> Pix {
        Pix::Empty
    }
}

pub struct Pixset {
    pub tiles: HashMap<Pix, TexCoords>,
    pub total_tiles: i32,
}

impl Pixset {
    pub fn new(total_tiles: i32) -> Pixset {
        let tile_dim = (total_tiles as f32).sqrt() as i32;

        let mut tiles: HashMap<Pix, TexCoords> = HashMap::new();
        for (i, tile) in PIX_ORDER.iter().enumerate() {
            let tex_coords = vec![i as i32 % tile_dim, i as i32 / tile_dim];
            tiles.insert(*tile, get_tex_coords(total_tiles, tex_coords));
        }

        Pixset {
            tiles: tiles,
            total_tiles: total_tiles,
        }
    }

    pub fn get(&self, pix: &Pix) -> TexCoords {
        *self.tiles.get(pix).expect("tile did not contain that pix")
    }
}

fn get_tex_coords(total_tiles: i32, loc: Vec<i32>) -> TexCoords {
    let tile_dim: f32 = (total_tiles as f32).sqrt();
    let per_tile: f32 = 1.0 / tile_dim;
    let fudge_factor = 0.0005; // push sampling one pixel to the right

    let top = (1.0 - loc[1] as f32 * per_tile) + fudge_factor;
    let right = ((loc[0] + 1) as f32 * per_tile) + fudge_factor;
    let bottom = (1.0 - (loc[1] + 1) as f32 * per_tile) + fudge_factor;
    let left = (loc[0] as f32 * per_tile) + fudge_factor;

    [[left, top], [right, top], [right, bottom], [left, bottom]]
}
