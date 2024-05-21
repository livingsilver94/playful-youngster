use std::ops::RangeInclusive;

pub struct Mmu {
    work_ram: [u8; (WORK_RAM_END - WORK_RAM_START + 1) as usize],
    echo_ram: [u8; (ECHO_RAM_END - ECHO_RAM_START + 1) as usize],
}

impl Mmu {
    pub fn new() -> Self {
        Self {
            work_ram: [0; (WORK_RAM_END - WORK_RAM_START + 1) as usize],
            echo_ram: [0; (ECHO_RAM_END - ECHO_RAM_START + 1) as usize],
        }
    }

    pub fn at(&self, addr: u16) -> u8 {
        *self.work_ram.get(addr as usize).unwrap()
    }

    pub fn at_mut(&mut self, addr: u16) -> &mut u8 {
        self.work_ram.get_mut(addr as usize).unwrap()
    }
}

const WORK_RAM_START: u16 = 0xC000;
const WORK_RAM_END: u16 = 0xCFFF;
const WORK_RAM: RangeInclusive<u16> = WORK_RAM_START..=WORK_RAM_END;

const ECHO_RAM_START: u16 = 0xD000;
const ECHO_RAM_END: u16 = 0xDFFF;
const ECHO_RAM: RangeInclusive<u16> = ECHO_RAM_START..=ECHO_RAM_END;
