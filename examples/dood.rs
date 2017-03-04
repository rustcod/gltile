#[macro_use]
extern crate glium;

#[macro_use]
extern crate lazy_static;

extern crate image;
extern crate gltile;

fn main() {
    use glium::Surface;

    let renderer = gltile::Renderer::new(1536 /* 96 */, 1024 /* 64 */, 16);

    let texture = {
        let png = gltile::read_bytes("assets/tileset.png").unwrap();
        let tileset_texture = gltile::read_png_to_texture(&png[..]);
        glium::texture::Texture2d::new(&renderer.display, tileset_texture).unwrap()
    };

    let tileset = texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);

    let cam_uniforms = {
        let mat4_id = gltile::mat4_id();
        let ortho_projection = gltile::ortho_projection(renderer.screen_size);

        let cam = [[1.0, 0.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0, 0.0],
                   [0.0, 0.0, 1.0, 0.0],
                   [-768.0, -512.0, 0.0, 1.0]];

        uniform! {
            mvp: gltile::model_view_projection(mat4_id, cam, ortho_projection),
            tileset: tileset,
        }
    };

    let vb = {
        let mut vb = gltile::VertexBuffer::new(renderer.size);

        let tile = {
            let mut tile = gltile::Tile::new();
            tile.fg = *gltile::colors::YELLOW;
            tile.pix = gltile::Pix::Dood;
            tile
        };

        vb.set(gltile::WindowLoc { x: 5, y: 5 }, tile);

        // TODO hide this as an implementation detail
        glium::VertexBuffer::new(&renderer.display, &vb.data()).unwrap()
    };

    loop {
        let mut target = renderer.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vb,
                  &renderer.indices,
                  &renderer.program,
                  &cam_uniforms,
                  &Default::default())
            .unwrap();
        target.finish().unwrap();

        for ev in renderer.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
