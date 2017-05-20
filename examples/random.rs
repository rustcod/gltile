extern crate glium;
extern crate gltile;
extern crate looper;
extern crate pixset;
extern crate rand;

use glium::DisplayBuild;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

pub struct RandomTile(gltile::Tile);

impl RandomTile {
    pub fn to_tile(self) -> gltile::Tile {
        self.0
    }
}

impl rand::Rand for RandomTile {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let mut tile = gltile::Tile::new();
        tile.fg = rng.gen::<[f32; 3]>();
        tile.bg = rng.gen::<[f32; 3]>();
        tile.pix = pixset::Pix::Dood;
        RandomTile(tile)
    }
}

fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1536, 1024)
        .build_glium()
        .unwrap();

    let mut renderer = gltile::Renderer::new(&display, 16, "assets/tileset.png");

    let mut rng = rand::thread_rng();
    let x = Range::new(0, 96);
    let y = Range::new(0, 64);

    looper::Looper::new(60.0).run(
        |_| {
            let tile = rng.gen::<RandomTile>().to_tile();
            let loc =
                gltile::units::ScreenTile2D::new(x.ind_sample(&mut rng), y.ind_sample(&mut rng));
            renderer.set(loc, tile);
            renderer.render();
            looper::Action::Continue
        },
        |_| {
            for ev in display.poll_events() {
                match ev {
                    glium::glutin::Event::Closed => return looper::Action::Stop,
                    _ => (),
                }
            }
            looper::Action::Continue
        },
    );
}
