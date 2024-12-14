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
    pub fn read<R: Read + Seek>(&self, mem: &mut Hardware<R>, addr: u16) -> io::Result<u8> {
        match self {
            Self::Mbc0 => mbc0::read(mem, addr),
            Self::Mbc1 => mbc1::read(mem, addr),
            Self::Mbc2 => mbc2::read(mem, addr),
            Self::Mbc3(status) => mbc3::read(*status, mem, addr),
        }
    }

    pub fn write<R: Read + Seek>(&mut self, mem: &mut Hardware<R>, addr: u16, val: u8) {
        match self {
            Self::Mbc0 => (),
            Self::Mbc1 => mbc1::write(mem, addr, val),
            Self::Mbc2 => mbc2::write(mem, addr, val),
            Self::Mbc3(status) => mbc3::write(status, mem, addr, val),
        }
    }
}
