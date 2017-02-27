use palette;

fn str_to_rgba(color: &str) -> [f32; 3] {
    let rgba = palette::named::from_str(color)
        .expect(&format!("unknown color: {}", color))
        .to_rgba();
    palette::pixel::RgbPixel::from_rgba(rgba.0, rgba.1, rgba.2, 1.0)
}

lazy_static! {
    pub static ref BLACK:  [f32; 3] = str_to_rgba("black");
    pub static ref BROWN:  [f32; 3] = str_to_rgba("saddlebrown");
    pub static ref BLUE:   [f32; 3] = str_to_rgba("blue");
    pub static ref RED:    [f32; 3] = str_to_rgba("red");
    pub static ref GREEN:  [f32; 3] = str_to_rgba("green");
    pub static ref YELLOW: [f32; 3] = str_to_rgba("yellow");
    pub static ref WHITE:  [f32; 3] = str_to_rgba("white");
}
