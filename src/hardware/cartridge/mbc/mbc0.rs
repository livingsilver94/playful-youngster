use std::io::{self, Read, Seek};

use crate::hardware::cartridge::Hardware;

pub fn read<R: Read + Seek>(hw: &mut Hardware<R>, addr: u16) -> io::Result<u8> {
    match addr {
        0x0000..=0x7FFF => hw.rom.at(addr),
        0xA000..=0xBFFF => Ok(hw.ram.read(addr - 0xA000)),
        _ => unreachable!(),
    }
}
