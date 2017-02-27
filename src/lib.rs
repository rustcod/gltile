#[macro_use]
extern crate glium;

#[macro_use]
extern crate lazy_static;

extern crate image;

mod camera;
mod color;
pub mod colors; // TODO
mod console;
mod data;
mod dir;
mod mvp;
mod pixset;
mod rgb;
mod utils;
mod vertex;

pub use camera::Camera;
pub use console::Console;
pub use console::Tile;
pub use console::VertexBuffer;
pub use data::Loc;
pub use data::Size;
pub use dir::Dir;
pub use pixset::{Pix, Pixset};
pub use data::WindowLoc;

pub use mvp::model_view_projection;

pub use utils::read_file;
pub use utils::read_png_to_texture;
pub use utils::indices;
pub use utils::mat4_id;
pub use utils::ortho_projection;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
