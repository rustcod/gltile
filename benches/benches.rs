#![feature(test)]

extern crate gltile;
extern crate pixset;
extern crate test;

use test::Bencher;

#[bench]
fn bench_vertex_data_set(b: &mut Bencher) {
    let pixset = pixset::Pixset::new(100, 16);
    let size = gltile::units::Size2D::new(40, 40);
    let mut vb = gltile::VertexData::new(size, &pixset);
    let st = gltile::units::ScreenTile2D::new(0, 0);
    let tile = gltile::Tile::make(
        *gltile::colors::YELLOW,
        *gltile::colors::BLACK,
        pixset::Pix::A,
    );
    let coords = pixset.get(&tile.pix);
    b.iter(|| vb.set(st, tile, coords));
}
