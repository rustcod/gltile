use console;
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

    pub fn set(
        &mut self,
        screen_loc: units::ScreenTile2D,
        tile: console::Tile,
        coords: (f32, f32, f32, f32),
    ) {
        self.vertex_data.set(screen_loc, tile, coords);
    }
}
