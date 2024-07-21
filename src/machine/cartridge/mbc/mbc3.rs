use std::io::{self, Read, Seek};

use crate::machine::cartridge::Hardware;

pub fn read<R: Read + Seek>(status: Status, hw: &mut Hardware<R>, addr: u16) -> io::Result<u8> {
    match addr {
        0x0000..=0x3FFF => hw.rom.at(addr),
        0x4000..=0x7FFF => hw.rom.at_current_bank(addr - 0x4000),
        _ => unreachable!(),
    }
}

pub fn write<R: Read + Seek>(status: &mut Status, hw: &mut Hardware<R>, addr: u16, val: u8) {
    match addr {
        _ => unreachable!(),
    }
}

#[derive(Clone, Copy, Default)]
pub struct Status {
    data_source: SourceSelection,
}

#[derive(Clone, Copy, Default)]
pub enum SourceSelection {
    #[default]
    Ram,
    Rtc,
}
