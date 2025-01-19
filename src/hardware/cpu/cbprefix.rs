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
        0x00 => rotate(cpu, Left, Reg(B), false),
        0x01 => rotate(cpu, Left, Reg(C), false),
        0x02 => rotate(cpu, Left, Reg(D), false),
        0x03 => rotate(cpu, Left, Reg(E), false),
        0x04 => rotate(cpu, Left, Reg(H), false),
        0x05 => rotate(cpu, Left, Reg(L), false),
        0x06 => rotate(cpu, Left, Addr(HL, hw), false),
        0x07 => rotate(cpu, Left, Reg(A), false),
        0x08 => rotate(cpu, Right, Reg(B), false),
        0x09 => rotate(cpu, Right, Reg(C), false),
        0x0A => rotate(cpu, Right, Reg(D), false),
        0x0B => rotate(cpu, Right, Reg(E), false),
        0x0C => rotate(cpu, Right, Reg(H), false),
        0x0D => rotate(cpu, Right, Reg(L), false),
        0x0E => rotate(cpu, Right, Addr(HL, hw), false),
        0x0F => rotate(cpu, Right, Reg(A), false),
        0x10 => rotate(cpu, Left, Reg(B), true),
        0x11 => rotate(cpu, Left, Reg(C), true),
        0x12 => rotate(cpu, Left, Reg(D), true),
        0x13 => rotate(cpu, Left, Reg(E), true),
        0x14 => rotate(cpu, Left, Reg(H), true),
        0x15 => rotate(cpu, Left, Reg(L), true),
        0x16 => rotate(cpu, Left, Addr(HL, hw), true),
        0x17 => rotate(cpu, Left, Reg(A), true),
        0x18 => rotate(cpu, Right, Reg(B), true),
        0x19 => rotate(cpu, Right, Reg(C), true),
        0x1A => rotate(cpu, Right, Reg(D), true),
        0x1B => rotate(cpu, Right, Reg(E), true),
        0x1C => rotate(cpu, Right, Reg(H), true),
        0x1D => rotate(cpu, Right, Reg(L), true),
        0x1E => rotate(cpu, Right, Addr(HL, hw), true),
        0x1F => rotate(cpu, Right, Reg(A), true),
        _ => unreachable!(),
    }
}

fn rotate(cpu: &mut Cpu, dir: Direction, mut byte: Byte, reuse_carry: bool) -> u8 {
    let mut value = byte.value(cpu);
    let carry = if reuse_carry {
        cpu.regs.flags.carry as u8
    } else {
        0
    };
    match dir {
        Direction::Left => {
            cpu.regs.flags.carry = value & 0b10000000 != 0;
            value = value.rotate_left(1)
        }
        Direction::Right => {
            cpu.regs.flags.carry = value & 0b00000001 != 0;
            value = value.rotate_right(1)
        }
    }
    value = (value & 0b11111110) | carry;
    byte.set_value(cpu, value);

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
