mod lcd_control;
mod oam;

use crate::machine::memory::RegisterMapping;
use lcd_control::LcdControl;
use oam::ObjAttr;

pub struct Gpu {
    lcd_control: LcdControl,

    /// Video random-access memory.
    vram: [u8; VRAM_SIZE],
    /// Object attribute memory, where sprite attributes are stored.
    oam: [ObjAttr; OAM_SIZE / ATTR_SIZE],
}

impl Gpu {
    pub fn new_gb() -> Self {
        Self {
            lcd_control: Default::default(),
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
}

impl RegisterMapping for Gpu {
    fn read_register(&self, idx: usize) -> u8 {
        match idx {
            0 => self.lcd_control.into(),
            _ => todo!(),
        }
    }

    fn write_register(&mut self, idx: usize, val: u8) {
        match idx {
            0 => self.lcd_control = val.into(),
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

const VRAM_SIZE: usize = 8192;

const OAM_SIZE: usize = 160;
const ATTR_SIZE: usize = 4;
