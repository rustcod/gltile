use glium;

use data;
use Size;
use console;
use mvp;
use read_file;
use utils;

pub struct Renderer {
    pub screen_size: Size,
    pub size: Size,
    pub display: glium::backend::glutin_backend::GlutinFacade,
    pub program: glium::Program,
    pub indices: glium::IndexBuffer<u16>,
    pub texture: glium::texture::Texture2d,
    pub vertex_buffer: console::VertexBuffer,
}

impl Renderer {
    pub fn new(width: i32, height: i32, tile_size: i32, texture_path: &str) -> Self {
        let screen_size = Size::new(width, height);
        let size = Size::new((screen_size.width / tile_size),
                             (screen_size.height / tile_size));

        let display = display(screen_size);
        let program = program(&display);
        let indices = indices(&display, size);
        let texture = texture(&display, texture_path);
        let vertex_buffer = console::VertexBuffer::new(size);

        Renderer {
            size: size,
            screen_size: screen_size,
            display: display,
            program: program,
            indices: indices,
            texture: texture,
            vertex_buffer: vertex_buffer,
        }
    }

    pub fn render(&self) {
        use glium::Surface;

        let vb = glium::VertexBuffer::new(&self.display, &self.vertex_buffer.data()).unwrap();

        let cam_uniforms = {
            let mat4_id = utils::mat4_id();
            let ortho_projection = utils::ortho_projection(self.screen_size);

            let cam = [[1.0, 0.0, 0.0, 0.0],
                       [0.0, 1.0, 0.0, 0.0],
                       [0.0, 0.0, 1.0, 0.0],
                       [-768.0, -512.0, 0.0, 1.0]];

            let tileset = self.texture
                .sampled()
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);

            uniform! {
                mvp: mvp::model_view_projection(mat4_id, cam, ortho_projection),
                tileset: tileset,
            }
        };

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vb,
                  &self.indices,
                  &self.program,
                  &cam_uniforms,
                  &Default::default())
            .unwrap();
        target.finish().unwrap();
    }

    pub fn set(&mut self, window_loc: data::WindowLoc, tile: console::Tile) {
        self.vertex_buffer.set(window_loc, tile);
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
    let frag = read_file("shaders/fragment.glsl").expect("could not find shaders/fragment.glsl");
    glium::Program::from_source(display as &glium::backend::Facade, &vertex, &frag, None).unwrap()
}

fn indices(display: &glium::backend::glutin_backend::GlutinFacade,
           size: Size)
           -> glium::IndexBuffer<u16> {
    use glium::index::PrimitiveType;

    let indices = utils::indices((size.width * size.height) as usize);
    glium::IndexBuffer::new(display as &glium::backend::Facade,
                            PrimitiveType::TrianglesList,
                            &indices)
        .unwrap()
}

fn texture(display: &glium::backend::glutin_backend::GlutinFacade,
           path: &str)
           -> glium::texture::Texture2d {
    let png = utils::read_bytes(path).unwrap();
    let texture = utils::read_png_to_texture(&png[..]);
    glium::texture::Texture2d::new(display as &glium::backend::Facade, texture).unwrap()
}
