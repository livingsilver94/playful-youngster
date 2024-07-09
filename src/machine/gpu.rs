mod oam;
mod vram;

use oam::ObjAttr;
use vram::{AddressingMode, Vram};
use crate::machine::memory::RegisterMapping;

pub struct Gpu {
    enabled: bool,
    window_enabled: bool,

    /// Video random-access memory.
    vram: Vram,
    /// Object attribute memory, where sprite attributes are stored.
    oam: [ObjAttr; OAM_SIZE / ATTR_SIZE],
}

impl Gpu {
    pub fn new_gb() -> Self {
        Self {
            enabled: false,
            window_enabled: false,
            vram: Vram::new(),
            oam: [Default::default(); OAM_SIZE / ATTR_SIZE],
        }
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        self.vram.read_byte(addr)
    }

    pub fn write_vram(&mut self, addr: u16, val: u8) {
        self.vram.write_byte(addr, val);
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
            // LCDC: LCD control.
            0 => {
                todo!()
            }
            _ => todo!(),
        }
    }

    fn write_register(&mut self, idx: usize, val: u8) {
        match idx {
            // LCDC: LCD control.
            0 => {
                self.enabled = val & 0b1000000 != 0;
                self.window_enabled = val & 0b0010000 != 0;
                self.vram.addressing_mode = if val & 0b00010000 != 0 {
                    AddressingMode::Unsigned
                } else {
                    AddressingMode::Signed
                }
            }
            _ => todo!(),
        }
    }
}

pub enum Palette {
    Obp0,
    Obp1,
}

const OAM_SIZE: usize = 160;
const ATTR_SIZE: usize = 4;
