use std::io::{self, Read, Seek};

/// The logic that a cartridge follows according to its hardware.
pub enum Mbc {
    Mbc0,
}

impl Mbc {
    pub fn new(typ: u8) -> Self {
        match typ {
            0x0 | 0x8..=0x9 => Self::Mbc0 {},
            _ => todo!(),
        }
    }

    pub fn read<R: Read + Seek>(
        &self,
        equipment: &mut super::Equipment<R>,
        addr: u16,
    ) -> io::Result<u8> {
        match self {
            Self::Mbc0 => mbc0_read(equipment, addr),
        }
    }

    pub fn write<R: Read + Seek>(&mut self, _equipment: &mut super::Equipment<R>, _addr: u16) {
        match self {
            _ => (),
        }
    }
}

fn mbc0_read<R: Read + Seek>(equipment: &mut super::Equipment<R>, addr: u16) -> io::Result<u8> {
    match addr {
        0x0000..=0x7FFF => equipment.rom.at(addr),
        0xA000..=0xBFFF => Ok(equipment.ram[(addr - 0xA000) as usize]),
        _ => unreachable!(),
    }
}
