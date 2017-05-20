use image;
use mvp;
use std;
use units;

pub fn mat4_id() -> mvp::Matrix4 {
    // TODO lazy_static!
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

// 2 / (r - l), 0,           0,            -((r + l) / (r - l))
// 0,           2 / (t - b), 0,            -((t + b) / (t - b))
// 0,           0,           -2 / (f - n), -((f + n) / (f - n))
// 0,           0,           0,          , 1

// w, 0,      0, -1
// 0, h,      0, -1
// 0, 0, -0.002, -1
// 0, 0,      0,  1
pub fn ortho_projection(screen_size: units::Size2D) -> mvp::Matrix4 {
    let o_w = 2.0 / screen_size.width as f32;
    let o_h = 2.0 / screen_size.height as f32;

    [
        [o_w, 0.0, 0.0, 0.0],
        [0.0, o_h, 0.0, 0.0],
        [0.0, 0.0, -0.002, 0.0],
        [-1.0, -1.0, -1.0, 1.0],
    ]
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


pub fn read_png_to_image(bytes: &[u8]) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    image::load(std::io::Cursor::new(bytes), image::PNG)
        .unwrap()
        .to_rgba()
}

pub fn indices(length: usize) -> Vec<u16> {
    (0..length)
        .into_iter()
        .flat_map(
            |i| {
                vec![0, 1, 2, 0, 2, 3]
                    .into_iter()
                    .map(|j| (j + i * 4) as u16)
                    .collect::<Vec<u16>>()
            },
        )
        .collect()
}
