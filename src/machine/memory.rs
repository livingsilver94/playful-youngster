use std::collections::BTreeMap;
use std::ops::RangeInclusive;
use std::{borrow, cmp};

use crate::machine::graphics::gpu;
use crate::machine::keypad::Keypad;
use crate::machine::timer::Timer;

pub struct Mmu<'a> {
    work_ram: [u8; (WORK_RAM_END - WORK_RAM_START + 1) as usize],
    echo_ram: [u8; (ECHO_RAM_END - ECHO_RAM_START + 1) as usize],
    oam_ram: &'a mut [u8; gpu::OAM_RAM_SIZE],
    video_ram: &'a mut [u8; gpu::VRAM_SIZE],
    devices: Devices<'a>,
    interrupts: Interrupts<'a>,
}

impl<'a> Mmu<'a> {
    pub fn new_gb(gpu: &'a mut gpu::Gpu, keys: &'a Keypad, timer: &'a Timer) -> Self {
        let mut mmu = Self {
            work_ram: [0; (WORK_RAM_END - WORK_RAM_START + 1) as usize],
            echo_ram: [0; (ECHO_RAM_END - ECHO_RAM_START + 1) as usize],
            oam_ram: &mut gpu.oam_ram,
            video_ram: &mut gpu.video_ram,
            devices: Default::default(),
            interrupts: Default::default(),
        };

        mmu.devices
            .register(MAPPED_IO_START..=MAPPED_IO_START, keys);
        mmu.devices.register(0xFF04..=0xFF07, timer);

        mmu.interrupts.register(Interrupt::Four, keys);
        mmu.interrupts.register(Interrupt::Two, timer);
        mmu
    }

    pub fn at(&self, addr: u16) -> u8 {
        match addr {
            VIDEO_RAM_START..=VIDEO_RAM_END => self.video_ram[(addr - VIDEO_RAM_START) as usize],
            WORK_RAM_START..=WORK_RAM_END => self.work_ram[(addr - WORK_RAM_START) as usize],
            ECHO_RAM_START..=ECHO_RAM_END => self.echo_ram[(addr - ECHO_RAM_START) as usize],
            OAM_RAM_START..=OAM_RAM_END => self.oam_ram[(addr - OAM_RAM_START) as usize],
            MAPPED_IO_START..=MAPPED_IO_END => {
                let (device, start_addr) = self.devices.get(addr).unwrap();
                device
                    .read_mem_mapped((addr - start_addr) as usize)
                    .unwrap()
            }
            _ => panic!(),
        }
    }

    pub fn at_mut(&mut self, addr: u16) -> &mut u8 {
        self.work_ram.get_mut(addr as usize).unwrap()
    }
}

pub trait MemMapped {
    fn read_mem_mapped(&self, idx: usize) -> Option<u8>;

    fn write_mem_mapped(&mut self, idx: usize, val: u8) -> Result<(), ()>;
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
    fn get(&self, addr: u16) -> Option<(&&'a dyn MemMapped, u16)> {
        self.0.range(..=addr).next_back().and_then(|(range, dev)| {
            if range.contains(addr) {
                return Some((dev, range.start()));
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

    fn start(&self) -> u16 {
        *self.0.start()
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

    fn write_mem_mapped(&mut self, idx: usize, val: u8) -> Result<(), ()> {
        Ok(())
    }
}

const VIDEO_RAM_START: u16 = 0x8000;
const VIDEO_RAM_END: u16 = 0x9FFF;

const WORK_RAM_START: u16 = 0xC000;
const WORK_RAM_END: u16 = 0xCFFF;

const ECHO_RAM_START: u16 = 0xD000;
const ECHO_RAM_END: u16 = 0xDFFF;

const OAM_RAM_START: u16 = 0xFE00;
const OAM_RAM_END: u16 = 0xFE9F;

const MAPPED_IO_START: u16 = 0xFF00;
const MAPPED_IO_END: u16 = 0xFF7F;
