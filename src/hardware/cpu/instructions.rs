use crate::hardware::cpu::*;

pub fn execute(cpu: &mut Cpu, hw: &mut Hardware, opcode: u8) -> u8 {
    use {Operand::*, Register16::*, Register8::*, Sign::*};
    match opcode {
        0x00 => nop(cpu),
        0x01 => ld_register16_immediate(cpu, hw, BC),
        0x02 => ld_addr_from_register8(cpu, hw, BC, A),
        0x03 => inc_register16(cpu, BC, 1),
        0x04 => inc_register8(cpu, B, 1),
        0x05 => inc_register8(cpu, B, -1),
        0x06 => ld_register8_immediate(cpu, hw, B),
        0x07 => rotate_circular_a(cpu, Direction::Left),
        0x08 => ld_from_stack_pointer_immediate(cpu, hw),
        0x09 => add_register16(cpu, HL, BC),
        0x0A => ld_register8_from_addr(cpu, hw, A, BC),
        0x0B => inc_register16(cpu, BC, -1),
        0x0C => inc_register8(cpu, C, 1),
        0x0D => inc_register8(cpu, C, -1),
        0x0E => ld_register8_immediate(cpu, hw, C),
        0x0F => rotate_circular_a(cpu, Direction::Right),
        0x10 => stop(cpu, hw),
        0x11 => ld_register16_immediate(cpu, hw, DE),
        0x12 => ld_addr_from_register8(cpu, hw, DE, A),
        0x13 => inc_register16(cpu, DE, 1),
        0x14 => inc_register8(cpu, D, 1),
        0x15 => inc_register8(cpu, D, -1),
        0x16 => ld_register8_immediate(cpu, hw, D),
        0x17 => rotate_a(cpu, Direction::Left),
        0x18 => jump_relative(cpu, hw, true),
        0x19 => add_register16(cpu, HL, DE),
        0x1A => ld_register8_from_addr(cpu, hw, A, DE),
        0x1B => inc_register16(cpu, DE, -1),
        0x1C => inc_register8(cpu, E, 1),
        0x1D => inc_register8(cpu, E, -1),
        0x1E => ld_register8_immediate(cpu, hw, E),
        0x1F => rotate_a(cpu, Direction::Right),
        0x20 => jump_relative(cpu, hw, !cpu.regs.flags.zero),
        0x21 => ld_register16_immediate(cpu, hw, HL),
        0x22 => ld_addr_from_a_increment(cpu, hw, 1),
        0x23 => inc_register16(cpu, HL, 1),
        0x24 => inc_register8(cpu, H, 1),
        0x25 => inc_register8(cpu, H, -1),
        0x26 => ld_register8_immediate(cpu, hw, H),
        0x27 => daa(cpu),
        0x28 => jump_relative(cpu, hw, cpu.regs.flags.zero),
        0x29 => add_register16(cpu, HL, HL),
        0x2A => ld_a_from_addr_increment(cpu, hw, 1),
        0x2B => inc_register16(cpu, HL, -1),
        0x2C => inc_register8(cpu, L, 1),
        0x2D => inc_register8(cpu, L, -1),
        0x2E => ld_register8_immediate(cpu, hw, L),
        0x2F => cpl(cpu),
        0x30 => jump_relative(cpu, hw, !cpu.regs.flags.carry),
        0x31 => ld_register16_immediate(cpu, hw, SP),
        0x32 => ld_addr_from_a_increment(cpu, hw, -1),
        0x33 => inc_register16(cpu, SP, 1),
        0x34 => inc_addr(cpu, hw, 1),
        0x35 => inc_addr(cpu, hw, -1),
        0x36 => ld_addr_from_immediate(cpu, hw),
        0x37 => scf(cpu),
        0x38 => jump_relative(cpu, hw, cpu.regs.flags.carry),
        0x39 => add_register16(cpu, HL, SP),
        0x3A => ld_a_from_addr_increment(cpu, hw, -1),
        0x3B => inc_register16(cpu, SP, -1),
        0x3C => inc_register8(cpu, A, 1),
        0x3D => inc_register8(cpu, A, -1),
        0x3E => ld_register8_immediate(cpu, hw, A),
        0x3F => ccf(cpu),
        0x40 => ld_register8(cpu, B, B),
        0x41 => ld_register8(cpu, B, C),
        0x42 => ld_register8(cpu, B, D),
        0x43 => ld_register8(cpu, B, E),
        0x44 => ld_register8(cpu, B, H),
        0x45 => ld_register8(cpu, B, L),
        0x46 => ld_register8_from_addr(cpu, hw, B, HL),
        0x47 => ld_register8(cpu, B, A),
        0x48 => ld_register8(cpu, C, B),
        0x49 => ld_register8(cpu, C, C),
        0x4A => ld_register8(cpu, C, D),
        0x4B => ld_register8(cpu, C, E),
        0x4C => ld_register8(cpu, C, H),
        0x4D => ld_register8(cpu, C, L),
        0x4E => ld_register8_from_addr(cpu, hw, C, HL),
        0x4F => ld_register8(cpu, C, A),
        0x50 => ld_register8(cpu, D, B),
        0x51 => ld_register8(cpu, D, C),
        0x52 => ld_register8(cpu, D, D),
        0x53 => ld_register8(cpu, D, E),
        0x54 => ld_register8(cpu, D, H),
        0x55 => ld_register8(cpu, D, L),
        0x56 => ld_register8_from_addr(cpu, hw, D, HL),
        0x57 => ld_register8(cpu, D, A),
        0x58 => ld_register8(cpu, E, B),
        0x59 => ld_register8(cpu, E, C),
        0x5A => ld_register8(cpu, E, D),
        0x5B => ld_register8(cpu, E, E),
        0x5C => ld_register8(cpu, E, H),
        0x5D => ld_register8(cpu, E, L),
        0x5E => ld_register8_from_addr(cpu, hw, E, HL),
        0x5F => ld_register8(cpu, E, A),
        0x60 => ld_register8(cpu, H, B),
        0x61 => ld_register8(cpu, H, C),
        0x62 => ld_register8(cpu, H, D),
        0x63 => ld_register8(cpu, H, E),
        0x64 => ld_register8(cpu, H, H),
        0x65 => ld_register8(cpu, H, L),
        0x66 => ld_register8_from_addr(cpu, hw, H, HL),
        0x67 => ld_register8(cpu, H, A),
        0x68 => ld_register8(cpu, L, B),
        0x69 => ld_register8(cpu, L, C),
        0x6A => ld_register8(cpu, L, D),
        0x6B => ld_register8(cpu, L, E),
        0x6C => ld_register8(cpu, L, H),
        0x6D => ld_register8(cpu, L, L),
        0x6E => ld_register8_from_addr(cpu, hw, L, HL),
        0x6F => ld_register8(cpu, L, A),
        0x70 => ld_addr_from_register8(cpu, hw, HL, B),
        0x71 => ld_addr_from_register8(cpu, hw, HL, C),
        0x72 => ld_addr_from_register8(cpu, hw, HL, D),
        0x73 => ld_addr_from_register8(cpu, hw, HL, E),
        0x74 => ld_addr_from_register8(cpu, hw, HL, H),
        0x75 => ld_addr_from_register8(cpu, hw, HL, L),
        0x76 => halt(cpu),
        0x77 => ld_addr_from_register8(cpu, hw, HL, A),
        0x78 => ld_register8(cpu, A, B),
        0x79 => ld_register8(cpu, A, C),
        0x7A => ld_register8(cpu, A, D),
        0x7B => ld_register8(cpu, A, E),
        0x7C => ld_register8(cpu, A, H),
        0x7D => ld_register8(cpu, A, L),
        0x7E => ld_register8_from_addr(cpu, hw, A, HL),
        0x7F => ld_register8(cpu, A, A),
        0x80 => add_register8(cpu, Reg(B), Positive, false),
        0x81 => add_register8(cpu, Reg(C), Positive, false),
        0x82 => add_register8(cpu, Reg(D), Positive, false),
        0x83 => add_register8(cpu, Reg(E), Positive, false),
        0x84 => add_register8(cpu, Reg(H), Positive, false),
        0x85 => add_register8(cpu, Reg(L), Positive, false),
        0x86 => add_register8(cpu, Addr(HL, hw), Positive, false),
        0x87 => add_register8(cpu, Reg(A), Positive, false),
        0x88 => add_register8(cpu, Reg(B), Positive, true),
        0x89 => add_register8(cpu, Reg(C), Positive, true),
        0x8A => add_register8(cpu, Reg(D), Positive, true),
        0x8B => add_register8(cpu, Reg(E), Positive, true),
        0x8C => add_register8(cpu, Reg(H), Positive, true),
        0x8D => add_register8(cpu, Reg(L), Positive, true),
        0x8E => add_register8(cpu, Addr(HL, hw), Positive, true),
        0x8F => add_register8(cpu, Reg(A), Positive, true),
        0x90 => add_register8(cpu, Reg(B), Negative, false),
        0x91 => add_register8(cpu, Reg(C), Negative, false),
        0x92 => add_register8(cpu, Reg(D), Negative, false),
        0x93 => add_register8(cpu, Reg(E), Negative, false),
        0x94 => add_register8(cpu, Reg(H), Negative, false),
        0x95 => add_register8(cpu, Reg(L), Negative, false),
        0x96 => add_register8(cpu, Addr(HL, hw), Negative, false),
        0x97 => add_register8(cpu, Reg(A), Negative, false),
        0x98 => add_register8(cpu, Reg(B), Negative, true),
        0x99 => add_register8(cpu, Reg(C), Negative, true),
        0x9A => add_register8(cpu, Reg(D), Negative, true),
        0x9B => add_register8(cpu, Reg(E), Negative, true),
        0x9C => add_register8(cpu, Reg(H), Negative, true),
        0x9D => add_register8(cpu, Reg(L), Negative, true),
        0x9E => add_register8(cpu, Addr(HL, hw), Negative, true),
        0x9F => add_register8(cpu, Reg(A), Negative, true),
        0xA0 => and_register8(cpu, Reg(B)),
        0xA1 => and_register8(cpu, Reg(C)),
        0xA2 => and_register8(cpu, Reg(D)),
        0xA3 => and_register8(cpu, Reg(E)),
        0xA4 => and_register8(cpu, Reg(H)),
        0xA5 => and_register8(cpu, Reg(L)),
        0xA6 => and_register8(cpu, Addr(HL, hw)),
        0xA7 => and_register8(cpu, Reg(A)),
        0xA8 => xor_register8(cpu, Reg(B)),
        0xA9 => xor_register8(cpu, Reg(C)),
        0xAA => xor_register8(cpu, Reg(D)),
        0xAB => xor_register8(cpu, Reg(E)),
        0xAC => xor_register8(cpu, Reg(H)),
        0xAD => xor_register8(cpu, Reg(L)),
        0xAE => xor_register8(cpu, Addr(HL, hw)),
        0xAF => xor_register8(cpu, Reg(A)),
        0xB0 => or_register8(cpu, Reg(B)),
        0xB1 => or_register8(cpu, Reg(C)),
        0xB2 => or_register8(cpu, Reg(D)),
        0xB3 => or_register8(cpu, Reg(E)),
        0xB4 => or_register8(cpu, Reg(H)),
        0xB5 => or_register8(cpu, Reg(L)),
        0xB6 => or_register8(cpu, Addr(HL, hw)),
        0xB7 => or_register8(cpu, Reg(A)),
        0xB8 => cp_register8(cpu, Reg(B)),
        0xB9 => cp_register8(cpu, Reg(C)),
        0xBA => cp_register8(cpu, Reg(D)),
        0xBB => cp_register8(cpu, Reg(E)),
        0xBC => cp_register8(cpu, Reg(H)),
        0xBD => cp_register8(cpu, Reg(L)),
        0xBE => cp_register8(cpu, Addr(HL, hw)),
        0xBF => cp_register8(cpu, Reg(A)),
        0xC0 => ret(cpu, hw, Some(!cpu.regs.flags.zero)),
        0xC1 => pop(cpu, hw, BC),
        0xC2 => jump_absolute(cpu, hw, !cpu.regs.flags.zero),
        0xC3 => jump_absolute(cpu, hw, true),
        0xC4 => call(cpu, hw, !cpu.regs.flags.zero),
        0xC5 => push(cpu, hw, BC),
        0xC6 => add_immediate(cpu, hw, Positive, false),
        0xC7 => rst(cpu, hw, 0x00),
        0xC8 => ret(cpu, hw, Some(cpu.regs.flags.zero)),
        0xC9 => ret(cpu, hw, None),
        0xCA => jump_absolute(cpu, hw, cpu.regs.flags.zero),
        0xCB => {
            let prefix = cpu.pop_prog_counter(hw);
            cbprefix::execute(cpu, hw, prefix)
        }
        0xCC => call(cpu, hw, cpu.regs.flags.zero),
        0xCD => call(cpu, hw, true),
        0xCE => add_immediate(cpu, hw, Positive, true),
        0xCF => rst(cpu, hw, 0x08),
        0xD0 => ret(cpu, hw, Some(!cpu.regs.flags.carry)),
        0xD1 => pop(cpu, hw, DE),
        0xD2 => jump_absolute(cpu, hw, !cpu.regs.flags.carry),
        0xD3 => unreachable!(),
        0xD4 => call(cpu, hw, !cpu.regs.flags.carry),
        0xD5 => push(cpu, hw, DE),
        0xD6 => add_immediate(cpu, hw, Negative, false),
        0xD7 => rst(cpu, hw, 0x10),
        0xD8 => ret(cpu, hw, Some(cpu.regs.flags.carry)),
        0xD9 => reti(cpu, hw),
        0xDA => jump_absolute(cpu, hw, cpu.regs.flags.carry),
        0xDB => unreachable!(),
        0xDC => call(cpu, hw, cpu.regs.flags.carry),
        0xDD => unreachable!(),
        0xDE => add_immediate(cpu, hw, Negative, true),
        0xDF => rst(cpu, hw, 0x18),
        _ => unreachable!(),
    }
}

fn nop(_cpu: &mut Cpu) -> u8 {
    4
}

#[derive(Clone, Copy)]
enum Operand<'a> {
    Reg(Register8),
    Addr(Register16, &'a Hardware),
}

impl<'a> Operand<'a> {
    fn value(self, cpu: &Cpu) -> u8 {
        match self {
            Self::Reg(reg) => cpu.regs[reg],
            Self::Addr(reg, hw) => hw.read(cpu.regs.combined(reg)),
        }
    }

    const fn extra_cycles(self) -> u8 {
        match self {
            Self::Reg(_) => 0,
            Self::Addr(_, _) => 4,
        }
    }
}

fn ld_register16_immediate(cpu: &mut Cpu, hw: &mut Hardware, reg: Register16) -> u8 {
    let lsb = cpu.pop_prog_counter(hw);
    let msb = cpu.pop_prog_counter(hw);
    cpu.regs.set_combined(reg, u16::from_le_bytes([lsb, msb]));
    12
}

fn ld_addr_from_register8(
    cpu: &mut Cpu,
    hw: &mut Hardware,
    reg_addr: Register16,
    src: Register8,
) -> u8 {
    hw.write(cpu.regs.combined(reg_addr), cpu.regs[src]);
    8
}

fn inc_register16(cpu: &mut Cpu, reg: Register16, val: i16) -> u8 {
    cpu.regs
        .set_combined(reg, ((cpu.regs.combined(reg) as i16) + val) as u16);
    8
}

fn ld_register8_immediate(cpu: &mut Cpu, hw: &mut Hardware, reg: Register8) -> u8 {
    cpu.regs[reg] = cpu.pop_prog_counter(hw);
    8
}

pub enum Direction {
    Left,
    Right,
}

fn rotate_circular_a(cpu: &mut Cpu, dir: Direction) -> u8 {
    match dir {
        Direction::Left => {
            cpu.regs.flags.carry = cpu.regs.a & 0b10000000 != 0;
            cpu.regs.a = cpu.regs.a.rotate_left(1)
        }
        Direction::Right => {
            cpu.regs.flags.carry = cpu.regs.a & 0b00000001 != 0;
            cpu.regs.a = cpu.regs.a.rotate_right(1)
        }
    };
    cpu.regs.flags.zero = false;
    cpu.regs.flags.neg = false;
    cpu.regs.flags.half_carry = false;
    4
}

fn ld_from_stack_pointer_immediate(cpu: &mut Cpu, hw: &mut Hardware) -> u8 {
    let lsb = cpu.pop_prog_counter(hw);
    let msb = cpu.pop_prog_counter(hw);
    let addr = u16::from_le_bytes([lsb, msb]);
    hw.write(addr, lo(cpu.regs.stack_pointer));
    hw.write(addr + 1, hi(cpu.regs.stack_pointer));
    20
}

fn add_register16(cpu: &mut Cpu, reg1: Register16, reg2: Register16) -> u8 {
    let (result, carry) = cpu
        .regs
        .combined(reg1)
        .overflowing_add(cpu.regs.combined(reg2));
    cpu.regs.set_combined(reg1, result);
    cpu.regs.flags.zero = false;
    cpu.regs.flags.half_carry = result >> 8 != 0;
    cpu.regs.flags.carry = carry;
    8
}

fn ld_register8_from_addr(
    cpu: &mut Cpu,
    hw: &mut Hardware,
    dst: Register8,
    reg_addr: Register16,
) -> u8 {
    cpu.regs[dst] = hw.read(cpu.regs.combined(reg_addr));
    8
}

fn inc_register8(cpu: &mut Cpu, reg: Register8, val: i8) -> u8 {
    let (result, carry) = cpu.regs[reg].overflowing_add_signed(val);
    cpu.regs[reg] = result;
    cpu.regs.flags.zero = result == 0;
    cpu.regs.flags.neg = val < 0;
    cpu.regs.flags.half_carry = carry;
    4
}

fn stop(cpu: &mut Cpu, hw: &mut Hardware) -> u8 {
    cpu.halted = false;
    cpu.pop_prog_counter(hw);
    4
}

fn rotate_a(cpu: &mut Cpu, dir: Direction) -> u8 {
    let (result, carry) = match dir {
        Direction::Left => cpu.regs.a.overflowing_shl(1),
        Direction::Right => cpu.regs.a.overflowing_shr(1),
    };
    cpu.regs.a = result;
    cpu.regs.flags.zero = false;
    cpu.regs.flags.neg = false;
    cpu.regs.flags.half_carry = false;
    cpu.regs.flags.carry = carry;
    4
}

fn jump_relative(cpu: &mut Cpu, hw: &mut Hardware, condition: bool) -> u8 {
    if !condition {
        return 8;
    }
    let offset = cpu.pop_prog_counter(hw) as i16;
    cpu.regs.prog_counter = ((cpu.regs.prog_counter as i16) + offset) as u16;
    12
}

fn ld_addr_from_a_increment(cpu: &mut Cpu, hw: &mut Hardware, inc: i16) -> u8 {
    ld_addr_from_register8(cpu, hw, Register16::HL, Register8::A);
    cpu.regs.set_combined(
        Register16::HL,
        ((cpu.regs.combined(Register16::HL) as i16) + inc) as u16,
    );
    8
}

fn daa(cpu: &mut Cpu) -> u8 {
    let a = &mut cpu.regs.a;
    if (*a & 0b00001111) > 9 || cpu.regs.flags.carry {
        *a += 0x06;
    }
    if (*a >> 4) > 9 || cpu.regs.flags.half_carry {
        *a += 0x60;
        cpu.regs.flags.carry = true;
    } else {
        cpu.regs.flags.carry = false;
    }
    cpu.regs.flags.zero = *a == 0;
    cpu.regs.flags.half_carry = false;
    4
}

fn ld_a_from_addr_increment(cpu: &mut Cpu, hw: &mut Hardware, inc: i16) -> u8 {
    ld_register8_from_addr(cpu, hw, Register8::A, Register16::HL);
    cpu.regs.set_combined(
        Register16::HL,
        ((cpu.regs.combined(Register16::HL) as i16) + inc) as u16,
    );
    8
}

fn cpl(cpu: &mut Cpu) -> u8 {
    cpu.regs.a = !cpu.regs.a;
    cpu.regs.flags.neg = true;
    cpu.regs.flags.half_carry = true;
    4
}

fn inc_addr(cpu: &mut Cpu, hw: &mut Hardware, val: i8) -> u8 {
    let byte = hw.read(cpu.regs.combined(Register16::HL));
    let (result, carry) = byte.overflowing_add_signed(val);
    hw.write(cpu.regs.combined(Register16::HL), result);
    cpu.regs.flags.zero = result == 0;
    cpu.regs.flags.neg = val < 0;
    cpu.regs.flags.half_carry = carry;
    12
}

fn ld_addr_from_immediate(cpu: &mut Cpu, hw: &mut Hardware) -> u8 {
    let byte = cpu.pop_prog_counter(hw);
    hw.write(cpu.regs.combined(Register16::HL), byte);
    12
}

fn scf(cpu: &mut Cpu) -> u8 {
    cpu.regs.flags.neg = false;
    cpu.regs.flags.half_carry = false;
    cpu.regs.flags.carry = true;
    4
}

fn ccf(cpu: &mut Cpu) -> u8 {
    cpu.regs.flags.neg = false;
    cpu.regs.flags.half_carry = false;
    cpu.regs.flags.carry = !cpu.regs.flags.carry;
    4
}

fn ld_register8(cpu: &mut Cpu, dst: Register8, src: Register8) -> u8 {
    cpu.regs[dst] = cpu.regs[src];
    4
}

fn halt(cpu: &mut Cpu) -> u8 {
    cpu.halted = true;
    4
}

#[derive(Clone, Copy)]
enum Sign {
    Positive = 1,
    Negative = -1,
}

fn add_register8(cpu: &mut Cpu, operand: Operand, sign: Sign, use_carry: bool) -> u8 {
    let carry = if use_carry {
        cpu.regs.flags.carry as i8
    } else {
        0
    };
    let (result, carry) = cpu.regs[Register8::A]
        .overflowing_add_signed((operand.value(cpu) as i8 + carry) * sign as i8);
    cpu.regs[Register8::A] = result;
    cpu.regs.flags.zero = result == 0;
    cpu.regs.flags.neg = (sign as i8) < 0;
    cpu.regs.flags.half_carry = result >> 4 != 0;
    cpu.regs.flags.carry = carry;
    4 + operand.extra_cycles()
}

fn and_register8(cpu: &mut Cpu, operand: Operand) -> u8 {
    cpu.regs[Register8::A] &= operand.value(cpu);
    cpu.regs.flags.zero = cpu.regs[Register8::A] == 0;
    cpu.regs.flags.neg = false;
    cpu.regs.flags.half_carry = true;
    cpu.regs.flags.carry = false;
    4 + operand.extra_cycles()
}

fn xor_register8(cpu: &mut Cpu, operand: Operand) -> u8 {
    cpu.regs[Register8::A] ^= operand.value(cpu);
    cpu.regs.flags.zero = cpu.regs[Register8::A] == 0;
    cpu.regs.flags.neg = false;
    cpu.regs.flags.half_carry = false;
    cpu.regs.flags.carry = false;
    4 + operand.extra_cycles()
}

fn or_register8(cpu: &mut Cpu, operand: Operand) -> u8 {
    cpu.regs[Register8::A] |= operand.value(cpu);
    cpu.regs.flags.zero = cpu.regs[Register8::A] == 0;
    cpu.regs.flags.neg = false;
    cpu.regs.flags.half_carry = false;
    cpu.regs.flags.carry = false;
    4 + operand.extra_cycles()
}

fn cp_register8(cpu: &mut Cpu, operand: Operand) -> u8 {
    let (result, carry) = cpu.regs[Register8::A].overflowing_sub(operand.value(cpu));
    cpu.regs.flags.zero = result == 0;
    cpu.regs.flags.neg = true;
    cpu.regs.flags.half_carry = result >> 4 != 0;
    cpu.regs.flags.carry = carry;
    4 + operand.extra_cycles()
}

fn ret(cpu: &mut Cpu, hw: &mut Hardware, condition: Option<bool>) -> u8 {
    let (condition, cycles) = match condition {
        Some(cond) => {
            if cond {
                (true, 20)
            } else {
                (false, 8)
            }
        }
        None => (true, 16),
    };
    if !condition {
        return cycles;
    }
    let lsb = hw.read(cpu.regs.stack_pointer);
    cpu.regs.stack_pointer += 1;
    let msb = hw.read(cpu.regs.stack_pointer);
    cpu.regs.stack_pointer += 1;
    let addr = u16::from_le_bytes([lsb, msb]);
    cpu.regs.prog_counter = addr;
    cycles
}

fn pop(cpu: &mut Cpu, hw: &mut Hardware, dest: Register16) -> u8 {
    let lsb = hw.read(cpu.regs.stack_pointer);
    cpu.regs.stack_pointer += 1;
    let msb = hw.read(cpu.regs.stack_pointer);
    cpu.regs.stack_pointer += 1;
    cpu.regs.set_combined(dest, u16::from_le_bytes([lsb, msb]));
    12
}

fn jump_absolute(cpu: &mut Cpu, hw: &mut Hardware, condition: bool) -> u8 {
    if !condition {
        return 12;
    }
    let lsb = cpu.pop_prog_counter(hw);
    let msb = cpu.pop_prog_counter(hw);
    cpu.regs.prog_counter = u16::from_le_bytes([lsb, msb]);
    16
}

fn call(cpu: &mut Cpu, hw: &mut Hardware, condition: bool) -> u8 {
    // The subroutine address is read even if the condition is false!
    let lsb = cpu.pop_prog_counter(hw);
    let msb = cpu.pop_prog_counter(hw);
    if !condition {
        return 12;
    }
    cpu.regs.stack_pointer -= 1;
    hw.write(
        cpu.regs.stack_pointer,
        cpu.regs.prog_counter.to_be_bytes()[0],
    );
    cpu.regs.stack_pointer -= 1;
    hw.write(
        cpu.regs.stack_pointer,
        cpu.regs.prog_counter.to_be_bytes()[1],
    );
    cpu.regs.prog_counter = u16::from_le_bytes([lsb, msb]);
    24
}

fn push(cpu: &mut Cpu, hw: &mut Hardware, src: Register16) -> u8 {
    let bytes = cpu.regs.combined(src).to_be_bytes();
    cpu.regs.stack_pointer -= 1;
    hw.write(cpu.regs.stack_pointer, bytes[0]);
    cpu.regs.stack_pointer -= 1;
    hw.write(cpu.regs.stack_pointer, bytes[1]);
    16
}

fn add_immediate(cpu: &mut Cpu, hw: &mut Hardware, sign: Sign, use_carry: bool) -> u8 {
    let carry = if use_carry {
        cpu.regs.flags.carry as i8
    } else {
        0
    };
    let operand = cpu.pop_prog_counter(hw);
    let (result, carry) =
        cpu.regs[Register8::A].overflowing_add_signed((operand as i8 + carry) * sign as i8);
    cpu.regs[Register8::A] = result;
    cpu.regs.flags.zero = result == 0;
    cpu.regs.flags.neg = (sign as i8) < 0;
    cpu.regs.flags.half_carry = result >> 4 != 0;
    cpu.regs.flags.carry = carry;
    4
}

fn rst(cpu: &mut Cpu, hw: &mut Hardware, lsb: u8) -> u8 {
    cpu.regs.stack_pointer -= 1;
    hw.write(
        cpu.regs.stack_pointer,
        cpu.regs.prog_counter.to_be_bytes()[0],
    );
    cpu.regs.stack_pointer -= 1;
    hw.write(
        cpu.regs.stack_pointer,
        cpu.regs.prog_counter.to_be_bytes()[1],
    );
    cpu.regs.prog_counter = u16::from_le_bytes([lsb, 0x00]);
    16
}

fn reti(cpu: &mut Cpu, hw: &mut Hardware) -> u8 {
    cpu.interrupt_enabled = true;
    ret(cpu, hw, None)
}

const fn lo(n: u16) -> u8 {
    (n & 0x00FF) as u8
}

const fn hi(n: u16) -> u8 {
    (n >> 8) as u8
}
