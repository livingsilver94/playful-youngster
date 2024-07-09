use std::ops;

use bitmaps::Bitmap;

use super::Palette;

/// Attributes of a sprite, which is either an 8x8 or 8x16 tile.
#[derive(Clone, Copy, Default)]
pub struct ObjAttr {
    y_position: u8,
    x_position: u8,
    tile_index: u8,
    flags: ObjFlags,
}

impl ops::Index<usize> for ObjAttr {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.y_position,
            1 => &self.x_position,
            2 => &self.tile_index,
            3 => self.flags.as_ref(),
            _ => unreachable!(),
        }
    }
}

impl ops::IndexMut<usize> for ObjAttr {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.y_position,
            1 => &mut self.x_position,
            2 => &mut self.tile_index,
            3 => self.flags.as_mut(),
            _ => unreachable!(),
        }
    }
}

impl ObjAttr {
    /// The actual Y coordinate of an object is 16 pixels below the in-memory coordinate.
    fn real_y_position(&self) -> u8 {
        self.y_position + 16
    }

    /// The actual X coordinate of an object is 8 pixels below the in-memory coordinate.
    fn real_x_position(&self) -> u8 {
        self.y_position + 8
    }
}

#[derive(Clone, Copy, Default)]
struct ObjFlags(Bitmap<8>);

impl ObjFlags {
    fn priority(&self) -> bool {
        // A value of 0 means this object is prioritary; it uses a reversed logic.
        !self.0.get(7)
    }

    fn y_mirror(&self) -> bool {
        self.0.get(6)
    }

    fn x_mirror(&self) -> bool {
        self.0.get(5)
    }

    fn palette(&self) -> Palette {
        if self.0.get(4) {
            Palette::Obp1
        } else {
            Palette::Obp0
        }
    }
}

impl AsRef<u8> for ObjFlags {
    fn as_ref(&self) -> &u8 {
        self.0.as_value()
    }
}

impl AsMut<u8> for ObjFlags {
    fn as_mut(&mut self) -> &mut u8 {
        &mut self.0.as_mut()[0]
    }
}

impl From<ObjFlags> for u8 {
    fn from(value: ObjFlags) -> Self {
        *value.0.as_value()
    }
}
