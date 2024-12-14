mod header;
mod mbc;

use std::io::{self, Read, Seek, SeekFrom};

use mbc::Mbc;

pub struct Cartridge<R: Read + Seek> {
    hw: Hardware<R>,
    /// Whether or not the cartridge sports a battery.
    /// The battery is used to retain values in RAM and/or
    /// power the embedded RTC. As far as the emulation is concerned,
    /// having a battery means having to store the cartridge state
    /// in a file.
    has_battery: bool,

    mbc: Mbc,
}

impl<R: Read + Seek> Cartridge<R> {
    /// Builds a cartridge hardware emulator according to a header contained in the cartridge itself.
    pub fn new_from_header(mut data: R) -> io::Result<Self> {
        let cartridge_type = header::cartridge_type(&mut data)?;
        let rom_banks = header::rom_banks(&mut data)?;
        let ram_banks = header::ram_banks(&mut data, cartridge_type)?;
        Ok(Self {
            hw: Hardware::new(data, rom_banks, ram_banks),
            has_battery: cartridge_type.has_battery(),
            mbc: cartridge_type.mbc(),
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
    pub rom: Rom<R>,
    pub ram: Ram,
    pub banking_mode: BankingMode,
    pub rtc: Rtc,
}

impl<R: Read + Seek> Hardware<R> {
    fn new(data: R, rom_banks: u8, ram_banks: u8) -> Self {
        Self {
            rom: Rom::new(data, rom_banks),
            ram: Ram::new(ram_banks),
            banking_mode: BankingMode::Rom,
            rtc: Rtc::new(),
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

struct Rtc {
    latched: bool,
    registers: RtcRegisters,
}

impl Rtc {
    fn new() -> Self {
        Self {
            latched: false,
            registers: Default::default(),
        }
    }

    /// Sets whether the RTC value is latched to a certain register.
    /// The RTC value is only latched when it is first "unlatched",
    /// i.e. it is required to call [`Self::set_latched`] passing
    /// `false` and then again passing `true`.
    fn set_latched(&mut self, latched: bool) {
        if !self.latched && latched {
            todo!("Save latched value");
        }
        self.latched = latched;
    }
}

#[derive(Default)]
struct RtcRegisters {
    seconds: u8,
    minutes: u8,
    hours: u8,
    /// Lower bits of the day counter.
    /// The most significant bit is contained in [`Self::bools`].
    days_lower: u8,
    bools: RtcBoolRegisters,
}

#[derive(Default)]
struct RtcBoolRegisters(bitmaps::Bitmap<8>);

impl RtcBoolRegisters {
    fn days_upper(&self) -> bool {
        self.0.get(0)
    }

    fn enabled(&self) -> bool {
        self.0.get(6)
    }

    fn days_overflowed(&self) -> bool {
        self.0.get(7)
    }

    fn set_days_upper(&mut self, val: bool) {
        self.0.set(0, val);
    }

    fn set_enabled(&mut self, val: bool) {
        self.0.set(6, val);
    }

    fn set_days_overflowed(&mut self, val: bool) {
        self.0.set(7, val);
    }
}

fn read_at<R: Read + Seek>(data: &mut R, addr: u16) -> io::Result<u8> {
    data.seek(SeekFrom::Start(addr as u64))?;
    let mut buf = [0; 1];
    data.read_exact(&mut buf)?;
    Ok(buf[0])
}
