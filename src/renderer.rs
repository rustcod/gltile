use console;
use glium;
use glium::backend::glutin::Display;
use mvp;
use pixset::PixLike;
use shaders;
use units;
use utils;
use std::borrow::Borrow;

fn get_screen_size(display: &Display) -> units::Size2D {
    let factor = display.gl_window().borrow().hidpi_factor();
    let (width, height) = display.get_framebuffer_dimensions();
    units::Size2D::new(
        (width as f32 / factor) as i32,
        (height as f32 / factor) as i32,
    )
}

pub struct Renderer<'a> {
    pub screen_size: units::Size2D,
    pub size: units::Size2D,
    pub display: &'a Display,
    pub program: glium::Program,
    pub indices: glium::IndexBuffer<u16>,
    pub texture: glium::texture::Texture2d,
    pub vertex_buffer: console::VertexBuffer,
}

// allow sampling of an empty texture
impl<'a> Renderer<'a> {
    pub fn new<P: PixLike>(display: &'a Display, tileset: &[u8], empty: P) -> Self {
        let (tile_width, tile_height) = empty.tile_size();

        let screen_size = get_screen_size(&display);
        let size = units::Size2D::new(
            (screen_size.width / tile_width as i32),
            (screen_size.height / tile_height as i32),
        );

        let program = program(&display, empty.tile_size());
        let indices = indices(&display, size);
        let texture = texture(&display, tileset);
        let vertex_buffer = console::VertexBuffer::new(size, empty);

        Renderer {
            size: size,
            screen_size: screen_size,
            display: &display,
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

            let cam = [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]; // X, Y

            let tileset = self.texture.sampled();

            uniform! {
                mvp: mvp::model_view_projection(mat4_id, cam, ortho_projection),
                tileset: tileset,
            }
        };

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target
            .draw(
                &vb,
                &self.indices,
                &self.program,
                &cam_uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    }

    pub fn set<P: PixLike, L: Into<units::ScreenTile2D>>(
        &mut self,
        screen_loc: L,
        tile: console::Tile<P>,
    ) {
        let coords = tile.pix.get();
        self.vertex_buffer.set(screen_loc.into(), tile, coords);
    }

    pub fn blit_console<P: PixLike>(
        &mut self,
        screen_loc: units::ScreenTile2D,
        console: &console::Console<P>,
    ) {
        // TODO iter ?
        for y in 0..console.get_height() {
            for x in 0..console.get_width() {
                let loc = screen_loc + units::ScreenTile2D::new(x as i32, y as i32);
                let tile = console.get_tile(x, y);
                let coords = tile.pix.get();
                self.vertex_buffer.set(loc, tile, coords)
            }
        }
    }
}

fn program(display: &Display, (width, height): (u32, u32)) -> glium::Program {
    glium::Program::from_source(
        display as &glium::backend::Facade,
        &shaders::VERTEX
            .replace("{width}", &(width as f32).to_string())
            .replace("{height}", &(height as f32).to_string()),
        &shaders::FRAGMENT,
        None,
    ).unwrap()
}

fn indices(display: &Display, size: units::Size2D) -> glium::IndexBuffer<u16> {
    use glium::index::PrimitiveType;

    let indices = utils::indices((size.width * size.height) as usize);
    glium::IndexBuffer::new(
        display as &glium::backend::Facade,
        PrimitiveType::TrianglesList,
        &indices,
    ).unwrap()
}

fn texture(display: &Display, tileset: &[u8]) -> glium::texture::Texture2d {
    use glium::{backend, texture};

    let image = utils::read_png_to_image(tileset);
    let image_dimensions = image.dimensions();
    let texture = texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    texture::Texture2d::new(display as &backend::Facade, texture).unwrap()
}
