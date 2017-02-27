use rgb;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn srgb(color: u8) -> f32 {
    ((((color as f32 / 255.0) + 0.055) / 1.055) as f32).powf(2.4)
}

impl Color {
    pub fn new(colors: [u8; 3]) -> Color {
        Color {
            r: colors[0],
            g: colors[1],
            b: colors[2],
        }
    }

    pub fn to_srgb(&self) -> rgb::RGB {
        [srgb(self.r), srgb(self.g), srgb(self.b)]
    }
}
