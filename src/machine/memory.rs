use std::ops::RangeInclusive;

pub struct Mmu<'a> {
    work_ram: [u8; (WORK_RAM_END - WORK_RAM_START + 1) as usize],
    echo_ram: [u8; (ECHO_RAM_END - ECHO_RAM_START + 1) as usize],
    interrupts: Interrupts<'a>,
}

impl<'a> Mmu<'a> {
    pub fn new() -> Self {
        Self {
            work_ram: [0; (WORK_RAM_END - WORK_RAM_START + 1) as usize],
            echo_ram: [0; (ECHO_RAM_END - ECHO_RAM_START + 1) as usize],
            interrupts: Interrupts::new(),
        }
    }

    pub fn register_interrupt(&mut self, idx: Interrupt, per: &'a dyn Peripheral) {
        self.interrupts.register(idx, per);
    }

    pub fn at(&self, addr: u16) -> u8 {
        *self.work_ram.get(addr as usize).unwrap()
    }

    pub fn at_mut(&mut self, addr: u16) -> &mut u8 {
        self.work_ram.get_mut(addr as usize).unwrap()
    }
}

pub trait MemMapRead {
    fn read_mem_mapped(&self, idx: usize) -> Option<u8>;
}

pub trait MemMapWrite {
    fn write_mem_mapped(&mut self, data: &[u8]) -> Result<(), ()>;
}

#[derive(Clone, Copy)]
pub enum Interrupt {
    Four = 4,
    Three = 3,
    Two = 2,
    One = 1,
    Zero = 0,
}

struct Interrupts<'a> {
    peripherals: [Option<&'a dyn Peripheral>; 5],
}

impl<'a> Interrupts<'a> {
    fn new() -> Self {
        Self {
            peripherals: [None; 5],
        }
    }

    fn register(&mut self, idx: Interrupt, per: &'a dyn Peripheral) {
        self.peripherals[idx as usize] = Some(per);
    }
}

impl<'a> MemMapRead for Interrupts<'a> {
    fn read_mem_mapped(&self, idx: usize) -> Option<u8> {
        if idx > 0 {
            return None;
        }
        let mut byte: u8 = 0;
        for (i, per) in self.peripherals.iter().enumerate() {
            match per {
                Some(per) => byte |= (per.has_interrupt() as u8) << i,
                None => continue,
            }
        }
        Some(byte)
    }
}

pub trait Peripheral {
    fn has_interrupt(&self) -> bool;
}

const WORK_RAM_START: u16 = 0xC000;
const WORK_RAM_END: u16 = 0xCFFF;
const WORK_RAM: RangeInclusive<u16> = WORK_RAM_START..=WORK_RAM_END;

const ECHO_RAM_START: u16 = 0xD000;
const ECHO_RAM_END: u16 = 0xDFFF;
const ECHO_RAM: RangeInclusive<u16> = ECHO_RAM_START..=ECHO_RAM_END;
