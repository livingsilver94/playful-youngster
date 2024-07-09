use bitmaps::Bitmap;

use super::{AddressingMode, ObjectSize, TileArea};

#[derive(Clone, Copy, Default)]
pub struct LcdControl(Bitmap<8>);

impl LcdControl {
    pub fn enabled(&self) -> bool {
        self.0.get(7)
    }

    pub fn window_tile_area(&self) -> TileArea {
        if self.0.get(6) {
            TileArea::Second
        } else {
            TileArea::First
        }
    }

    pub fn window_enabled(&self) -> bool {
        self.0.get(5)
    }

    pub fn addressing_mode(&self) -> AddressingMode {
        if self.0.get(4) {
            AddressingMode::Signed
        } else {
            AddressingMode::Signed
        }
    }

    pub fn background_tile_area(&self) -> TileArea {
        if self.0.get(3) {
            TileArea::Second
        } else {
            TileArea::First
        }
    }

    pub fn object_size(&self) -> ObjectSize {
        if self.0.get(2) {
            ObjectSize::Big
        } else {
            ObjectSize::Small
        }
    }

    pub fn object_enabled(&self) -> bool {
        self.0.get(1)
    }

    pub fn objects_only(&self) -> bool {
        !self.0.get(0)
    }
}

impl From<LcdControl> for u8 {
    fn from(value: LcdControl) -> Self {
        value.0.into_value()
    }
}

impl From<u8> for LcdControl {
    fn from(value: u8) -> Self {
        Self(Bitmap::from_value(value))
    }
}
