/// a 2D point located on the screen
/// independent of game location
///
/// e.g. `ScreenPoint2D::new(0, 0)` is always the upper left tile of the screen
/// regardless of what that game location that happens to be

use euclid;

pub struct ScreenSpace;
pub type ScreenPoint2D = euclid::TypedPoint2D<i32, ScreenSpace>;
