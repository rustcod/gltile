use console;
use data;
use pixset;
use vertex;

pub struct VertexData {
    data: Vec<vertex::Vertex>,
    size: data::Size,
}

// TODO remove Size
impl VertexData {
    pub fn new(size: data::Size) -> Self {
        let length = (size.width * size.height) as usize;
        let tileset_coords = pixset::PIXSET.get(&pixset::Pix::Empty);
        let mut data: Vec<vertex::Vertex> = Vec::with_capacity(length * 4);

        for i in 0..length {
            let window_loc = [i as i32 % size.width, i as i32 / size.width as i32];
            data.push(vertex::Vertex::new([-0.5, 0.5],
                                          tileset_coords[0],
                                          [window_loc[0], window_loc[1] + 1]));
            data.push(vertex::Vertex::new([0.5, 0.5],
                                          tileset_coords[1],
                                          [window_loc[0] + 1, window_loc[1] + 1]));
            data.push(vertex::Vertex::new([0.5, -0.5],
                                          tileset_coords[2],
                                          [window_loc[0] + 1, window_loc[1]]));
            data.push(vertex::Vertex::new([-0.5, -0.5],
                                          tileset_coords[3],
                                          [window_loc[0], window_loc[1]]));
        }

        VertexData {
            data: data,
            size: size,
        }
    }

    pub fn data(&self) -> &[vertex::Vertex] {
        &self.data[..]
    }

    // TODO have this take references? bench it?
    pub fn set(&mut self, window_loc: data::WindowLoc, tile: console::Tile) {
        let offset: usize = ((self.size.height - 1 - window_loc.y) * self.size.width +
                             window_loc.x) as usize * 4;
        let tileset_coords = pixset::PIXSET.get(&tile.pix);

        self.data[offset].tileset_coords = tileset_coords[0];
        self.data[offset].foreground_color = tile.fg;
        self.data[offset].background_color = tile.bg;

        self.data[offset + 1].tileset_coords = tileset_coords[1];
        self.data[offset + 1].foreground_color = tile.fg;
        self.data[offset + 1].background_color = tile.bg;

        self.data[offset + 2].tileset_coords = tileset_coords[2];
        self.data[offset + 2].foreground_color = tile.fg;
        self.data[offset + 2].background_color = tile.bg;

        self.data[offset + 3].tileset_coords = tileset_coords[3];
        self.data[offset + 3].foreground_color = tile.fg;
        self.data[offset + 3].background_color = tile.bg;
    }
}
