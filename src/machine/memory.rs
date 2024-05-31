use std::collections::BTreeMap;
use std::ops::RangeInclusive;
use std::{borrow, cmp};

use crate::keypad::Keypad;

pub struct Mmu<'a> {
    work_ram: [u8; (WORK_RAM_END - WORK_RAM_START + 1) as usize],
    echo_ram: [u8; (ECHO_RAM_END - ECHO_RAM_START + 1) as usize],
    devices: Devices<'a>,
    interrupts: Interrupts<'a>,
}

impl<'a> Mmu<'a> {
    pub fn new_gb(keys: &'a Keypad) -> Self {
        let mut ret = Self::default();
        ret.devices.register(0xFF00..=0xFF00, keys);
        ret.interrupts.register(Interrupt::Four, keys);
        ret
    }

    pub fn at(&self, addr: u16) -> u8 {
        *self.work_ram.get(addr as usize).unwrap()
    }

    pub fn at_mut(&mut self, addr: u16) -> &mut u8 {
        self.work_ram.get_mut(addr as usize).unwrap()
    }
}

impl<'a> Default for Mmu<'a> {
    fn default() -> Self {
        Self {
            work_ram: [0; (WORK_RAM_END - WORK_RAM_START + 1) as usize],
            echo_ram: [0; (ECHO_RAM_END - ECHO_RAM_START + 1) as usize],
            devices: Default::default(),
            interrupts: Default::default(),
        }
    }
}

pub trait MemMapped {
    fn read_mem_mapped(&self, idx: usize) -> Option<u8>;

    fn write_mem_mapped(&mut self, data: &[u8]) -> Result<(), ()>;
}

pub trait Peripheral {
    fn has_interrupt(&self) -> bool;
}

#[derive(Clone, Copy)]
pub enum Interrupt {
    Four = 4,
    Three = 3,
    Two = 2,
    One = 1,
    Zero = 0,
}

#[derive(Default)]
struct Devices<'a>(BTreeMap<AddrRange, &'a dyn MemMapped>);

impl<'a> Devices<'a> {
    fn get(&self, addr: u16) -> Option<&&'a dyn MemMapped> {
        self.0.range(..=addr).next_back().and_then(|(range, dev)| {
            if range.contains(addr) {
                return Some(dev);
            }
            None
        })
    }

    fn register(&mut self, range: impl Into<AddrRange>, dev: &'a dyn MemMapped) {
        self.0.insert(range.into(), dev);
    }
}

struct AddrRange(core::ops::RangeInclusive<u16>);

impl AddrRange {
    fn contains(&self, addr: u16) -> bool {
        self.0.contains(&addr)
    }
}

impl PartialEq for AddrRange {
    fn eq(&self, other: &Self) -> bool {
        self.0.start() == other.0.start()
    }
}

impl Eq for AddrRange {}

impl PartialOrd for AddrRange {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AddrRange {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.start().cmp(other.0.start())
    }
}

impl borrow::Borrow<u16> for AddrRange {
    fn borrow(&self) -> &u16 {
        self.0.start()
    }
}

impl From<RangeInclusive<u16>> for AddrRange {
    fn from(value: RangeInclusive<u16>) -> Self {
        Self(value)
    }
}

#[derive(Default)]
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

impl<'a> MemMapped for Interrupts<'a> {
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

    fn write_mem_mapped(&mut self, data: &[u8]) -> Result<(), ()> {
        Ok(())
    }
}

const WORK_RAM_START: u16 = 0xC000;
const WORK_RAM_END: u16 = 0xCFFF;
const WORK_RAM: RangeInclusive<u16> = WORK_RAM_START..=WORK_RAM_END;

const ECHO_RAM_START: u16 = 0xD000;
const ECHO_RAM_END: u16 = 0xDFFF;
const ECHO_RAM: RangeInclusive<u16> = ECHO_RAM_START..=ECHO_RAM_END;
