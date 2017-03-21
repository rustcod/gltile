use super::{Console, Tile};
use units;
use vertex;

pub struct VertexBuffer {
    vertex_data: vertex::VertexData,
}

impl VertexBuffer {
    pub fn new(size: units::Size2D) -> Self {
        VertexBuffer { vertex_data: vertex::VertexData::new(size) }
    }

    pub fn data(&self) -> &[vertex::Vertex] {
        &self.vertex_data.data()
    }

    pub fn set(&mut self, screen_loc: units::ScreenTile2D, tile: Tile) {
        self.vertex_data.set(screen_loc, tile);
    }

    pub fn blit_console(&mut self, console: &Console, screen_loc: units::ScreenTile2D) {
        // TODO iter ?
        for y in 0..console.size.height {
            for x in 0..console.size.width {
                let loc = screen_loc + units::ScreenTile2D::new(x as i32, y as i32);
                self.vertex_data.set(loc, console.get_tile(x, y))
            }
        }
    }
}
