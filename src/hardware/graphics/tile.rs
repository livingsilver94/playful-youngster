use std::ops::Deref;

struct Tile([u8; TILE_SIZE]);

impl Deref for Tile {
    type Target = [u8; TILE_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Size, in bytes, of a single tile.
///
/// A tile is composed of 8x8 pixels. Each pixel is associated to a Palette,
/// identified by 2 bits. This makes a total of 8x8x2 **bits**, that are 16 **bytes**.
const TILE_SIZE: usize = 16;
