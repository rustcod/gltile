extern crate glium;
extern crate gltile;
extern crate looper;

fn main() {
    let mut renderer = gltile::Renderer::new(1536, 1024, 16, "assets/tileset.png");

    let tile = {
        let mut tile = gltile::Tile::new();
        tile.fg = *gltile::colors::YELLOW;
        tile.pix = gltile::Pix::Dood;
        tile
    };

    renderer.set(gltile::ScreenPoint2D::new(5, 5), tile);

    looper::Looper::new(60.0).run(|| {
        renderer.render();

        for ev in renderer.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return looper::Action::Stop,
                _ => (),
            }
        }

        looper::Action::Continue
    })
}
