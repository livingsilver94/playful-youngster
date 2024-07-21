use std::io::{self, Read, Seek};

use crate::machine::cartridge::{BankingMode, Hardware};

pub fn read<R: Read + Seek>(hw: &mut Hardware<R>, addr: u16) -> io::Result<u8> {
    match addr {
        0x0000..=0x3FFF => hw.rom.at(addr),
        0x4000..=0x7FFF => hw.rom.at_current_bank(addr - 0x4000),
        0xA000..=0xBFFF => Ok(hw.ram.read_current_bank(addr - 0xA000)),
        _ => unreachable!(),
    }
}

pub fn write<R: Read + Seek>(hw: &mut Hardware<R>, addr: u16, val: u8) {
    match addr {
        0x0000..=0x1FFF => hw.ram.enabled = val & 0x0F == 0x0A, // Any value with 0xA in the lower 4 bits enables the ram.
        0x2000..=0x3FFF => hw.rom.set_bank(val),
        0x4000..=0x5FFF => {
            // This value is either the ROM bank selector (upper 2 bits),
            // if at least 64 ROM banks are present, or the RAM bank selector.
            if hw.rom.banks <= 32 && hw.ram.banks <= 1 {
                // This cartridge is too small for bank selection to be meaningful.
                return;
            }
            match hw.banking_mode {
                BankingMode::Rom => hw.rom.set_bank_extended(val),
                BankingMode::Ram => hw.ram.set_bank(val),
            }
        }
        0x6000..=0x7FFF => {
            if hw.rom.banks <= 32 && hw.ram.banks <= 1 {
                // This cartridge is too small for banking mode to be meaningful.
                return;
            }
            let new_mode = BankingMode::try_from(val).unwrap();
            if hw.banking_mode == new_mode {
                return;
            }
            match new_mode {
                BankingMode::Rom => {
                    hw.rom.set_bank_extended(hw.ram.curr_bank);
                    hw.ram.set_bank(0);
                }
                BankingMode::Ram => {
                    hw.ram.set_bank(hw.rom.curr_bank & 0b11000000 >> 6);
                    hw.rom.set_bank_extended(0);
                }
            }
            hw.banking_mode = new_mode;
        }
        0xA000..=0xBFFF => {
            hw.ram.write_current_bank(addr - 0x4000, val);
        }
        _ => unreachable!(),
    }
}