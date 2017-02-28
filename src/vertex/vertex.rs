use colors;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tileset_coords: [f32; 2],
    pub window_loc: [i32; 2],
    pub foreground_color: colors::Srgb,
    pub background_color: colors::Srgb,
}

implement_vertex!(Vertex,
                  position,
                  tileset_coords,
                  window_loc,
                  foreground_color,
                  background_color);

impl Vertex {
    pub fn new(position: [f32; 2], tileset_coords: [f32; 2], window_loc: [i32; 2]) -> Self {
        Vertex {
            position: position,
            window_loc: window_loc,
            tileset_coords: tileset_coords,
            foreground_color: *colors::BLACK,
            background_color: *colors::BLACK,
        }
    }
}
