use super::{Console, Tile};
use euclid;
use units;
use vertex;

pub struct VertexBuffer {
    vertex_data: vertex::VertexData,
}

impl VertexBuffer {
    pub fn new(size: euclid::Size2D<i32>) -> Self {
        VertexBuffer { vertex_data: vertex::VertexData::new(size) }
    }

    pub fn data(&self) -> &[vertex::Vertex] {
        &self.vertex_data.data()
    }

    pub fn set(&mut self, screen_point: units::ScreenPoint2D, tile: Tile) {
        self.vertex_data.set(screen_point, tile);
    }

    pub fn blit_console(&mut self, console: &Console, screen_point: units::ScreenPoint2D) {
        // TODO iter ?
        for y in 0..console.size.height {
            for x in 0..console.size.width {
                let pt = screen_point + units::ScreenPoint2D::new(x as i32, y as i32);
                self.vertex_data.set(pt, console.get_tile(x, y))
            }
        }
    }
}
