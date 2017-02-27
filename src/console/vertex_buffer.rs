use data;

use super::{Console, Tile};
use vertex::{Vertex, VertexData};

pub struct VertexBuffer {
    vertex_data: VertexData,
}

impl VertexBuffer {
    pub fn new(size: data::Size) -> Self {
        VertexBuffer { vertex_data: VertexData::new(size) }
    }

    pub fn data(&self) -> &[Vertex] {
        &self.vertex_data.data()
    }

    pub fn set(&mut self, window_loc: data::WindowLoc, tile: Tile) {
        self.vertex_data.set(window_loc, tile);
    }

    pub fn blit_console(&mut self, console: &Console, window_loc: data::WindowLoc) {
        // TODO iter ?
        for y in 0..console.size.height {
            for x in 0..console.size.width {
                let wl = window_loc.offset(data::WindowLocOffset::new(x as i32, y as i32));
                self.vertex_data.set(wl, console.get_tile(x, y))
            }
        }
    }
}
