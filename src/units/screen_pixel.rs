/// a 2D pixel located on the screen
///
/// e.g. `ScreenPixel2D::new(0, 0)` is always the upper left pixel of the screen

use euclid;

pub struct ScreenPixelSpace;
pub type ScreenPixel2D = euclid::TypedPoint2D<i32, ScreenPixelSpace>;
