extern crate glium;
extern crate gltile;
extern crate looper;
extern crate pixset;
extern crate rand;

use pixset::{PixLike, TilesetLike};
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use std::ops::Deref;

pub struct RandomTile<P: PixLike>(gltile::Tile<P>);

impl<P: PixLike> Deref for RandomTile<P> {
    type Target = gltile::Tile<P>;

    fn deref(&self) -> &gltile::Tile<P> {
        &self.0
    }
}

impl rand::Rand for RandomTile<pixset::Pix> {
    fn rand<R: Rng>(rng: &mut R) -> RandomTile<pixset::Pix> {
        let tile = gltile::Tile::make(
            rng.gen::<[f32; 3]>(),
            rng.gen::<[f32; 3]>(),
            pixset::Pix::Dood,
        );
        RandomTile(tile)
    }
}

fn main() {
    let dim = 512i32;
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new().with_dimensions(dim as u32, dim as u32);
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut renderer = gltile::Renderer::new(&display, &pixset::TILESET);


    let mut rng = rand::thread_rng();
    let (width, height) = (&pixset::TILESET).get_tile_size();
    let x = Range::new(0, dim / width as i32);
    let y = Range::new(0, dim / height as i32);

    let render = |_| {
        let tile = *rng.gen::<RandomTile<pixset::Pix>>();
        let loc = (x.ind_sample(&mut rng), y.ind_sample(&mut rng));
        renderer.set(loc, tile);
        renderer.render();
        looper::Action::Continue
    };

    let update = |_| {
        let mut action = looper::Action::Continue;
        events_loop.poll_events(|event| match event {
            glium::glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glium::glutin::WindowEvent::Closed => action = looper::Action::Stop,
                    _ => (),
                }
            }
            _ => (),
        });

        action
    };

    looper::Looper::new(60.0).run(render, update);
}
