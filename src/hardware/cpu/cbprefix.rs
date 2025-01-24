use crate::hardware::{
    cpu::{instructions::Direction, Register16, Register8},
    Cpu, Hardware,
};

pub fn execute(cpu: &mut Cpu, hw: &mut Hardware, prefix: u8) -> u8 {
    use Byte::*;
    use Direction::*;
    use Register16::*;
    use Register8::*;

    match prefix {
        0x00 => rotate_carry(cpu, Reg(B), Left),
        0x01 => rotate_carry(cpu, Reg(C), Left),
        0x02 => rotate_carry(cpu, Reg(D), Left),
        0x03 => rotate_carry(cpu, Reg(E), Left),
        0x04 => rotate_carry(cpu, Reg(H), Left),
        0x05 => rotate_carry(cpu, Reg(L), Left),
        0x06 => rotate_carry(cpu, Addr(HL, hw), Left),
        0x07 => rotate_carry(cpu, Reg(A), Left),
        0x08 => rotate_carry(cpu, Reg(B), Right),
        0x09 => rotate_carry(cpu, Reg(C), Right),
        0x0A => rotate_carry(cpu, Reg(D), Right),
        0x0B => rotate_carry(cpu, Reg(E), Right),
        0x0C => rotate_carry(cpu, Reg(H), Right),
        0x0D => rotate_carry(cpu, Reg(L), Right),
        0x0E => rotate_carry(cpu, Addr(HL, hw), Right),
        0x0F => rotate_carry(cpu, Reg(A), Right),
        0x10 => rotate(cpu, Reg(B), Left),
        0x11 => rotate(cpu, Reg(C), Left),
        0x12 => rotate(cpu, Reg(D), Left),
        0x13 => rotate(cpu, Reg(E), Left),
        0x14 => rotate(cpu, Reg(H), Left),
        0x15 => rotate(cpu, Reg(L), Left),
        0x16 => rotate(cpu, Addr(HL, hw), Left),
        0x17 => rotate(cpu, Reg(A), Left),
        0x18 => rotate(cpu, Reg(B), Right),
        0x19 => rotate(cpu, Reg(C), Right),
        0x1A => rotate(cpu, Reg(D), Right),
        0x1B => rotate(cpu, Reg(E), Right),
        0x1C => rotate(cpu, Reg(H), Right),
        0x1D => rotate(cpu, Reg(L), Right),
        0x1E => rotate(cpu, Addr(HL, hw), Right),
        0x1F => rotate(cpu, Reg(A), Right),
        0x20 => shift_left(cpu, Reg(B)),
        0x21 => shift_left(cpu, Reg(C)),
        0x22 => shift_left(cpu, Reg(D)),
        0x23 => shift_left(cpu, Reg(E)),
        0x24 => shift_left(cpu, Reg(H)),
        0x25 => shift_left(cpu, Reg(L)),
        0x26 => shift_left(cpu, Addr(HL, hw)),
        0x27 => shift_left(cpu, Reg(A)),
        0x28 => shift_right(cpu, Reg(B), true),
        0x29 => shift_right(cpu, Reg(C), true),
        0x2A => shift_right(cpu, Reg(D), true),
        0x2B => shift_right(cpu, Reg(E), true),
        0x2C => shift_right(cpu, Reg(H), true),
        0x2D => shift_right(cpu, Reg(L), true),
        0x2E => shift_right(cpu, Addr(HL, hw), true),
        0x2F => shift_right(cpu, Reg(A), true),
        0x30 => swap(cpu, Reg(B)),
        0x31 => swap(cpu, Reg(C)),
        0x32 => swap(cpu, Reg(D)),
        0x33 => swap(cpu, Reg(E)),
        0x34 => swap(cpu, Reg(H)),
        0x35 => swap(cpu, Reg(L)),
        0x36 => swap(cpu, Addr(HL, hw)),
        0x37 => swap(cpu, Reg(A)),
        0x38 => shift_right(cpu, Reg(B), false),
        0x39 => shift_right(cpu, Reg(C), false),
        0x3A => shift_right(cpu, Reg(D), false),
        0x3B => shift_right(cpu, Reg(E), false),
        0x3C => shift_right(cpu, Reg(H), false),
        0x3D => shift_right(cpu, Reg(L), false),
        0x3E => shift_right(cpu, Addr(HL, hw), false),
        0x3F => shift_right(cpu, Reg(A), false),
        0x40 => bit(cpu, Reg(B), 0),
        0x41 => bit(cpu, Reg(C), 0),
        0x42 => bit(cpu, Reg(D), 0),
        0x43 => bit(cpu, Reg(E), 0),
        0x44 => bit(cpu, Reg(H), 0),
        0x45 => bit(cpu, Reg(L), 0),
        0x46 => bit(cpu, Addr(HL, hw), 0),
        0x47 => bit(cpu, Reg(A), 0),
        0x48 => bit(cpu, Reg(B), 1),
        0x49 => bit(cpu, Reg(C), 1),
        0x4A => bit(cpu, Reg(D), 1),
        0x4B => bit(cpu, Reg(E), 1),
        0x4C => bit(cpu, Reg(H), 1),
        0x4D => bit(cpu, Reg(L), 1),
        0x4E => bit(cpu, Addr(HL, hw), 1),
        0x4F => bit(cpu, Reg(A), 1),
        0x50 => bit(cpu, Reg(B), 2),
        0x51 => bit(cpu, Reg(C), 2),
        0x52 => bit(cpu, Reg(D), 2),
        0x53 => bit(cpu, Reg(E), 2),
        0x54 => bit(cpu, Reg(H), 2),
        0x55 => bit(cpu, Reg(L), 2),
        0x56 => bit(cpu, Addr(HL, hw), 2),
        0x57 => bit(cpu, Reg(A), 2),
        0x58 => bit(cpu, Reg(B), 3),
        0x59 => bit(cpu, Reg(C), 3),
        0x5A => bit(cpu, Reg(D), 3),
        0x5B => bit(cpu, Reg(E), 3),
        0x5C => bit(cpu, Reg(H), 3),
        0x5D => bit(cpu, Reg(L), 3),
        0x5E => bit(cpu, Addr(HL, hw), 3),
        0x5F => bit(cpu, Reg(A), 3),
        0x60 => bit(cpu, Reg(B), 4),
        0x61 => bit(cpu, Reg(C), 4),
        0x62 => bit(cpu, Reg(D), 4),
        0x63 => bit(cpu, Reg(E), 4),
        0x64 => bit(cpu, Reg(H), 4),
        0x65 => bit(cpu, Reg(L), 4),
        0x66 => bit(cpu, Addr(HL, hw), 4),
        0x67 => bit(cpu, Reg(A), 4),
        0x68 => bit(cpu, Reg(B), 5),
        0x69 => bit(cpu, Reg(C), 5),
        0x6A => bit(cpu, Reg(D), 5),
        0x6B => bit(cpu, Reg(E), 5),
        0x6C => bit(cpu, Reg(H), 5),
        0x6D => bit(cpu, Reg(L), 5),
        0x6E => bit(cpu, Addr(HL, hw), 5),
        0x6F => bit(cpu, Reg(A), 5),
        0x70 => bit(cpu, Reg(B), 6),
        0x71 => bit(cpu, Reg(C), 6),
        0x72 => bit(cpu, Reg(D), 6),
        0x73 => bit(cpu, Reg(E), 6),
        0x74 => bit(cpu, Reg(H), 6),
        0x75 => bit(cpu, Reg(L), 6),
        0x76 => bit(cpu, Addr(HL, hw), 6),
        0x77 => bit(cpu, Reg(A), 6),
        0x78 => bit(cpu, Reg(B), 7),
        0x79 => bit(cpu, Reg(C), 7),
        0x7A => bit(cpu, Reg(D), 7),
        0x7B => bit(cpu, Reg(E), 7),
        0x7C => bit(cpu, Reg(H), 7),
        0x7D => bit(cpu, Reg(L), 7),
        0x7E => bit(cpu, Addr(HL, hw), 7),
        0x7F => bit(cpu, Reg(A), 7),
        0x80 => reset(cpu, Reg(B), 0),
        0x81 => reset(cpu, Reg(C), 0),
        0x82 => reset(cpu, Reg(D), 0),
        0x83 => reset(cpu, Reg(E), 0),
        0x84 => reset(cpu, Reg(H), 0),
        0x85 => reset(cpu, Reg(L), 0),
        0x86 => reset(cpu, Addr(HL, hw), 0),
        0x87 => reset(cpu, Reg(A), 0),
        0x88 => reset(cpu, Reg(B), 1),
        0x89 => reset(cpu, Reg(C), 1),
        0x8A => reset(cpu, Reg(D), 1),
        0x8B => reset(cpu, Reg(E), 1),
        0x8C => reset(cpu, Reg(H), 1),
        0x8D => reset(cpu, Reg(L), 1),
        0x8E => reset(cpu, Addr(HL, hw), 1),
        0x8F => reset(cpu, Reg(A), 1),
        0x90 => reset(cpu, Reg(B), 2),
        0x91 => reset(cpu, Reg(C), 2),
        0x92 => reset(cpu, Reg(D), 2),
        0x93 => reset(cpu, Reg(E), 2),
        0x94 => reset(cpu, Reg(H), 2),
        0x95 => reset(cpu, Reg(L), 2),
        0x96 => reset(cpu, Addr(HL, hw), 2),
        0x97 => reset(cpu, Reg(A), 2),
        0x98 => reset(cpu, Reg(B), 3),
        0x99 => reset(cpu, Reg(C), 3),
        0x9A => reset(cpu, Reg(D), 3),
        0x9B => reset(cpu, Reg(E), 3),
        0x9C => reset(cpu, Reg(H), 3),
        0x9D => reset(cpu, Reg(L), 3),
        0x9E => reset(cpu, Addr(HL, hw), 3),
        0x9F => reset(cpu, Reg(A), 3),
        0xA0 => reset(cpu, Reg(B), 4),
        0xA1 => reset(cpu, Reg(C), 4),
        0xA2 => reset(cpu, Reg(D), 4),
        0xA3 => reset(cpu, Reg(E), 4),
        0xA4 => reset(cpu, Reg(H), 4),
        0xA5 => reset(cpu, Reg(L), 4),
        0xA6 => reset(cpu, Addr(HL, hw), 4),
        0xA7 => reset(cpu, Reg(A), 4),
        0xA8 => reset(cpu, Reg(B), 5),
        0xA9 => reset(cpu, Reg(C), 5),
        0xAA => reset(cpu, Reg(D), 5),
        0xAB => reset(cpu, Reg(E), 5),
        0xAC => reset(cpu, Reg(H), 5),
        0xAD => reset(cpu, Reg(L), 5),
        0xAE => reset(cpu, Addr(HL, hw), 5),
        0xAF => reset(cpu, Reg(A), 5),
        0xB0 => reset(cpu, Reg(B), 6),
        0xB1 => reset(cpu, Reg(C), 6),
        0xB2 => reset(cpu, Reg(D), 6),
        0xB3 => reset(cpu, Reg(E), 6),
        0xB4 => reset(cpu, Reg(H), 6),
        0xB5 => reset(cpu, Reg(L), 6),
        0xB6 => reset(cpu, Addr(HL, hw), 6),
        0xB7 => reset(cpu, Reg(A), 6),
        0xB8 => reset(cpu, Reg(B), 7),
        0xB9 => reset(cpu, Reg(C), 7),
        0xBA => reset(cpu, Reg(D), 7),
        0xBB => reset(cpu, Reg(E), 7),
        0xBC => reset(cpu, Reg(H), 7),
        0xBD => reset(cpu, Reg(L), 7),
        0xBE => reset(cpu, Addr(HL, hw), 7),
        0xBF => reset(cpu, Reg(A), 7),
        0xC0 => set(cpu, Reg(B), 0),
        0xC1 => set(cpu, Reg(C), 0),
        0xC2 => set(cpu, Reg(D), 0),
        0xC3 => set(cpu, Reg(E), 0),
        0xC4 => set(cpu, Reg(H), 0),
        0xC5 => set(cpu, Reg(L), 0),
        0xC6 => set(cpu, Addr(HL, hw), 0),
        0xC7 => set(cpu, Reg(A), 0),
        0xC8 => set(cpu, Reg(B), 1),
        0xC9 => set(cpu, Reg(C), 1),
        0xCA => set(cpu, Reg(D), 1),
        0xCB => set(cpu, Reg(E), 1),
        0xCC => set(cpu, Reg(H), 1),
        0xCD => set(cpu, Reg(L), 1),
        0xCE => set(cpu, Addr(HL, hw), 1),
        0xCF => set(cpu, Reg(A), 1),
        0xD0 => set(cpu, Reg(B), 2),
        0xD1 => set(cpu, Reg(C), 2),
        0xD2 => set(cpu, Reg(D), 2),
        0xD3 => set(cpu, Reg(E), 2),
        0xD4 => set(cpu, Reg(H), 2),
        0xD5 => set(cpu, Reg(L), 2),
        0xD6 => set(cpu, Addr(HL, hw), 2),
        0xD7 => set(cpu, Reg(A), 2),
        0xD8 => set(cpu, Reg(B), 3),
        0xD9 => set(cpu, Reg(C), 3),
        0xDA => set(cpu, Reg(D), 3),
        0xDB => set(cpu, Reg(E), 3),
        0xDC => set(cpu, Reg(H), 3),
        0xDD => set(cpu, Reg(L), 3),
        0xDE => set(cpu, Addr(HL, hw), 3),
        0xDF => set(cpu, Reg(A), 3),
        0xE0 => set(cpu, Reg(B), 4),
        0xE1 => set(cpu, Reg(C), 4),
        0xE2 => set(cpu, Reg(D), 4),
        0xE3 => set(cpu, Reg(E), 4),
        0xE4 => set(cpu, Reg(H), 4),
        0xE5 => set(cpu, Reg(L), 4),
        0xE6 => set(cpu, Addr(HL, hw), 4),
        0xE7 => set(cpu, Reg(A), 4),
        0xE8 => set(cpu, Reg(B), 5),
        0xE9 => set(cpu, Reg(C), 5),
        0xEA => set(cpu, Reg(D), 5),
        0xEB => set(cpu, Reg(E), 5),
        0xEC => set(cpu, Reg(H), 5),
        0xED => set(cpu, Reg(L), 5),
        0xEE => set(cpu, Addr(HL, hw), 5),
        0xEF => set(cpu, Reg(A), 5),
        0xF0 => set(cpu, Reg(B), 6),
        0xF1 => set(cpu, Reg(C), 6),
        0xF2 => set(cpu, Reg(D), 6),
        0xF3 => set(cpu, Reg(E), 6),
        0xF4 => set(cpu, Reg(H), 6),
        0xF5 => set(cpu, Reg(L), 6),
        0xF6 => set(cpu, Addr(HL, hw), 6),
        0xF7 => set(cpu, Reg(A), 6),
        0xF8 => set(cpu, Reg(B), 7),
        0xF9 => set(cpu, Reg(C), 7),
        0xFA => set(cpu, Reg(D), 7),
        0xFB => set(cpu, Reg(E), 7),
        0xFC => set(cpu, Reg(H), 7),
        0xFD => set(cpu, Reg(L), 7),
        0xFE => set(cpu, Addr(HL, hw), 7),
        0xFF => set(cpu, Reg(A), 7),
    }
}

fn rotate_carry(cpu: &mut Cpu, mut byte: Byte, dir: Direction) -> u8 {
    let mut value = byte.value(cpu);
    match dir {
        Direction::Left => {
            let old_carry = cpu.regs.flags.carry as u8;
            cpu.regs.flags.carry = value & 0b10000000 != 0;
            value = (byte.value(cpu).rotate_left(1) & 0b11111110) | old_carry;
        }
        Direction::Right => {
            let old_carry = (cpu.regs.flags.carry as u8) << 7;
            cpu.regs.flags.carry = value & 0b00000001 != 0;
            value = (byte.value(cpu).rotate_right(1) & 0b01111111) | old_carry;
        }
    }
    byte.set_value(cpu, value);
    cpu.regs.flags.zero = value == 0;
    cpu.regs.flags.neg = false;
    cpu.regs.flags.half_carry = false;

    if matches!(byte, Byte::Addr(_, _)) {
        16
    } else {
        8
    }
}

fn rotate(cpu: &mut Cpu, mut byte: Byte, dir: Direction) -> u8 {
    let mut value = byte.value(cpu);
    match dir {
        Direction::Left => {
            cpu.regs.flags.carry = value & 0b10000000 != 0;
            value = byte.value(cpu).rotate_left(1);
        }
        Direction::Right => {
            cpu.regs.flags.carry = value & 0b00000001 != 0;
            value = byte.value(cpu).rotate_right(1);
        }
    }
    byte.set_value(cpu, value);
    cpu.regs.flags.zero = value == 0;
    cpu.regs.flags.neg = false;
    cpu.regs.flags.half_carry = false;

    if matches!(byte, Byte::Addr(_, _)) {
        16
    } else {
        8
    }
}

fn shift_left(cpu: &mut Cpu, mut byte: Byte) -> u8 {
    let mut value = byte.value(cpu);
    cpu.regs.flags.carry = value & 0b10000000 != 0;
    value = ((byte.value(cpu) as i8) << 1) as u8; // Shift left is always arithmetic.
    byte.set_value(cpu, value);
    cpu.regs.flags.zero = value == 0;
    cpu.regs.flags.neg = false;
    cpu.regs.flags.half_carry = false;

    if matches!(byte, Byte::Addr(_, _)) {
        16
    } else {
        8
    }
}

fn shift_right(cpu: &mut Cpu, mut byte: Byte, signed: bool) -> u8 {
    let mut value = byte.value(cpu);
    cpu.regs.flags.carry = value & 0b00000001 != 0;
    value = if signed {
        ((byte.value(cpu) as i8) >> 1) as u8 // Arithmetic (aka signed).
    } else {
        value >> 1 // Logical.
    };
    byte.set_value(cpu, value);
    cpu.regs.flags.zero = value == 0;
    cpu.regs.flags.neg = false;
    cpu.regs.flags.half_carry = false;

    if matches!(byte, Byte::Addr(_, _)) {
        16
    } else {
        8
    }
}

fn swap(cpu: &mut Cpu, mut byte: Byte) -> u8 {
    let mut value = byte.value(cpu);
    value = value.rotate_right(4);
    byte.set_value(cpu, value);

    if matches!(byte, Byte::Addr(_, _)) {
        16
    } else {
        8
    }
}

fn bit(cpu: &mut Cpu, byte: Byte, index: u8) -> u8 {
    cpu.regs.flags.zero = (byte.value(cpu) & (1 << index)) == 0;
    cpu.regs.flags.neg = false;
    cpu.regs.flags.half_carry = true;

    if matches!(byte, Byte::Addr(_, _)) {
        12
    } else {
        8
    }
}

fn reset(cpu: &mut Cpu, mut byte: Byte, index: u8) -> u8 {
    let value = byte.value(cpu);
    byte.set_value(cpu, value & !(1 << index));

    if matches!(byte, Byte::Addr(_, _)) {
        16
    } else {
        8
    }
}

fn set(cpu: &mut Cpu, mut byte: Byte, index: u8) -> u8 {
    let value = byte.value(cpu);
    byte.set_value(cpu, value | (1 << index));

    if matches!(byte, Byte::Addr(_, _)) {
        16
    } else {
        8
    }
}

enum Byte<'a> {
    Reg(Register8),
    Addr(Register16, &'a mut Hardware),
}

impl Byte<'_> {
    fn value(&self, cpu: &Cpu) -> u8 {
        match self {
            Self::Reg(reg) => cpu.regs[*reg],
            Self::Addr(reg, hw) => hw.read(cpu.regs.combined(*reg)),
        }
    }

    fn set_value(&mut self, cpu: &mut Cpu, value: u8) {
        match self {
            Self::Reg(reg) => cpu.regs[*reg] = value,
            Self::Addr(reg, hw) => hw.write(cpu.regs.combined(*reg), value),
        }
    }
}
