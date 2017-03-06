use palette;
use palette::pixel::RgbPixel;

pub type Srgb = [f32; 3]; // TODO rename rgb

fn normalize(color: [f32; 3]) -> [f32; 3] {
    [color[0] / 255.0, color[1] / 255.0, color[2] / 255.0]
}

fn srgb_to_rgb(color: [f32; 3]) -> [f32; 3] {
    let srgb = palette::pixel::Srgb::new(color[0], color[1], color[2]);
    let rgb = palette::Rgb::from(srgb);
    [rgb.red, rgb.green, rgb.blue]
}

fn str_to_rgb(color: &str) -> Srgb {
    let rgba = palette::named::from_str(color)
        .expect(&format!("unknown color: {}", color))
        .to_rgba();
    srgb_to_rgb([rgba.0, rgba.1, rgba.2])
}

lazy_static! {
    pub static ref BLACK:      Srgb = str_to_rgb("black");
    pub static ref CYAN:       Srgb = str_to_rgb("cyan");
    pub static ref BROWN:      Srgb = str_to_rgb("saddlebrown");
    pub static ref BLUE:       Srgb = str_to_rgb("blue");
    pub static ref RED:        Srgb = str_to_rgb("red");
    pub static ref GREEN:      Srgb = str_to_rgb("green");
    pub static ref YELLOW:     Srgb = str_to_rgb("yellow");
    pub static ref WHITE:      Srgb = str_to_rgb("white");

    pub static ref YOGA_TEAL:  Srgb = srgb_to_rgb(normalize([151.0, 220.0, 207.0]));
    pub static ref YOGA_GRAY:  Srgb = srgb_to_rgb(normalize([48.0,   56.0,  70.0]));
}
