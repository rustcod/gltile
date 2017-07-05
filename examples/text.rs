extern crate glium;
extern crate gltile;
extern crate looper;
extern crate pixset;

use glium::DisplayBuild;

fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(512, 512)
        .build_glium()
        .unwrap();

    let mut renderer = gltile::Renderer::new(&display, pixset::TILESET, pixset::Pix::Empty);

    for (pix, offset) in pixset::PixStr::from("Yo, Dawg;").iter() {
        let tile = gltile::Tile::make(*gltile::colors::YELLOW, *gltile::colors::BLACK, pix);
        renderer.set((5 + offset.0, 10), tile);
    }

    let render = |_| {
        renderer.render();
        looper::Action::Continue
    };

    let update = |_| {
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return looper::Action::Stop,
                _ => (),
            }
        }
        looper::Action::Continue
    };

    looper::Looper::new(60.0).run(render, update);
}
