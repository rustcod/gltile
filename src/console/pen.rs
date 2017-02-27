use colors;
use data;

#[derive(Clone, Copy, Debug, Default)]
pub struct Pen {
    pub cursor_loc: data::WindowLoc,
    pub fg: colors::Srgb,
    pub bg: colors::Srgb,
}

impl Pen {
    #[allow(dead_code)]
    pub fn offset_loc(self, offset: data::WindowLocOffset) -> Self {
        Pen { cursor_loc: self.cursor_loc.offset(offset), ..self }
    }
}
