mod apu;
mod cartridge;
mod cpu;
mod gpu;
mod keypad;
mod memory;
mod timer;

use std::io::{Read, Seek};

use cartridge::Cartridge;
use cpu::Cpu;
use gpu::Gpu;
use keypad::Keypad;
use memory::Mmu;
use timer::Timer;

pub struct Hardware<R: Read + Seek> {
    gpu: Gpu,
    keypad: Keypad,
    timer: Timer,
    cartridge: Option<Cartridge<R>>,
}

impl<R: Read + Seek> Hardware<R> {
    pub fn new_gb() -> Self {
        Self {
            gpu: Gpu::new_gb(),
            keypad: Keypad::new(),
            timer: Timer::default(),
            cartridge: None,
        }
    }

    pub fn insert_cartridge(&mut self, cartridge: Cartridge<R>) {
        self.cartridge = Some(cartridge);
    }
}

pub struct Emulator<'a> {
    cpu: Cpu<'a>,
}

impl<'a> Emulator<'a> {
    pub fn new_gb<R: Read + Seek>(hw: &'a mut Hardware<R>) -> Self {
        let mmu = Mmu::new_gb(&mut hw.gpu, &mut hw.keypad, &mut hw.timer);
        Self { cpu: Cpu::new(mmu) }
    }
}
