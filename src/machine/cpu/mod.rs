mod instructions;

use std::ops;

use crate::machine::memory;

pub struct Cpu<'a> {
    regs: Registers,
    memory: &'a mut memory::Memory,

    interrupt_enabled: bool,
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
            zero: value & 0b1000_0000 != 0,
            neg: value & 0b0100_0000 != 0,
            half_carry: value & 0b0010_0000 != 0,
            carry: value & 0b0001_0000 != 0,
        }
    }
}

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    flags: Flags,
    /// Points to the next instruction of the program.
    prog_counter: u16,
    /// Points to the top of the stack.
    stack_pointer: u16,
}

#[derive(Clone, Copy)]
enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone, Copy)]
enum Register16 {
    BC,
    DE,
    HL,
}

impl ops::Index<Register8> for Registers {
    type Output = u8;

    fn index(&self, index: Register8) -> &Self::Output {
        match index {
            Register8::A => &self.a,
            Register8::B => &self.b,
            Register8::C => &self.c,
            Register8::D => &self.d,
            Register8::E => &self.e,
            Register8::H => &self.h,
            Register8::L => &self.l,
        }
    }
}

impl ops::IndexMut<Register8> for Registers {
    fn index_mut(&mut self, index: Register8) -> &mut Self::Output {
        match index {
            Register8::A => &mut self.a,
            Register8::B => &mut self.b,
            Register8::C => &mut self.c,
            Register8::D => &mut self.d,
            Register8::E => &mut self.e,
            Register8::H => &mut self.h,
            Register8::L => &mut self.l,
        }
    }
}

impl Registers {
    fn combined(&self, reg: Register16) -> u16 {
        u16::from_be_bytes(match reg {
            Register16::BC => [self.b, self.c],
            Register16::DE => [self.d, self.e],
            Register16::HL => [self.h, self.l],
        })
    }

    fn set_combined(&mut self, reg: Register16, val: u16) {
        let bytes = val.to_be_bytes();
        let (reg_high, reg_low) = match reg {
            Register16::BC => (&mut self.b, &mut self.c),
            Register16::DE => (&mut self.d, &mut self.e),
            Register16::HL => (&mut self.h, &mut self.l),
        };
        *reg_high = bytes[0];
        *reg_low = bytes[1];
    }
}
