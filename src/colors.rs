use color;
use rgb;

lazy_static! {
    pub static ref BLACK:  rgb::RGB = color::Color::new([  0,   0,   0]).to_srgb();
    pub static ref BROWN:  rgb::RGB = color::Color::new([122,  82,  48]).to_srgb();
    pub static ref BLUE:   rgb::RGB = color::Color::new([  0,   0, 255]).to_srgb();
    pub static ref RED:    rgb::RGB = color::Color::new([255,   0,   0]).to_srgb();
    pub static ref GREEN:  rgb::RGB = color::Color::new([  0, 255,   0]).to_srgb();
    pub static ref YELLOW: rgb::RGB = color::Color::new([255, 255,   0]).to_srgb();
    pub static ref WHITE:  rgb::RGB = color::Color::new([255, 255, 255]).to_srgb();
}
