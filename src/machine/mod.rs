mod apu;
mod cpu;
mod gpu;
mod keypad;
mod memory;
mod timer;

use cpu::Cpu;
use gpu::Gpu;
use keypad::Keypad;
use memory::Mmu;
use timer::Timer;

pub struct Hardware {
    gpu: Gpu,
    keypad: Keypad,
    timer: Timer,
}

impl Hardware {
    pub fn new_gb() -> Self {
        Self {
            gpu: Gpu::new_gb(),
            keypad: Keypad::new(),
            timer: Timer::default(),
        }
    }
}

pub struct Emulator<'a> {
    cpu: Cpu<'a>,
}

impl<'a> Emulator<'a> {
    pub fn new_gb(hw: &'a mut Hardware) -> Self {
        let mmu = Mmu::new_gb(&mut hw.gpu, &mut hw.keypad, &mut hw.timer);
        Self { cpu: Cpu::new(mmu) }
    }
}
