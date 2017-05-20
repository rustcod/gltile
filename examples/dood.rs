extern crate glium;
extern crate gltile;
extern crate looper;
extern crate pixset;

use glium::DisplayBuild;

fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1536, 1024)
        .build_glium()
        .unwrap();

    let pixset = pixset::Pixset::new(100, 16);
    let mut renderer = gltile::Renderer::new(&display, pixset);

    let tile = {
        let mut tile = gltile::Tile::new();
        tile.fg = *gltile::colors::YELLOW;
        tile.pix = pixset::Pix::Dood;
        tile
    };

    renderer.set(gltile::units::ScreenTile2D::new(5, 5), tile);

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
