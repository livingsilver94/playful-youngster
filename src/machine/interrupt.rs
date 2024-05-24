use crate::memory::MemMapRead;

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
    pub fn new() -> Self {
        Self {
            peripherals: [None; 5],
        }
    }

    pub fn register(&mut self, idx: Interrupt, per: &'a dyn Peripheral) {
        self.peripherals[idx as usize] = Some(per);
    }
}

impl<'a> MemMapRead for Interrupts<'a> {
    fn read_as_mem(&self) -> [u8] {
        let mut byte: u8 = 0;
        for (i, per) in self.peripherals.iter().enumerate() {
            match per {
                Some(per) => byte |= (per.has_interrupt() as u8) << i,
                None => continue,
            }
        }
        [byte;1].as_ref()
    }
}

pub trait Peripheral {
    fn has_interrupt(&self) -> bool;
}
