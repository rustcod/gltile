#[macro_use]
extern crate pixset_derive;

#[macro_use]
extern crate pixset;
extern crate glium;
extern crate gltile;
extern crate looper;

use pixset::PixLike;

pix! {
    tileset => "assets/custom-tileset.png";
    width => "16";
    height => "16";
    total => "4";
    One,
    Two,
    Three,
    Empty;
    'a' => One,
    'b' => Two,
    'c' => Three,
    'd' => Empty;
}

fn render_tile(renderer: &mut gltile::Renderer, loc: (i32, i32), pix: Pix) {
    let tile = gltile::Tile::make(*gltile::colors::YELLOW, *gltile::colors::BLACK, pix);
    renderer.set(loc, tile);
}

fn main() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new().with_dimensions(512, 512);
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut renderer = gltile::Renderer::new(&display, TILESET, Pix::Empty);

    render_tile(&mut renderer, (5, 5), Pix::One);
    render_tile(&mut renderer, (6, 6), Pix::Two);
    render_tile(&mut renderer, (7, 7), Pix::Three);
    render_tile(&mut renderer, (8, 8), Pix::Empty);

    for (pix, offset) in PixStr::from("abcdcba").iter() {
        let tile = gltile::Tile::make(*gltile::colors::YELLOW, *gltile::colors::BLACK, pix);
        renderer.set((5 + offset.0, 10), tile);
    }

    let render = |_| {
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
