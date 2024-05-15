mod instructions;

use crate::machine::memory;

pub struct Cpu<'a> {
    registers: Registers,
    memory: &'a mut memory::Memory,
}

impl<'a> Cpu<'a> {
    fn fetch(&mut self) {
        let opcode = self.increment_prog_counter();
    }

    fn increment_prog_counter(&mut self) -> u8 {
        let mem = self.memory.at(self.registers.prog_counter);
        self.registers.prog_counter += 1;
        mem
    }
}

#[derive(Clone, Copy)]
enum Flag {
    Zero = 7,
    Neg = 6,
    HalfCarry = 5,
    Carry = 4,

}

struct Registers {
    /// Accumulator.
    a: u8,
    /// Flags.
    f: u8,
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

impl Registers {
    fn set_flag(&mut self, flag: Flag, val: bool) {
        let bit_cleared = self.f & !(0x1 << (flag as u8));
        self.f = bit_cleared | (val as u8) << (flag as u8);
    }
}
