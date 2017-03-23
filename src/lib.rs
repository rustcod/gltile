#[macro_use]
extern crate glium;

#[macro_use]
extern crate lazy_static;

extern crate euclid;
extern crate image;
extern crate palette;

pub mod colors; // TODO
mod console;
pub mod font; // TODO
mod mvp;
mod pixset;
mod renderer;
mod shaders;
pub mod units; // TODO
mod utils;
mod vertex;

pub use console::Console;
pub use console::Tile;
pub use console::VertexBuffer;
pub use mvp::Matrix4;
pub use mvp::model_view_projection;
pub use pixset::{Pix, Pixset};
pub use renderer::Renderer;
pub use utils::indices;
pub use utils::mat4_id;
pub use utils::ortho_projection;
pub use utils::read_bytes;
pub use utils::read_file;
pub use utils::read_png_to_texture;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
