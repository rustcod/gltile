use console;
use glium;
use mvp;
use shaders;
use units;
use utils;

fn get_screen_size(display: &glium::backend::glutin_backend::GlutinFacade) -> units::Size2D {
    let factor = display.get_window().unwrap().hidpi_factor();
    let (width, height) = display.get_framebuffer_dimensions();
    units::Size2D::new((width as f32 / factor) as i32,
                       (height as f32 / factor) as i32)
}

pub struct Renderer<'a> {
    pub screen_size: units::Size2D,
    pub size: units::Size2D,
    pub display: &'a glium::backend::glutin_backend::GlutinFacade,
    pub program: glium::Program,
    pub indices: glium::IndexBuffer<u16>,
    pub texture: glium::texture::Texture2d,
    pub vertex_buffer: console::VertexBuffer,
}

impl<'a> Renderer<'a> {
    pub fn new(
        display: &'a glium::backend::glutin_backend::GlutinFacade,
        tile_size: i32,
        texture_path: &str
    ) -> Self {
        let screen_size = get_screen_size(display);
        let size = units::Size2D::new((screen_size.width / tile_size),
                                      (screen_size.height / tile_size));

        let program = program(display);
        let indices = indices(display, size);
        let texture = texture(display, texture_path);
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

        let vb = glium::VertexBuffer::new(self.display, &self.vertex_buffer.data()).unwrap();

        let cam_uniforms = {
            let mat4_id = utils::mat4_id();
            let ortho_projection = utils::ortho_projection(self.screen_size);

            // 1, 0, 0, X
            // 0, 1, 0, Y
            // 0, 0, 1, 0
            // 0, 0, 0, 1

            let cam = [[1.0, 0.0, 0.0, 0.0],
                       [0.0, 1.0, 0.0, 0.0],
                       [0.0, 0.0, 1.0, 0.0],
                       [0.0, 0.0, 0.0, 1.0]]; // X, Y

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

    pub fn set(&mut self, screen_loc: units::ScreenTile2D, tile: console::Tile) {
        self.vertex_buffer.set(screen_loc, tile);
    }
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
