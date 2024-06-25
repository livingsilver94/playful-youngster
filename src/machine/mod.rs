use cpu::Cpu;
use keypad::Keypad;
use memory::Mmu;
use timer::Timer;

mod apu;
pub mod cpu;
pub mod keypad;
pub mod memory;
mod timer;

#[derive(Default)]
pub struct Hardware {
    key: Keypad,
    timer: Timer,
}

pub struct Emulator<'a> {
    cpu: Cpu<'a>,
}

impl<'a> Emulator<'a> {
    pub fn new_gb(hw: &'a Hardware) -> Self {
        Self {
            cpu: Cpu::new(Mmu::new_gb(&hw.key, &hw.timer)),
        }
    }
}
