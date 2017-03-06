use palette;
use palette::pixel::RgbPixel;

pub type Rgb = [f32; 3];
pub type Srgb = [f32; 3];

fn normalize(color: Srgb) -> Srgb {
    [color[0] / 255.0, color[1] / 255.0, color[2] / 255.0]
}

fn srgb_to_rgb(color: Srgb) -> Rgb {
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
    pub static ref BLACK:     Rgb = str_to_rgb("black");
    pub static ref CYAN:      Rgb = str_to_rgb("cyan");
    pub static ref BROWN:     Rgb = str_to_rgb("saddlebrown");
    pub static ref BLUE:      Rgb = str_to_rgb("blue");
    pub static ref RED:       Rgb = str_to_rgb("red");
    pub static ref GREEN:     Rgb = str_to_rgb("green");
    pub static ref YELLOW:    Rgb = str_to_rgb("yellow");
    pub static ref WHITE:     Rgb = str_to_rgb("white");

    pub static ref YOGA_TEAL: Rgb = srgb_to_rgb(normalize([151.0, 220.0, 207.0]));
    pub static ref YOGA_GRAY: Rgb = srgb_to_rgb(normalize([48.0,   56.0,  70.0]));
}
