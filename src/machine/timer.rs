use crate::machine::memory::MemMapped;

use super::memory::Peripheral;

pub struct Timer {
    /// Counter that +1's after 256 clock ticks. Always enabled.
    divider: u8,

    /// Counter that +1's according to the current demultiplier.
    /// When it overflows, its value is set to [`Self::modulo`]
    /// and an interrupt is raised. It can be disabled.
    counter: u8,

    modulo: u8,
    enabled: bool,
    divider_ticks: u32,
    counter_ticks: u32,
    interrupt: bool,
}

impl Timer {
    pub fn step(&mut self, ticks: u32) {
        self.divider_ticks += ticks;
        if self.divider_ticks >= DIVIDER_FRACTION {
            self.divider = self.divider.wrapping_add(1);
            self.divider_ticks = 0;
        }

        if !self.enabled {
            return;
        }
        self.counter_ticks += ticks;

    }
}

impl MemMapped for Timer {
    fn read_mem_mapped(&self, idx: usize) -> Option<u8> {
        if idx > 0 {
            return None;
        }
        Some(0)
    }

    fn write_mem_mapped(&mut self, data: &[u8]) -> Result<(), ()> {
        todo!()
    }
}

impl Peripheral for Timer {
    fn has_interrupt(&self) -> bool {
        self.interrupt
    }
}

const DIVIDER_FRACTION: u32 = 256;
