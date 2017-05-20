use console;
use pixset;
use units;
use vertex;

pub struct VertexData {
    data: Vec<vertex::Vertex>,
    size: units::Size2D,
}

// TODO remove Size
impl VertexData {
    pub fn new(size: units::Size2D, pixset: &pixset::Pixset) -> Self {
        let length = (size.width * size.height) as usize;
        let (lt, rt, rb, lb) = pixset.get(&pixset::Pix::Empty);
        let mut data: Vec<vertex::Vertex> = Vec::with_capacity(length * 4);
        let inv_y = size.height % size.width - 1;

        for i in 0..length {
            let (x, y) = (i as i32 % size.width, inv_y - i as i32 / size.width as i32);
            data.push(vertex::Vertex::new([-0.5, 0.5], lt, [x, y + 1]));
            data.push(vertex::Vertex::new([0.5, 0.5], rt, [x + 1, y + 1]));
            data.push(vertex::Vertex::new([0.5, -0.5], rb, [x + 1, y]));
            data.push(vertex::Vertex::new([-0.5, -0.5], lb, [x, y]));
        }

        VertexData {
            data: data,
            size: size,
        }
    }

    pub fn data(&self) -> &[vertex::Vertex] {
        &self.data[..]
    }

    pub fn set(
        &mut self,
        screen_loc: units::ScreenTile2D,
        tile: console::Tile,
        coords: ([f32; 2], [f32; 2], [f32; 2], [f32; 2]),
    ) {
        let offset = ((self.size.width * screen_loc.y + screen_loc.x) * 4) as usize;
        let (lt, rt, rb, lb) = coords;

        self.data[offset].tileset_coords = lt;
        self.data[offset].foreground_color = tile.fg;
        self.data[offset].background_color = tile.bg;

        self.data[offset + 1].tileset_coords = rt;
        self.data[offset + 1].foreground_color = tile.fg;
        self.data[offset + 1].background_color = tile.bg;

        self.data[offset + 2].tileset_coords = rb;
        self.data[offset + 2].foreground_color = tile.fg;
        self.data[offset + 2].background_color = tile.bg;

        self.data[offset + 3].tileset_coords = lb;
        self.data[offset + 3].foreground_color = tile.fg;
        self.data[offset + 3].background_color = tile.bg;
    }
}
