use std::io::{self, Read, Seek};

use crate::hardware::cartridge::Hardware;

pub fn read<R: Read + Seek>(hw: &mut Hardware<R>, addr: u16) -> io::Result<u8> {
    match addr {
        0x0000..=0x3FFF => hw.rom.at(addr),
        0x4000..=0x7FFF => hw.rom.at_current_bank(addr - 0x4000),
        0xA000..=0xA1FF => Ok(hw.ram.read(addr - 0xA000)),
        0xA200..=0xBFFF => Ok(hw.ram.read(addr - 0xA200)), // Just echoes the above.
        _ => unreachable!(),
    }
}

pub fn write<R: Read + Seek>(hw: &mut Hardware<R>, addr: u16, val: u8) {
    match addr {
        0x0000..=0x3FFF => {
            // The LSB of the address controls whether
            // we are going to set ROM or RAM.
            if addr & 0b00000001 == 0 {
                hw.ram.enabled = val == 0x0A;
            } else {
                hw.rom.set_bank(val & 0b00001111);
            }
        }
        0xA000..=0xA1FF => hw.ram.write(addr - 0xA000, val),
        0xA200..=0xBFFF => hw.ram.write(addr - 0xA200, val), // Just echoes the above.
        _ => unreachable!(),
    }
}
