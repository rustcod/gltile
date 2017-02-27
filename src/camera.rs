use data;
use mvp;

#[derive(Debug)]
pub struct Camera {
    screen_size: data::Size, // dimensions in pixels
    loc: data::Loc, // upper left game coords
    dim: data::Size, // dimensions in tiles
    mat: mvp::Matrix4,
    square_size: i32, // number of pixels in a tile
    halve: bool,
}

impl Camera {
    pub fn new(screen_size: data::Size, loc: data::Loc, square_size: i32, halve: bool) -> Camera {
        use utils;

        let width = screen_size.width as i32 / square_size;
        let height = screen_size.height as i32 / square_size;

        Camera {
            screen_size: screen_size,
            loc: loc,
            dim: data::Size {
                // TODO new
                width: width,
                height: height,
            },
            mat: utils::mat4_id(),
            square_size: screen_size.width as i32 / width,
            halve: halve,
        }
    }

    pub fn as_mat(&mut self) -> mvp::Matrix4 {
        let x_offset = (self.loc.x * self.square_size) as f32;
        let y_offset = ((self.loc.y + 1) * self.square_size) as f32;

        let x_o = -(self.screen_size.width as f32 / 2.0) - x_offset;
        let y_o = (self.screen_size.height as f32 / 2.0) - y_offset;

        self.mat[3][0] = x_o;
        self.mat[3][1] = y_o;
        self.mat
    }

    pub fn loc(&self) -> data::Loc {
        self.loc
    }

    // pub fn pan(&mut self, dir: Dir) -> &mut Self {
    //    match dir {
    //        Dir::Right => self.loc.x += 1,
    //        Dir::Left => self.loc.x -= 1,
    //        Dir::Up => self.loc.y += 1,
    //        Dir::Down => self.loc.y -= 1,
    //    }

    //    self
    //

    pub fn to_game_loc(&self, pixel_loc: data::PixelLoc) -> data::Loc {
        let mut x = pixel_loc.x as f32 / self.square_size as f32;
        let mut y = pixel_loc.y as f32 / self.square_size as f32;

        if self.halve {
            x = x / 2.0;
            y = y / 2.0;
        }

        // TODO new
        data::Loc {
            x: x as i32 + self.loc.x,
            y: -(y as i32 - self.loc.y),
        }
    }
}
