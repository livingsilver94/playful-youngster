//! The `header` module allows to parse the header contained in every cartridge.
//! This header is fixed in size and provides the identity of the cartridge:
//! as an emulator, we are interested in the memory controller, ROM and RAM size, and
//! whether a battery is included.
//!
//! See https://gbdev.io/pandocs/The_Cartridge_Header.html#the-cartridge-header

use std::io::{self, Read, Seek};

use crate::machine::cartridge::{self, mbc::Mbc};

#[derive(Clone, Copy)]
pub struct CartridgeType(u8);

impl CartridgeType {
    pub fn mbc(&self) -> Mbc {
        match self.0 {
            0x00 | 0x08..=0x09 => Mbc::Mbc0,
            0x01..=0x03 => Mbc::Mbc1,
            0x05..=0x06 => Mbc::Mbc2,
            0x0F..=0x13 => Mbc::Mbc3(Default::default()),
            _ => todo!(),
        }
    }

    pub fn has_battery(&self) -> bool {
        [0x03, 0x06, 0x09, 0x0D, 0x0F, 0x10, 0x13, 0x1B, 0x1E, 0x22].contains(&self.0)
    }
}

pub fn cartridge_type<R: Read + Seek>(data: &mut R) -> io::Result<CartridgeType> {
    cartridge::read_at(data, 0x147).map(|code| CartridgeType(code))
}

pub fn rom_banks<R: Read + Seek>(data: &mut R) -> io::Result<u8> {
    Ok(match cartridge::read_at(data, 0x148)? {
        code if (0x00..=0x08).contains(&code) => (1 << code) + 1,
        0x52 => 72,
        0x53 => 80,
        0x54 => 96,
        _ => unreachable!(),
    })
}

pub fn ram_banks<R: Read + Seek>(data: &mut R, cartridge_type: CartridgeType) -> io::Result<u8> {
    Ok(match cartridge::read_at(data, 0x149)? {
        0x00 => {
            // MBC2 has 512 half-bytes or RAM, but it's internal, so ram_banks
            // is technically zero. However, we don't emulate the hardware layout precisely,
            // so we pretend it has 1 bank of regular RAM.
            if (0x05..=0x06).contains(&cartridge_type.0) {
                1
            } else {
                0
            }
        }
        0x02 => 1,
        0x03 => 4,
        0x04 => 16,
        0x05 => 8,
        _ => unreachable!(),
    })
}
