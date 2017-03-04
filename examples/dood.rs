#[macro_use]
extern crate glium;

#[macro_use]
extern crate lazy_static;

extern crate image;
extern crate gltile;

//display
//index buffer
//vertex buffer
//program

fn display(screen_size: gltile::Size) -> glium::backend::glutin_backend::GlutinFacade {
    use glium::DisplayBuild;
    glium::glutin::WindowBuilder::new()
        .with_dimensions(screen_size.width as u32, screen_size.height as u32)
        .build_glium()
        .unwrap()
}

fn program(display: &glium::backend::glutin_backend::GlutinFacade) -> glium::Program {
    let vertex = gltile::read_file("shaders/vertex.glsl")
        .expect("could not find shaders/vertex.glsl");
    let fragment = gltile::read_file("shaders/fragment.glsl")
        .expect("could not find shaders/fragment.glsl");
    glium::Program::from_source(display as &glium::backend::Facade, &vertex, &fragment, None)
        .unwrap()
}

fn indices(size: gltile::Size,
           display: &glium::backend::glutin_backend::GlutinFacade)
           -> glium::IndexBuffer<u16> {
    use glium::index::PrimitiveType;

    let indices = gltile::indices((size.width * size.height) as usize);
    glium::IndexBuffer::new(display as &glium::backend::Facade,
                            PrimitiveType::TrianglesList,
                            &indices)
        .unwrap()
}

fn main() {
    use glium::Surface;

    let tile_size = gltile::Size::new(16, 16);
    let screen_size = gltile::Size::new(1536 /* 96 */, 1024 /* 64 */);
    let display = display(screen_size);
    let program = program(&display);

    let texture = {
        let tileset = gltile::read_png_to_texture(&include_bytes!("../assets/tileset.png")[..]);
        glium::texture::Texture2d::new(&display, tileset).unwrap()
    };

    let tileset = texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);

    let size = gltile::Size::new((screen_size.width / tile_size.width),
                                 (screen_size.height / tile_size.height));

    let indices = indices(size, &display);

    let cam_uniforms = {
        let mat4_id = gltile::mat4_id();
        let ortho_projection = gltile::ortho_projection(screen_size);

        let cam = [[1.0, 0.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0, 0.0],
                   [0.0, 0.0, 1.0, 0.0],
                   [-768.0, -512.0, 0.0, 1.0]];

        uniform! {
            mvp: gltile::model_view_projection(mat4_id, cam, ortho_projection),
            tileset: tileset,
        }
    };

    let vb = {
        let mut vb = gltile::VertexBuffer::new(size);

        let tile = {
            let mut tile = gltile::Tile::new();
            tile.fg = *gltile::colors::YELLOW;
            tile.pix = gltile::Pix::Dood;
            tile
        };

        vb.set(gltile::WindowLoc { x: 5, y: 5 }, tile);

        // TODO hide this as an implementation detail
        glium::VertexBuffer::new(&display, &vb.data()).unwrap()
    };

    // let mut console = gltile::Console::new(size);

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vb, &indices, &program, &cam_uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
