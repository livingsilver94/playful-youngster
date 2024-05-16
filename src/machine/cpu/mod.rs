mod instructions;

use crate::machine::memory;

pub struct Cpu<'a> {
    regs: Registers,
    memory: &'a mut memory::Memory,
}

impl<'a> Cpu<'a> {
    fn fetch(&mut self) {
        let opcode = self.increment_prog_counter();
    }

    fn increment_prog_counter(&mut self) -> u8 {
        let mem = self.memory.at(self.regs.prog_counter);
        self.regs.prog_counter += 1;
        mem
    }
}

#[derive(Clone, Copy)]
struct Flags {
    zero: bool,
    neg: bool,
    half_carry: bool,
    carry: bool,
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Self {
            zero: value & 0b10000000 != 0,
            neg: value & 0b01000000 != 0,
            half_carry: value & 0b00100000 != 0,
            carry: value & 0b00010000 != 0,
        }
    }
}

struct Registers {
    /// Accumulator.
    a: u8,
    /// Flags.
    flags: Flags,
    /// B and C general-purpose registers.
    bc: u16,
    /// D and E general-purpose registers.
    de: u16,
    /// H and L general-purpose registers.
    hl: u16,

    /// Points to the next instruction of the program.
    prog_counter: u16,
    /// Points to the top of the stack.
    stack_pointer: u16,
}
