mod mbc;

use std::io::{self, Read, Seek, SeekFrom};

use mbc::Mbc;

pub struct Cartridge<R: Read + Seek> {
    equipment: Equipment<R>,
    mbc: Mbc,
}

impl<R: Read + Seek> Cartridge<R> {
    pub fn new(mut data: R) -> io::Result<Self> {
        let typ = cartridge_type(&mut data)?;

        Ok(Self {
            equipment: Equipment::new(data, typ),
            mbc: Mbc::new(typ),
        })
    }
}

struct Rom<R: Read + Seek>(R);

impl<R: Read + Seek> Rom<R> {
    fn at(&mut self, addr: u16) -> io::Result<u8> {
        self.0.seek(SeekFrom::Start(addr as u64))?;
        let mut buf = [0; 1];
        self.0.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

struct Equipment<R: Read + Seek> {
    rom: Rom<R>,
    ram: Vec<u8>,
    /// Whether or not the cartridge sports a battery.
    /// The battery is used to retain values in RAM and/or
    /// power the embedded RTC. For the sake of emulation,
    /// having a battery means having to store the cartridge state
    /// in a file.
    has_battery: bool,
}

impl<R: Read + Seek> Equipment<R> {
    fn new(data: R, typ: u8) -> Self {
        let rom = Rom(data);
        let ram = match typ {
            _ => Vec::new(),
        };
        let has_battery = match typ {
            _ => false,
        };

        Self {
            rom,
            ram,
            has_battery,
        }
    }
}

fn cartridge_type<R: Read + Seek>(data: &mut R) -> io::Result<u8> {
    const CARTRIDGE_TYPE: u64 = 0x147;

    data.seek(SeekFrom::Start(CARTRIDGE_TYPE))?;
    let mut buf = [0; 1];
    data.read_exact(&mut buf)?;
    Ok(buf[0])
}
