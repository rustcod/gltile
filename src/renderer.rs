use console;
use glium;
use mvp;
use shaders;
use units;
use utils;

pub struct Renderer {
    pub screen_size: units::Size2D,
    pub size: units::Size2D,
    pub display: glium::backend::glutin_backend::GlutinFacade,
    pub program: glium::Program,
    pub indices: glium::IndexBuffer<u16>,
    pub texture: glium::texture::Texture2d,
    pub vertex_buffer: console::VertexBuffer,
}

impl Renderer {
    pub fn new(width: i32, height: i32, tile_size: i32, texture_path: &str) -> Self {
        let screen_size = units::Size2D::new(width, height);
        let size = units::Size2D::new((screen_size.width / tile_size),
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

    pub fn set(&mut self, screen_point: units::ScreenPoint2D, tile: console::Tile) {
        self.vertex_buffer.set(screen_point, tile);
    }
}

fn display(screen_size: units::Size2D) -> glium::backend::glutin_backend::GlutinFacade {
    use glium::DisplayBuild;
    glium::glutin::WindowBuilder::new()
        .with_dimensions(screen_size.width as u32, screen_size.height as u32)
        .build_glium()
        .unwrap()
}

fn program(display: &glium::backend::glutin_backend::GlutinFacade) -> glium::Program {
    glium::Program::from_source(display as &glium::backend::Facade,
                                &shaders::VERTEX,
                                &shaders::FRAGMENT,
                                None)
        .unwrap()
}

fn indices(
    display: &glium::backend::glutin_backend::GlutinFacade,
    size: units::Size2D
) -> glium::IndexBuffer<u16> {
    use glium::index::PrimitiveType;

    let indices = utils::indices((size.width * size.height) as usize);
    glium::IndexBuffer::new(display as &glium::backend::Facade,
                            PrimitiveType::TrianglesList,
                            &indices)
        .unwrap()
}

fn texture(
    display: &glium::backend::glutin_backend::GlutinFacade,
    path: &str
) -> glium::texture::Texture2d {
    let png = utils::read_bytes(path).expect(&format!("Texture not found: {}", path)[..]);
    let texture = utils::read_png_to_texture(&png[..]);
    glium::texture::Texture2d::new(display as &glium::backend::Facade, texture).unwrap()
}
