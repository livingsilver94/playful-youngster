mod mbc0;
mod mbc1;
mod mbc2;
mod mbc3;

use std::io::{self, Read, Seek};

use crate::machine::cartridge::Hardware;

/// The logic that a cartridge follows according to its hardware.
pub enum Mbc {
    Mbc0,
    Mbc1, // NOTE: this does not emulate MBC1M.
    Mbc2,
    Mbc3(mbc3::Status),
}

impl Mbc {
    pub fn new(typ: u8) -> Self {
        match typ {
            0x00 | 0x08..=0x09 => Self::Mbc0,
            0x01..=0x03 => Self::Mbc1,
            0x05..=0x06 => Self::Mbc2,
            0x0F..=0x13 => Self::Mbc3(Default::default()),
            _ => todo!(),
        }
    }

    pub fn read<R: Read + Seek>(&self, hw: &mut super::Hardware<R>, addr: u16) -> io::Result<u8> {
        match self {
            Self::Mbc0 => mbc0::read(hw, addr),
            Self::Mbc1 => mbc1::read(hw, addr),
            Self::Mbc2 => mbc2::read(hw, addr),
            Self::Mbc3(status) => mbc3::read(*status, hw, addr),
        }
    }

    pub fn write<R: Read + Seek>(&mut self, hw: &mut Hardware<R>, addr: u16, val: u8) {
        match self {
            Self::Mbc0 => (),
            Self::Mbc1 => mbc1::write(hw, addr, val),
            Self::Mbc2 => mbc2::write(hw, addr, val),
            Self::Mbc3(status) => mbc3::write(status, hw, addr, val),
        }
    }
}
