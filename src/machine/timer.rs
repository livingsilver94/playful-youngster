use crate::machine::memory::RegisterReadWrite;

use super::memory::Interruptible;

#[derive(Default)]
pub struct Timer {
    /// The master counter. The most significant 8 bits are exposed through a register, so that
    /// divider is also used as a timer that ticks every 256 CPU clock ticks.
    divider: u16,

    /// Counter that +1's according to the current demultiplier.
    /// When it overflows, its value is set to [`Self::modulo`]
    /// and an interrupt is raised.
    counter: u8,
    counter_ticks: u16,

    /// Value at which [`Self::counter`] resets when it overflows.
    modulo: u8,

    /// Frequency at which [`Self::counter`] ticks.
    demultiplier: u16,

    enabled: bool,
    interrupt: bool,
}

impl Timer {
    pub fn step(&mut self, ticks: u16) {
        self.divider = self.divider.wrapping_add(ticks);

        if !self.enabled {
            return;
        }
        self.counter_ticks = self.counter_ticks.wrapping_add(ticks);
        if self.counter_ticks > self.demultiplier {
            let (val, overflow) = self.counter.overflowing_add(1);
            if overflow {
                self.counter = self.modulo;
                self.interrupt = true;
            } else {
                self.counter = val;
            }
            self.counter_ticks = 0;
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        if !enabled {
            self.divider = 0;
            self.counter_ticks = 0;
        }
        self.enabled = enabled;
    }
}

impl RegisterReadWrite for Timer {
    fn read_register(&self, idx: usize) -> u8 {
        match idx {
            0 => (self.divider >> 8) as u8,
            1 => self.counter,
            2 => self.modulo,
            3 => unimplemented!(), // FIXME: Do games read this value at all?
            _ => panic!("timer maps only 4 bytes"),
        }
    }

    fn write_register(&mut self, idx: usize, val: u8) {
        match idx {
            0 => {
                self.divider = 0;
                self.counter_ticks = 0;
            }
            1 => self.counter = val,
            2 => self.modulo = val,
            3 => {
                self.enabled = val & 0b00000100 != 0;
                self.demultiplier = match val & 0b00000011 {
                    0x00 => 256,
                    0x01 => 4,
                    0x10 => 16,
                    0x11 => 64,
                    _ => unreachable!(),
                };
            }
            _ => panic!("timer maps only 4 bytes"),
        }
    }
}

impl Interruptible for Timer {
    fn has_interrupt(&self) -> bool {
        self.interrupt
    }
}
