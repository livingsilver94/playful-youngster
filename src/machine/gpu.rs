mod oam;
mod registers;

use crate::machine::memory::RegisterMapping;
use oam::ObjAttr;
use registers::{LcdControl, LcdStatus};

pub struct Gpu {
    lcd_control: LcdControl,
    lcd_status: LcdStatus,
    background_y: u8,
    background_x: u8,

    /// Video random-access memory.
    vram: [u8; VRAM_SIZE],
    /// Object attribute memory, where sprite attributes are stored.
    oam: [ObjAttr; OAM_SIZE / ATTR_SIZE],
}

impl Gpu {
    pub fn new_gb() -> Self {
        Self {
            lcd_control: Default::default(),
            lcd_status: Default::default(),
            background_y: 0,
            background_x: 0,

            vram: [0; VRAM_SIZE],
            oam: [Default::default(); OAM_SIZE / ATTR_SIZE],
        }
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        self.vram[self.lcd_control.addressing_mode().compute_address(addr)]
    }

    pub fn write_vram(&mut self, addr: u16, val: u8) {
        self.vram[self.lcd_control.addressing_mode().compute_address(addr)] = val;
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        let attr = self.oam[addr as usize / ATTR_SIZE];
        attr[addr as usize % ATTR_SIZE]
    }

    pub fn write_oam(&mut self, addr: u16, val: u8) {
        let mut attr = self.oam[addr as usize / ATTR_SIZE];
        attr[addr as usize % ATTR_SIZE] = val;
    }

    fn real_background_coords(&self) -> (u16, u16) {
        (
            (self.background_y as u16 + 143) % 256,
            (self.background_x as u16 + 159) % 256,
        )
    }
}

impl RegisterMapping for Gpu {
    fn read_register(&self, idx: usize) -> u8 {
        match idx {
            0x0 => self.lcd_control.into(),
            0x4 => self.lcd_status.into(),
            0xA => self.background_y,
            0xB => self.background_x,
            _ => todo!(),
        }
    }

    fn write_register(&mut self, idx: usize, val: u8) {
        match idx {
            0x0 => self.lcd_control = val.into(),
            0x4 => self.lcd_status = val.into(),
            0xA => self.background_y = val,
            0xB => self.background_x = val,
            _ => todo!(),
        }
    }
}

enum Palette {
    Obp0,
    Obp1,
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

pub enum PpuMode {
    Mode0,
    Mode1,
    Mode2,
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

const VRAM_SIZE: usize = 8192;

const OAM_SIZE: usize = 160;
const ATTR_SIZE: usize = 4;
