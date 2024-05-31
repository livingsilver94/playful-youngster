use crate::memory::{MemMapped, Peripheral};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Button {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

#[derive(Default)]
pub struct Keypad {
    dpad: KeyRow,
    btns: KeyRow,
    interrupt_raised: bool,
}

impl Keypad {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_pressed(&mut self, button: Button, pressed: bool) {
        let (row, index) = if button <= Button::Down {
            (&mut self.dpad.values, button as usize)
        } else {
            (&mut self.btns.values, button as usize - 4)
        };
        // Oddly, a pressed button has value zero.
        if pressed {
            *row &= !(1 << index);
            self.interrupt_raised = true;
        } else {
            *row |= 1 << index;
        }
    }
}

impl MemMapped for Keypad {
    fn read_mem_mapped(&self, idx: usize) -> Option<u8> {
        if idx != 0 {
            return None;
        }
        if self.dpad.selected {
            return Some(self.dpad.values);
        }
        if self.btns.selected {
            return Some(self.btns.values);
        }
        Some(0xF)
    }

    fn write_mem_mapped(&mut self, data: &[u8]) -> Result<(), ()> {
        self.btns.selected = (data[0] & (1 << 5)) == 0;
        self.btns.selected = (data[0] & (1 << 4)) == 0;
        Ok(())
    }
}

impl Peripheral for Keypad {
    fn has_interrupt(&self) -> bool {
        self.interrupt_raised
    }
}

#[derive(Default)]
struct KeyRow {
    selected: bool,
    values: u8,
}
