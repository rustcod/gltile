use colors;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tileset_coords: [f32; 2],
    pub screen_loc: [i32; 2],
    pub foreground_color: colors::Rgb,
    pub background_color: colors::Rgb,
}

implement_vertex!(
    Vertex,
    position,
    tileset_coords,
    screen_loc,
    foreground_color,
    background_color
);

impl Vertex {
    pub fn new(position: [f32; 2], tileset_coords: [f32; 2], screen_loc: [i32; 2]) -> Self {
        Vertex {
            position: position,
            screen_loc: screen_loc,
            tileset_coords: tileset_coords,
            foreground_color: *colors::BLACK,
            background_color: *colors::BLACK,
        }
    }
}
