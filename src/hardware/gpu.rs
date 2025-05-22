mod lcdcontrol;
mod lcdstatus;

use crate::hardware::gpu::{lcdcontrol::LcdControl, lcdstatus::LcdStatus};

pub struct Gpu {
    /// Contains tiles, that are 8x8 pixel images, with each pixel taking 2 bytes.
    tile_data: [u8; 4096],
    /// First tile map.
    /// A tile map contains the 1-byte indexes of tiles in [`Self::tile_data`].
    /// The actual memory address the index points to depends on the LCDC register.
    tile_map1: [u8; 1024],
    /// Second tile map.
    /// A tile map contains the 1-byte indexes of tiles in [`Self::tile_data`].
    /// The actual memory address the index points to depends on the LCDC register.
    tile_map2: [u8; 1024],

    /// The X coordinate of the background viewport.
    /// It corresponds to register SCX.
    background_x: u8,
    /// The Y coordinate of the background viewport.
    /// It corresponds to register SCY.
    background_y: u8,

    /// The X coordinate of the window viewport.
    /// It corresponds to register WX.
    window_x: u8,
    /// The Y coordinate of the window viewport.
    /// It corresponds to register WY.
    window_y: u8,

    lcd_control: LcdControl,
    lcd_status: LcdStatus,

    /// Indicates the current horizontal line, which might be
    /// about to be drawn, being drawn, or just been drawn.
    /// It is the LY register.
    lcd_y: u8,
    /// Contains an arbitrary value chosen by games.
    /// When the value is equal to [`Self::lcd_y`], the "LYC=LY" flag
    /// in [`Self::lcd_status`] is set, and (if enabled) a STAT interrupt is requested.
    lcd_y_compare: u8,
}

impl Default for Gpu {
    fn default() -> Self {
        Self {
            tile_data: [0; 4096],
            tile_map1: [0; 1024],
            tile_map2: [0; 1024],
            background_x: Default::default(),
            background_y: Default::default(),
            window_x: Default::default(),
            window_y: Default::default(),
            lcd_control: Default::default(),
            lcd_status: Default::default(),
            lcd_y: Default::default(),
            lcd_y_compare: Default::default(),
        }
    }
}

impl Gpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        todo!()
    }

    pub fn write_vram(&mut self, addr: u16, value: u8) {
        todo!()
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        todo!()
    }

    pub fn write_oam(&mut self, addr: u16, value: u8) {
        todo!()
    }
}
