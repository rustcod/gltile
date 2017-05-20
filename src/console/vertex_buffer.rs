use console;
use pixset;
use units;
use vertex;

pub struct VertexBuffer {
    vertex_data: vertex::VertexData,
}

impl VertexBuffer {
    pub fn new(size: units::Size2D, pixset: &pixset::Pixset) -> Self {
        VertexBuffer { vertex_data: vertex::VertexData::new(size, pixset) }
    }

    pub fn data(&self) -> &[vertex::Vertex] {
        &self.vertex_data.data()
    }

    pub fn set(
        &mut self,
        screen_loc: units::ScreenTile2D,
        tile: console::Tile,
        coords: pixset::TexCoords,
    ) {
        self.vertex_data.set(screen_loc, tile, coords);
    }
}
