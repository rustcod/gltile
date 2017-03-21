extern crate glium;
extern crate gltile;
extern crate looper;

use glium::DisplayBuild;

fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1536, 1024)
        .build_glium()
        .unwrap();

    let mut renderer = gltile::Renderer::new(&display, 16, "assets/tileset.png");

    let tile = {
        let mut tile = gltile::Tile::new();
        tile.fg = *gltile::colors::YELLOW;
        tile.pix = gltile::Pix::Dood;
        tile
    };

    renderer.set(gltile::ScreenTile2D::new(5, 5), tile);

    looper::Looper::new(60.0).run(|| {
        renderer.render();
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return looper::Action::Stop,
                _ => (),
            }
        }
        looper::Action::Continue
    })
}
