/// a 2D tile located on the screen
/// independent of game location
///
/// e.g. `ScreenTile2D::new(0, 0)` is always the upper left tile of the screen
/// regardless of what that game location that happens to be

use euclid;

pub struct ScreenTileSpace;
pub type ScreenTile2D = euclid::TypedPoint2D<i32, ScreenTileSpace>;
