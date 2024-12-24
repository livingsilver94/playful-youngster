mod apu;
mod cartridge;
mod cpu;
mod graphics;
mod keypad;
mod timer;

pub use crate::hardware::cartridge::Cartridge;
pub use crate::hardware::cpu::Cpu;
pub use crate::hardware::keypad::Keypad;

use crate::hardware::graphics::Gpu;
use crate::hardware::timer::Timer;

pub struct Hardware {
    work_ram: [u8; (WORK_RAM_END - WORK_RAM_START + 1) as usize],
    echo_ram: [u8; (ECHO_RAM_END - ECHO_RAM_START + 1) as usize],

    gpu: Gpu,
    keypad: Keypad,
    timer: Timer,
}

impl Hardware {
    pub fn new() -> Self {
        Self {
            work_ram: [0; (WORK_RAM_END - WORK_RAM_START + 1) as usize],
            echo_ram: [0; (ECHO_RAM_END - ECHO_RAM_START + 1) as usize],
            gpu: Gpu::new(),
            keypad: Keypad::new(),
            timer: Default::default(),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            VIDEO_RAM_START..=VIDEO_RAM_END => self.gpu.read_vram(addr),
            WORK_RAM_START..=WORK_RAM_END => self.work_ram[(addr - WORK_RAM_START) as usize],
            ECHO_RAM_START..=ECHO_RAM_END => self.echo_ram[(addr - ECHO_RAM_START) as usize],
            OAM_RAM_START..=OAM_RAM_END => self.gpu.read_oam(addr),

            MAPPED_KEYPAD_START..=MAPPED_KEYPAD_END => self
                .keypad
                .read_register((addr - MAPPED_KEYPAD_START) as usize),
            MAPPED_TIMER_START..=MAPPED_TIMER_END => self
                .timer
                .read_register((addr - MAPPED_TIMER_START) as usize),

            INTERRUPTS_START..=INTERRUPTS_END => self.read_interrupts(),
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            MAPPED_DMA => self.dma_write(val),
            _ => todo!(),
        }
        todo!()
    }

    fn read_interrupts(&self) -> u8 {
        let mut byte: u8 = 0;
        let ints: [Option<&dyn Interruptible>; 5] =
            [None, None, Some(&self.timer), None, Some(&self.keypad)];
        for (i, per) in ints.iter().enumerate() {
            match per {
                Some(per) => byte |= (per.has_interrupt() as u8) << i,
                None => continue,
            }
        }
        byte
    }

    // TODO: return 160 clock ticks.
    fn dma_write(&mut self, addr: u8) {
        // DMA copies 0xA0 bytes starting from address addr, but multiplied by 256.
        let read_base = (addr as u16) << 8;
        const WRITE_BASE: u16 = 0xFE00;
        for i in 0..0xA0 {
            self.write(WRITE_BASE + i, self.read(read_base + i))
        }
    }
}

pub trait Interruptible {
    fn has_interrupt(&self) -> bool;
}

const VIDEO_RAM_START: u16 = 0x8000;
const VIDEO_RAM_END: u16 = 0x9FFF;

const WORK_RAM_START: u16 = 0xC000;
const WORK_RAM_END: u16 = 0xCFFF;

const ECHO_RAM_START: u16 = 0xD000;
const ECHO_RAM_END: u16 = 0xDFFF;

const OAM_RAM_START: u16 = 0xFE00;
const OAM_RAM_END: u16 = 0xFE9F;

const MAPPED_KEYPAD_START: u16 = 0xFF00;
const MAPPED_KEYPAD_END: u16 = 0xFF00;

const MAPPED_TIMER_START: u16 = 0xFF04;
const MAPPED_TIMER_END: u16 = 0xFF07;
const MAPPED_DMA: u16 = 0xFF46;

const INTERRUPTS_START: u16 = 0xFFFF;
const INTERRUPTS_END: u16 = 0xFFFF;
