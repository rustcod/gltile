# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project (tries to!) adhere to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- `Renderer` takes a `Tileset` that `impl TilesetLike` (needed for custom tilesets), provided by `pix!` macro
- removed `VertexBuffer` as it was not doing anything
- removed `MagnifySamplerFilter` on the tileset uniform; fixes single pixel lines on tiles
- bumped `glium`, `image`, and `lazy_static`
- updated examples for `glium` `0.17.0`
- cargo fmt

### Removed
- *breaking* removed `units`, `mvp`, and `utils` from public interface

## [0.0.2] - 2017-07-09

### Added
- support for non-square tilesets
- `non_square` example (16x24 tileset)
- `non_square_2` example (24x16 tileset)

### Changed
- bumped `pixset`, `looper`, and `pixset_derive` to `0.0.2`

## [0.0.1] - 2017-07-04

### Added
- initial impl
