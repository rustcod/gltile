use glium;
use mvp;
use std;
use units;

pub fn mat4_id() -> mvp::Matrix4 {
    // TODO lazy_static!
    [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]
}

pub fn ortho_projection(screen_size: units::Size2D) -> mvp::Matrix4 {
    let o_w = 2.0 / screen_size.width as f32;
    let o_h = 2.0 / screen_size.height as f32;

    // TODO make this better?
    [[o_w, 0.0, 0.0, 0.0], [0.0, o_h, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]
}

pub fn read_file(path: &str) -> std::io::Result<(String)> {
    use std::io::Read;
    use std::fs::File;

    let mut file = try!(File::open(path));
    let mut string = String::new();
    try!(file.read_to_string(&mut string));
    Ok(string)
}

pub fn read_bytes(path: &str) -> std::io::Result<Vec<u8>> {
    use std::io::Read;
    use std::fs::File;

    let mut file = try!(File::open(path));
    let mut bytes = Vec::new();
    try!(file.read_to_end(&mut bytes));
    Ok(bytes)
}


pub fn read_png_to_texture(bytes: &[u8]) -> glium::texture::RawImage2d<u8> {
    use image;

    let image = image::load(std::io::Cursor::new(bytes), image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions)
}

pub fn indices(length: usize) -> Vec<u16> {
    (0..length)
        .into_iter()
        .flat_map(|i| {
            vec![0, 1, 2, 0, 2, 3].into_iter().map(|j| (j + i * 4) as u16).collect::<Vec<u16>>()
        })
        .collect()
}
