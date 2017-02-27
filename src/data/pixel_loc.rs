/// pixel based window location
///
/// e.g. `PixelLoc { x: 0, y: 0 }` is always the upper left pixel of the screen
#[derive(Clone, Copy, Debug, Default)]
pub struct PixelLoc {
    pub x: i32,
    pub y: i32,
}
