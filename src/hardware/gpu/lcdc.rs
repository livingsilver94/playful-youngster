#[derive(Default)]
pub struct LcdControl(bitflags::BitFlags8);

impl From<u8> for LcdControl {
    fn from(value: u8) -> Self {
        Self(bitflags::BitFlags8::from(value))
    }
}

impl LcdControl {
    /// Returns whether the LCD is enabled.
    /// If not enabled, the screen is blank.
    pub fn lcd_enabled(&self) -> bool {
        self.0.get(7)
    }

    /// Returns which tile map is currently used to render the window.
    pub fn window_tilemap(&self) -> Tilemap {
        if self.0.get(6) {
            Tilemap::Second
        } else {
            Tilemap::First
        }
    }

    /// Returns whether the window shall be displayed or not.
    pub fn window_enabled(&self) -> bool {
        self.0.get(5)
    }

    /// Returns which addressing mode is currently in use.
    pub fn addressing_mode(&self) -> AddrMode {
        if self.0.get(4) {
            AddrMode::Unsigned
        } else {
            AddrMode::Signed
        }
    }

    /// Returns which tile map is currently used to render the background.
    pub fn background_tilemap(&self) -> Tilemap {
        if self.0.get(3) {
            Tilemap::Second
        } else {
            Tilemap::First
        }
    }

    /// Returns the object size.
    pub fn object_size(&self) -> ObjSize {
        if self.0.get(2) {
            ObjSize::TwoTiles
        } else {
            ObjSize::OneTile
        }
    }

    /// Returns whether objects are rendered.
    pub fn objects_enabled(&self) -> bool {
        self.0.get(1)
    }

    /// Returns whether only objects are rendered.
    /// When so, window and background are white.
    pub fn objects_only(&self) -> bool {
        !self.0.get(0)
    }
}

/// Which tile map is used for rendering.
pub enum Tilemap {
    /// First tile map.
    First,
    /// Second tile map.
    Second,
}

/// Addressing mode for indexes contained in tile maps.
pub enum AddrMode {
    /// The unsigned addressing mode uses the first byte of tile data as pivot point.
    /// Actual memory addresses are computed as `0 + index`, where `index` is unsigned.
    Unsigned,
    /// The signed addressing mode uses the byte 4096 of tile data as pivot point.
    /// Actual memory addresses are computed as `4096 + index`, where `index` is signed.
    Signed,
}

/// Object size in use.
pub enum ObjSize {
    /// Objects are one-tile big.
    OneTile,
    /// Objects are two-tiles big, stacked vertically.
    TwoTiles,
}
