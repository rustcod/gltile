#[macro_use]
extern crate glium;

#[macro_use]
extern crate lazy_static;

extern crate image;
extern crate palette;
extern crate pixset;

pub mod colors; // TODO
mod console;
mod mvp;
mod renderer;
mod shaders;
mod units;
mod utils;
mod vertex;

pub use console::Console;
pub use console::Tile;
pub use renderer::Renderer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
