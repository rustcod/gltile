#[macro_use]
extern crate glium;

#[macro_use]
extern crate lazy_static;

extern crate image;
extern crate gltile;

fn main() {
    use glium::{DisplayBuild, Surface};

    let screen_size = gltile::Size {
        width: 1536, // 96
        height: 1024, // 64
    };

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(screen_size.width as u32, screen_size.height as u32)
        .build_glium()
        .unwrap();

    let vertex = gltile::read_file("shaders/vertex.glsl")
        .expect("could not find shaders/vertex.glsl");
    let fragment = gltile::read_file("shaders/fragment.glsl")
        .expect("could not find shaders/fragment.glsl");
    let program = glium::Program::from_source(&display, &vertex, &fragment, None).unwrap();

    let texture = {
        let tileset = gltile::read_png_to_texture(&include_bytes!("../assets/tileset.png")[..]);
        glium::texture::Texture2d::new(&display, tileset).unwrap()
    };

    let tileset = texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);

    // TODO hardcoded
    let size = gltile::Size::new((screen_size.width as f32 / 16.0) as i32,
                                 (screen_size.height as f32 / 16.0) as i32);

    let mat4_id = gltile::mat4_id();
    let ortho_projection = gltile::ortho_projection(screen_size);

    let mut camera = {
        let halve_pixels = (display.get_window().unwrap().hidpi_factor() - 2.0).abs() < 0.1;
        gltile::Camera::new(screen_size, gltile::Loc { x: 0, y: 63 }, 16, halve_pixels)
    };

    let indices = {
        use glium::index::PrimitiveType;

        let indices = gltile::indices((size.width * size.height) as usize);
        glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &indices).unwrap()
    };

    let cam_uniforms = uniform! {
        mvp: gltile::model_view_projection(mat4_id, camera.as_mat(), ortho_projection),
        tileset: tileset
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
