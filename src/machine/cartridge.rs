mod mbc;

use std::io::{self, Read, Seek, SeekFrom};

use mbc::Mbc;

pub struct Cartridge<R: Read + Seek> {
    hw: Hardware<R>,
    mbc: Mbc,
}

impl<R: Read + Seek> Cartridge<R> {
    pub fn new(mut data: R) -> io::Result<Self> {
        const TYPE: u16 = 0x147;
        const ROM_SIZE: u16 = 0x148;
        const RAM_SIZE: u16 = 0x149;

        let typ = read_at(&mut data, TYPE)?;
        let rom_size = read_at(&mut data, ROM_SIZE)?;
        let ram_size = read_at(&mut data, RAM_SIZE)?;
        Ok(Self {
            hw: Hardware::from_cardridge_header(data, typ, rom_size, ram_size),
            mbc: Mbc::new(typ),
        })
    }
}

struct Rom<R: Read + Seek> {
    data: R,
    /// Number of banks composing the ROM.
    banks: u8,
    /// The currently selected ROM bank.
    /// The default value is 1, since the first
    /// [`Self::BANK_SIZE`] bytes are directly accessible.
    curr_bank: u8,
}

impl<R: Read + Seek> Rom<R> {
    /// Size, in bytes, of each ROM bank.
    const BANK_SIZE: u16 = 16 * 1024;

    fn new(data: R, banks: u8) -> Self {
        Self {
            data,
            banks,
            curr_bank: 1,
        }
    }

    /// Reads data at an absolute address.
    fn at(&mut self, addr: u16) -> io::Result<u8> {
        read_at(&mut self.data, addr)
    }

    /// Reads data relative to the currently selected bank.
    fn at_current_bank(&mut self, addr: u16) -> io::Result<u8> {
        self.at(self.curr_bank as u16 * Self::BANK_SIZE + addr)
    }

    fn set_bank(&mut self, bank: u8) {
        let mut bank = bank & 0b00011111; // The bank is addressed by 5 bits.
        bank = if bank == 0 { 1 } else { bank };
        // If bank number is too high, it is masked by the amount of bits
        // required to represent the bank count.
        bank &= (1 << self.banks.ilog2() as u8) - 1;

        self.curr_bank = bank;
    }

    /// Sets the upper two bits of the current bank selection.
    fn set_bank_extended(&mut self, ext: u8) {
        let ext = ext & 0b00000011;
        self.curr_bank = (self.curr_bank & 0b0011111) + (ext << 5);
    }
}

struct Ram {
    data: Vec<u8>,
    banks: u8,
    curr_bank: u8,
    enabled: bool,
}

impl Ram {
    /// Size, in bytes, of each RAM bank.
    const BANK_SIZE: u16 = 8 * 1024;

    fn new(banks: u8) -> Self {
        Self {
            data: vec![0; banks as usize * Self::BANK_SIZE as usize],
            banks,
            curr_bank: 0,
            enabled: false,
        }
    }

    /// Reads data at an absolute address.
    fn read(&self, addr: u16) -> u8 {
        if !self.enabled {
            return 0xFF;
        }
        self.data[addr as usize]
    }

    /// Reads data relative to the currently selected bank.
    fn read_current_bank(&mut self, addr: u16) -> u8 {
        self.read(self.curr_bank as u16 * Self::BANK_SIZE + addr)
    }

    /// Writes data at an absolute address.
    fn write(&mut self, addr: u16, val: u8) {
        if !self.enabled {
            return;
        }
        self.data[addr as usize] = val;
    }

    /// Writes data relative to the currently selected bank.
    fn write_current_bank(&mut self, addr: u16, val: u8) {
        self.write(self.curr_bank as u16 * Self::BANK_SIZE + addr, val)
    }

    fn set_bank(&mut self, bank: u8) {
        self.curr_bank = bank & 0b00000011;
    }
}

struct Hardware<R: Read + Seek> {
    rom: Rom<R>,
    ram: Ram,
    /// Whether or not the cartridge sports a battery.
    /// The battery is used to retain values in RAM and/or
    /// power the embedded RTC. As far as the emulation is concerned,
    /// having a battery means having to store the cartridge state
    /// in a file.
    has_battery: bool,
    banking_mode: BankingMode,
}

impl<R: Read + Seek> Hardware<R> {
    fn from_cardridge_header(data: R, typ: u8, rom_size: u8, ram_size: u8) -> Self {
        let rom_banks = match rom_size {
            0x00..=0x08 => (1 << rom_size) + 1,
            0x52 => 72,
            0x53 => 80,
            0x54 => 96,
            _ => unreachable!(),
        };
        let ram_banks = match ram_size {
            0x00 => {
                // MBC2 has 512 half-bytes or RAM, but it's internal, so ram_banks
                // is technically zero. We don't emulate the hardware layout precisely,
                // so we pretend it has 1 bank of regular RAM.
                if (0x05..=0x06).contains(&typ) {
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
        };
        let has_battery = match typ {
            0x3 => true,
            _ => false,
        };

        Self {
            rom: Rom::new(data, rom_banks),
            ram: Ram::new(ram_banks),
            has_battery,
            banking_mode: BankingMode::Rom,
        }
    }
}

/// Determines whether certain areas of a cartridge
/// are mapped to a ROM bank or to a RAM bank.
#[derive(PartialEq, Eq)]
enum BankingMode {
    /// Maps flexible areas of a cartridge to a ROM bank.
    Rom = 0,
    /// Maps flexible areas of a cartridge to a RAM bank.
    Ram = 1,
}

impl TryFrom<u8> for BankingMode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BankingMode::Rom),
            1 => Ok(BankingMode::Ram),
            _ => Err(format!("unsupported bank addressing value: {}", value)),
        }
    }
}

fn read_at<R: Read + Seek>(data: &mut R, addr: u16) -> io::Result<u8> {
    data.seek(SeekFrom::Start(addr as u64))?;
    let mut buf = [0; 1];
    data.read_exact(&mut buf)?;
    Ok(buf[0])
}
