use glium;
use glium::uniforms::{Sampler, UniformsStorage, EmptyUniforms};

use Size;
use mvp;
use read_file;
use utils;

pub struct Renderer {
    pub screen_size: Size,
    pub size: Size,
    pub display: glium::backend::glutin_backend::GlutinFacade,
    pub program: glium::Program,
    pub indices: glium::IndexBuffer<u16>,
}

impl Renderer {
    pub fn new(width: i32, height: i32, tile_size: i32) -> Self {
        let screen_size = Size::new(width, height);
        let size = Size::new((screen_size.width / tile_size),
                             (screen_size.height / tile_size));

        let display = display(screen_size);
        let program = program(&display);
        let indices = indices(size, &display);

        Renderer {
            size: size,
            screen_size: screen_size,
            display: display,
            program: program,
            indices: indices,
        }
    }
}

fn display(screen_size: Size) -> glium::backend::glutin_backend::GlutinFacade {
    use glium::DisplayBuild;
    glium::glutin::WindowBuilder::new()
        .with_dimensions(screen_size.width as u32, screen_size.height as u32)
        .build_glium()
        .unwrap()
}

fn program(display: &glium::backend::glutin_backend::GlutinFacade) -> glium::Program {
    let vertex = read_file("shaders/vertex.glsl").expect("could not find shaders/vertex.glsl");
    let fragment = read_file("shaders/fragment.glsl")
        .expect("could not find shaders/fragment.glsl");
    glium::Program::from_source(display as &glium::backend::Facade, &vertex, &fragment, None)
        .unwrap()
}

fn indices(size: Size,
           display: &glium::backend::glutin_backend::GlutinFacade)
           -> glium::IndexBuffer<u16> {
    use glium::index::PrimitiveType;

    let indices = utils::indices((size.width * size.height) as usize);
    glium::IndexBuffer::new(display as &glium::backend::Facade,
                            PrimitiveType::TrianglesList,
                            &indices)
        .unwrap()
}
