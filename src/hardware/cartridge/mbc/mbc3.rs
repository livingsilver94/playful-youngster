use std::io;

use crate::hardware::cartridge::{BankingMode, Hardware};

pub fn read(hw: &mut Hardware, addr: u16) -> io::Result<u8> {
    match addr {
        0x0000..=0x3FFF => hw.rom.at(addr),
        0x4000..=0x7FFF => hw.rom.at_current_bank(addr - 0x4000),
        0xA000..=0xBFFF => match hw.banking_mode {
            BankingMode::Ram => Ok(hw.ram.read_current_bank(addr - 0xA000)),
            BankingMode::Rtc => Ok(hw.rtc.read_current_register()),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

pub fn write(hw: &mut Hardware, addr: u16, val: u8) {
    match addr {
        0x0000..=0x1FFF => {
            hw.ram.enabled = (val & 0x0F) == 0x0A;
            hw.rtc.enabled = hw.ram.enabled;
        }
        0x2000..=0x3FFF => hw.rom.set_bank(val),
        0x4000..=0x5FFF => {
            if (0x08..=0x0C).contains(&val) {
                hw.banking_mode = BankingMode::Rtc;
                hw.rtc.set_current_register(val - 0x08);
            } else {
                hw.banking_mode = BankingMode::Ram;
                hw.ram.set_current_bank(val);
            }
        }
        0xA000..=0xBFFF => match hw.banking_mode {
            BankingMode::Ram => hw.ram.write_current_bank(addr - 0xA000, val),
            BankingMode::Rtc => hw.rtc.write_current_register(val),
            _ => unreachable!(),
        },
        0x6000..=0x7FFF => hw.rtc.set_latched(val != 0),
        _ => unreachable!(),
    }
}
