use crate::hardware::gpu::lcdc::LcdControl;

mod lcdc;

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
