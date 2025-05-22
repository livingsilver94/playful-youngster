mod lcdcontrol;
mod lcdstatus;
mod palette;

use crate::hardware::gpu::{
    lcdcontrol::LcdControl,
    lcdstatus::{LcdStatus, PpuMode},
    palette::PaletteIds,
};

pub struct Gpu {
    /// Contains tiles, that are 8x8 pixel images, with each pixel taking 2 bytes.
    tile_data: Box<[u8; 6144]>,
    /// First tile map.
    /// A tile map contains the 1-byte indexes of tiles in [`Self::tile_data`].
    /// The actual memory address the index points to depends on the LCDC register.
    tile_map1: [u8; 1024],
    /// Second tile map.
    /// A tile map contains the 1-byte indexes of tiles in [`Self::tile_data`].
    /// The actual memory address the index points to depends on the LCDC register.
    tile_map2: [u8; 1024],
    /// Object Attribute Memory, that is metadata associated to each sprite.
    oam: [u8; 160],

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

    /// Palette IDs used for the background.
    background_palette: PaletteIds,
    /// First set of palette IDs used for objects.
    /// White can't be used for objects, as it actually means transparent.
    object_palette0: PaletteIds,
    /// Second set of palette IDs used for objects.
    /// White can't be used for objects, as it actually means transparent.
    object_palette1: PaletteIds,
}

impl Default for Gpu {
    fn default() -> Self {
        Self {
            tile_data: Box::new([0; 6144]),
            tile_map1: [0; 1024],
            tile_map2: [0; 1024],
            oam: [0; 160],
            background_x: Default::default(),
            background_y: Default::default(),
            window_x: Default::default(),
            window_y: Default::default(),
            lcd_control: Default::default(),
            lcd_status: Default::default(),
            lcd_y: Default::default(),
            lcd_y_compare: Default::default(),
            background_palette: Default::default(),
            object_palette0: Default::default(),
            object_palette1: Default::default(),
        }
    }
}

impl Gpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        if self.lcd_status.ppu_mode() >= PpuMode::Mode3 {
            return 0xFF;
        }
        match addr {
            0x0000..=0x17FF => self.tile_data[addr as usize],
            0x1800..=0x1BFF => self.tile_map1[addr as usize - 0x1800],
            0x1C00..=0x1FFF => self.tile_map2[addr as usize - 0x1C00],
            _ => unreachable!(),
        }
    }

    pub fn write_vram(&mut self, addr: u16, value: u8) {
        if self.lcd_status.ppu_mode() >= PpuMode::Mode3 {
            return;
        }
        match addr {
            0x0000..=0x17FF => self.tile_data[addr as usize] = value,
            0x1800..=0x1BFF => self.tile_map1[addr as usize - 0x1800] = value,
            0x1C00..=0x1FFF => self.tile_map2[addr as usize - 0x1C00] = value,
            _ => unreachable!(),
        }
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        if self.lcd_status.ppu_mode() >= PpuMode::Mode2 {
            return 0xFF;
        }
        self.oam[addr as usize]
    }

    pub fn write_oam(&mut self, addr: u16, value: u8) {
        if self.lcd_status.ppu_mode() >= PpuMode::Mode2 {
            return;
        }
        self.oam[addr as usize] = value;
    }

    pub fn read_register(&self, addr: u16) -> u8 {
        match addr {
            0x00 => From::from(self.lcd_control),
            0x01 => From::from(self.lcd_status),
            0x02 => self.background_y,
            0x03 => self.background_x,
            0x04 => self.lcd_y,
            0x05 => self.lcd_y_compare,
            0x07 => From::from(self.background_palette),
            0x08 => From::from(self.object_palette0),
            0x09 => From::from(self.object_palette1),
            0x0A => self.window_y,
            0x0B => self.window_x,
            _ => unreachable!(),
        }
    }

    pub fn write_register(&mut self, addr: u16, val: u8) {
        match addr {
            0x00 => self.lcd_control = From::from(val),
            0x01 => {
                // Last 2 bits are read-only.
                let old_val = u8::from(self.lcd_status);
                self.lcd_status = From::from((val & 0b11111100) | (old_val & 0b00000011));
            }
            0x02 => self.background_y = val,
            0x03 => self.background_x = val,
            0x04 => (), // LCY is read-only.
            0x05 => self.lcd_y_compare = val,
            0x07 => self.background_palette = From::from(val),
            0x08 => self.object_palette0 = From::from(val),
            0x09 => self.object_palette1 = From::from(val),
            0x0A => self.window_y = val,
            0x0B => self.window_x = val,
            _ => unreachable!(),
        }
    }
}
