#[macro_use]
extern crate glium;

#[macro_use]
extern crate lazy_static;

extern crate image;
extern crate gltile;

fn main() {
    let mut renderer = gltile::Renderer::new(1536, /* 96 */
                                             1024, /* 64 */
                                             16,
                                             "assets/tileset.png");

    let tile = {
        let mut tile = gltile::Tile::new();
        tile.fg = *gltile::colors::YELLOW;
        tile.pix = gltile::Pix::Dood;
        tile
    };

    renderer.set(gltile::WindowLoc { x: 5, y: 5 }, tile);

    loop {
        renderer.render();

        for ev in renderer.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
