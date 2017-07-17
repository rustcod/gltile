extern crate glium;
extern crate gltile;
extern crate looper;
extern crate pixset;

fn main() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new().with_dimensions(512, 512);
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut renderer = gltile::Renderer::new(&display, &pixset::TILESET);

    let tile = gltile::Tile::make(
        *gltile::colors::YELLOW,
        *gltile::colors::BLACK,
        pixset::Pix::Dood,
    );

    renderer.set((5, 5), tile);

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
