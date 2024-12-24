mod oam;
mod ppu;
mod registers;
mod tile;

use oam::ObjAttr;
use registers::{LcdControl, LcdStatus, Palette};

pub const SCREEN_WIDTH: u32 = 166;
pub const SCREEN_HEIGHT: u32 = 140;

pub struct Gpu {
    lcd_control: LcdControl,
    lcd_status: LcdStatus,
    background_palette: Palette,
    object_palette0: Palette,
    object_palette1: Palette,
    background_y: u8,
    background_x: u8,
    window_y: u8,
    window_x: u8,

    /// Video random-access memory.
    vram: [u8; VRAM_SIZE],
    /// Object attribute memory, where sprite attributes are stored.
    oam: [ObjAttr; OAM_SIZE / ATTR_SIZE],

    current_mode: PpuMode,
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            lcd_control: Default::default(),
            lcd_status: Default::default(),
            background_palette: Default::default(),
            object_palette0: Default::default(),
            object_palette1: Default::default(),
            background_y: 0,
            background_x: 0,
            window_y: 0,
            window_x: 0,

            vram: [0; VRAM_SIZE],
            oam: [Default::default(); OAM_SIZE / ATTR_SIZE],

            current_mode: PpuMode::Mode0,
        }
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        if self.current_mode > PpuMode::Mode2 {
            // VRAM is inaccessible in Mode3. Any read attempt receives garbage values.
            return 0xFF;
        }
        self.vram[self.lcd_control.addressing_mode().compute_address(addr)]
    }

    pub fn write_vram(&mut self, addr: u16, val: u8) {
        if self.current_mode > PpuMode::Mode2 {
            // VRAM is inaccessible in Mode3. Any write attempt is noop.
            return;
        }
        self.vram[self.lcd_control.addressing_mode().compute_address(addr)] = val;
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        if self.current_mode > PpuMode::Mode1 {
            // OAM is inaccessible after Mode1. Any read attempt receives garbage values.
            return 0xFF;
        }
        let attr = self.oam[addr as usize / ATTR_SIZE];
        attr[addr as usize % ATTR_SIZE]
    }

    pub fn write_oam(&mut self, addr: u16, val: u8) {
        if self.current_mode > PpuMode::Mode1 {
            // OAM is inaccessible after Mode1. Any write attempt is noop.
            return;
        }
        let mut attr = self.oam[addr as usize / ATTR_SIZE];
        attr[addr as usize % ATTR_SIZE] = val;
    }

    pub fn read_register(&self, idx: usize) -> u8 {
        match idx {
            0x0 => self.lcd_control.into(),
            0x2 => self.background_y,
            0x3 => self.background_x,
            0x4 => self.lcd_status.into(),
            0x7 => self.background_palette.into(),
            0x8 => self.object_palette0.into(),
            0x9 => self.object_palette1.into(),
            0xA => self.window_y,
            0xB => self.window_x,
            _ => todo!(),
        }
    }

    pub fn write_register(&mut self, idx: usize, val: u8) {
        match idx {
            0x0 => {
                self.lcd_control = val.into();
                if !self.lcd_control.enabled() {
                    self.current_mode = PpuMode::Mode0;
                }
            }
            0x2 => self.background_y = val,
            0x3 => self.background_x = val,
            0x4 => self.lcd_status = val.into(),
            0x7 => self.background_palette = val.into(),
            0x8 => self.object_palette0 = val.into(),
            0x9 => self.object_palette1 = val.into(),
            0xA => self.window_y = val,
            0xB => self.window_x = val,
            _ => todo!(),
        }
    }

    fn real_background_coords(&self) -> (u16, u16) {
        (
            (self.background_y as u16 + 143) % 256,
            (self.background_x as u16 + 159) % 256,
        )
    }
}

#[repr(u16)]
enum TileArea {
    First = 0x9800,
    Second = 0x9C00,
}

enum ObjectSize {
    Small,
    Big,
}

impl ObjectSize {
    pub fn pixels(&self) -> (u8, u8) {
        match self {
            ObjectSize::Small => (8, 8),
            ObjectSize::Big => (8, 16),
        }
    }
}

pub enum AddressingMode {
    /// This addressing mode uses 0x0000 as the base address, plus
    /// an unsigned offset from it.
    Unsigned,

    /// This addressing mode uses 0x1000 as the base address, plus
    /// a signed offset from it.
    Signed,
}

impl AddressingMode {
    pub fn compute_address(&self, addr: u16) -> usize {
        let (base, sign) = match self {
            AddressingMode::Unsigned => (0x0, 1),
            AddressingMode::Signed => (0x1000, -1),
        };
        base + (sign * (addr as isize)) as usize
    }
}

#[derive(PartialEq, PartialOrd)]
pub enum PpuMode {
    /// HBlank Period.
    Mode0,
    /// VBlank Period.
    Mode1,
    /// Searching OAM Period.
    Mode2,
    /// Drawing pixels period.
    Mode3,
}

impl From<u8> for PpuMode {
    fn from(value: u8) -> Self {
        let value = value & 0x3;
        match value {
            x if x == Self::Mode0 as u8 => Self::Mode0,
            x if x == Self::Mode1 as u8 => Self::Mode1,
            x if x == Self::Mode2 as u8 => Self::Mode2,
            x if x == Self::Mode3 as u8 => Self::Mode3,
            _ => unreachable!(),
        }
    }
}

enum Color {
    /// The white color, or transparent in the case of objects.
    White = 0,
    LightGray = 1,
    DarkGray = 2,
    Black = 3,
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            x if x == Self::White as u8 => Self::White,
            x if x == Self::LightGray as u8 => Self::LightGray,
            x if x == Self::DarkGray as u8 => Self::DarkGray,
            x if x == Self::Black as u8 => Self::Black,
            _ => unreachable!(),
        }
    }
}

const VRAM_SIZE: usize = 8192;

const OAM_SIZE: usize = 160;
const ATTR_SIZE: usize = 4;
