use palette;
use palette::pixel::RgbPixel;

pub type Srgb = [f32; 3];

fn str_to_rgba(color: &str) -> [f32; 3] {
    let rgba = palette::named::from_str(color)
        .expect(&format!("unknown color: {}", color))
        .to_rgba();
    palette::pixel::RgbPixel::from_rgba(rgba.0, rgba.1, rgba.2, 1.0)
}

lazy_static! {
    pub static ref BLACK:  Srgb = str_to_rgba("black");
    pub static ref BROWN:  Srgb = str_to_rgba("saddlebrown");
    pub static ref BLUE:   Srgb = str_to_rgba("blue");
    pub static ref RED:    Srgb = str_to_rgba("red");
    pub static ref GREEN:  Srgb = str_to_rgba("green");
    pub static ref YELLOW: Srgb = str_to_rgba("yellow");
    pub static ref WHITE:  Srgb = str_to_rgba("white");
}
