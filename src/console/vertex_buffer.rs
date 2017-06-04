use console;
use pixset::PixLike;
use units;
use vertex;

pub struct VertexBuffer {
    vertex_data: vertex::VertexData,
}

impl VertexBuffer {
    pub fn new<P: PixLike>(size: units::Size2D, empty: P) -> Self {
        VertexBuffer { vertex_data: vertex::VertexData::new(size, empty) }
    }

    pub fn data(&self) -> &[vertex::Vertex] {
        &self.vertex_data.data()
    }

    pub fn set<P: PixLike>(
        &mut self,
        screen_loc: units::ScreenTile2D,
        tile: console::Tile<P>,
        coords: (f32, f32, f32, f32),
    ) {
        self.vertex_data.set(screen_loc, tile, coords);
    }
}
