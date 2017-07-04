use console;
use pixset::PixLike;
use units;
use vertex;

pub struct VertexData {
    data: Vec<vertex::Vertex>,
    size: units::Size2D,
}

// TODO remove Size
impl VertexData {
    pub fn new<P: PixLike>(size: units::Size2D, empty: P) -> Self {
        let length = (size.width * size.height) as usize;
        let (top, right, bottom, left) = empty.get();
        let mut data: Vec<vertex::Vertex> = Vec::with_capacity(length * 4);

        for i in 0..length {
            let (x, y) = (i as i32 % size.width, size.height - 1 - i as i32 / size.width as i32);
            data.push(vertex::Vertex::new([-0.5, 0.5], [left, top], [x, y + 1]));
            data.push(vertex::Vertex::new(
                [0.5, 0.5],
                [right, top],
                [x + 1, y + 1],
            ));
            data.push(vertex::Vertex::new(
                [0.5, -0.5],
                [right, bottom],
                [x + 1, y],
            ));
            data.push(vertex::Vertex::new([-0.5, -0.5], [left, bottom], [x, y]));
        }

        VertexData {
            data: data,
            size: size,
        }
    }

    pub fn data(&self) -> &[vertex::Vertex] {
        &self.data[..]
    }

    pub fn set<P: PixLike>(
        &mut self,
        screen_loc: units::ScreenTile2D,
        tile: console::Tile<P>,
        coords: (f32, f32, f32, f32),
    ) {
        let offset = ((self.size.width * screen_loc.y + screen_loc.x) * 4) as usize;
        let (top, right, bottom, left) = coords;

        self.data[offset].tileset_coords = [left, top];
        self.data[offset].foreground_color = tile.fg;
        self.data[offset].background_color = tile.bg;

        self.data[offset + 1].tileset_coords = [right, top];
        self.data[offset + 1].foreground_color = tile.fg;
        self.data[offset + 1].background_color = tile.bg;

        self.data[offset + 2].tileset_coords = [right, bottom];
        self.data[offset + 2].foreground_color = tile.fg;
        self.data[offset + 2].background_color = tile.bg;

        self.data[offset + 3].tileset_coords = [left, bottom];
        self.data[offset + 3].foreground_color = tile.fg;
        self.data[offset + 3].background_color = tile.bg;
    }
}
