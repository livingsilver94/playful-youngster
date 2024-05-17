use std::ops::Not;

use crate::machine::cpu::*;

pub fn execute(cpu: &mut Cpu, opcode: u8) -> u8 {
    use {Register16::*, Register8::*};
    match opcode {
        0x00 => nop(cpu),
        0x01 => ld_register16_immediate(cpu, BC),
        0x02 => ld_to_addr_from_a(cpu, BC),
        0x03 => inc_register16(cpu, BC),
        0x04 => inc_register8(cpu, B),
        0x05 => dec_register8(cpu, B),
        0x06 => ld_register8_immediate(cpu, B),
        0x07 => rotate_circular_a(cpu, Direction::Left),
        0x08 => ld_from_stack_pointer_immediate(cpu),
        0x09 => add_register16(cpu, HL, BC),
        0x0A => ld_to_a_from_addr(cpu, BC),
        0x0B => dec_register16(cpu, BC),
        0x0C => inc_register8(cpu, C),
        0x0D => dec_register8(cpu, C),
        0x0E => ld_register8_immediate(cpu, C),
        0x0F => rotate_circular_a(cpu, Direction::Right),
        0x10 => stop(cpu),
        0x11 => ld_register16_immediate(cpu, DE),
        0x12 => ld_to_addr_from_a(cpu, DE),
        0x13 => inc_register16(cpu, DE),
        0x14 => inc_register8(cpu, D),
        0x15 => dec_register8(cpu, D),
        0x16 => ld_register8_immediate(cpu, D),
        0x17 => rotate_a(cpu, Direction::Left),
        0x18 => jump_relative(cpu, true),
        0x19 => add_register16(cpu, HL, DE),
        0x1A => ld_to_a_from_addr(cpu, DE),
        0x1B => dec_register16(cpu, DE),
        0x1C => inc_register8(cpu, E),
        0x1D => dec_register8(cpu, E),
        0x1E => ld_register8_immediate(cpu, E),
        0x1F => rotate_a(cpu, Direction::Right),
        0x20 => jump_relative(cpu, !cpu.regs.flags.zero),
        0x21 => ld_register16_immediate(cpu, HL),
        0x22 => ld_to_addr_from_a_increment(cpu, 1),
        0x23 => inc_register16(cpu, HL),
        0x24 => inc_register8(cpu, H),
        0x25 => dec_register8(cpu, H),
        0x26 => ld_register8_immediate(cpu, H),
        0x27 => daa(cpu),
        0x28 => jump_relative(cpu, cpu.regs.flags.zero),
        0x29 => add_register16(cpu, HL, HL),
        0x2A => ld_to_a_from_addr_increment(cpu, 1),
        0x2B => dec_register16(cpu, HL),
        0x2C => inc_register8(cpu, L),
        0x2D => dec_register8(cpu, L),
        0x2E => ld_register8_immediate(cpu, L),
        0x2F => cpl(cpu),
        _ => unreachable!(),
    }
}

fn nop(_cpu: &mut Cpu) -> u8 {
    4
}

fn ld_register16_immediate(cpu: &mut Cpu, reg: Register16) -> u8 {
    let lsb = cpu.increment_prog_counter();
    let msb = cpu.increment_prog_counter();
    cpu.regs.set_combined(reg, u16::from_le_bytes([lsb, msb]));
    12
}

fn ld_to_addr_from_a(cpu: &mut Cpu, reg: Register16) -> u8 {
    *cpu.memory.at_mut(cpu.regs.combined(reg)) = cpu.regs.a;
    8
}

fn inc_register16(cpu: &mut Cpu, reg: Register16) -> u8 {
    cpu.regs.set_combined(reg, cpu.regs.combined(reg) + 1);
    8
}

fn ld_register8_immediate(cpu: &mut Cpu, reg: Register8) -> u8 {
    cpu.regs[reg] = cpu.increment_prog_counter();
    8
}

enum Direction {
    Left,
    Right,
}

fn rotate_circular_a(cpu: &mut Cpu, dir: Direction) -> u8 {
    let result = match dir {
        Direction::Left => (cpu.regs.a as u16).rotate_left(1),
        Direction::Right => (cpu.regs.a as u16).rotate_right(1),
    };
    cpu.regs.a = result as u8;
    cpu.regs.flags.zero = false;
    cpu.regs.flags.neg = false;
    cpu.regs.flags.half_carry = false;
    cpu.regs.flags.carry = result >> 8 != 0;
    4
}

fn ld_from_stack_pointer_immediate(cpu: &mut Cpu) -> u8 {
    let lsb = cpu.increment_prog_counter();
    let msb = cpu.increment_prog_counter();
    let addr = u16::from_le_bytes([lsb, msb]);
    *cpu.memory.at_mut(addr) = lo(cpu.regs.stack_pointer);
    *cpu.memory.at_mut(addr + 1) = hi(cpu.regs.stack_pointer);
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

fn ld_to_a_from_addr(cpu: &mut Cpu, reg: Register16) -> u8 {
    cpu.regs.a = cpu.memory.at(cpu.regs.combined(reg));
    8
}

fn dec_register16(cpu: &mut Cpu, reg: Register16) -> u8 {
    cpu.regs.set_combined(reg, cpu.regs.combined(reg) - 1);
    8
}

enum Side {
    Lo,
    Hi,
}

fn inc_register8(cpu: &mut Cpu, reg: Register8) -> u8 {
    let (result, carry) = cpu.regs[reg].overflowing_add(1);
    cpu.regs[reg] = result;
    cpu.regs.flags.zero = result == 0;
    cpu.regs.flags.neg = false;
    cpu.regs.flags.half_carry = carry;
    4
}

fn dec_register8(cpu: &mut Cpu, reg: Register8) -> u8 {
    let (result, carry) = cpu.regs[reg].overflowing_add_signed(-1);
    cpu.regs[reg] = result;
    cpu.regs.flags.zero = result == 0;
    cpu.regs.flags.neg = true;
    cpu.regs.flags.half_carry = carry;
    4
}

fn stop(cpu: &mut Cpu) -> u8 {
    cpu.interrupt_enabled = false;
    cpu.increment_prog_counter();
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

fn jump_relative(cpu: &mut Cpu, condition: bool) -> u8 {
    if !condition {
        return 8;
    }
    let offset = cpu.increment_prog_counter() as i16;
    cpu.regs.prog_counter = ((cpu.regs.prog_counter as i16) + offset) as u16;
    12
}

fn ld_to_addr_from_a_increment(cpu: &mut Cpu, inc: i16) -> u8 {
    ld_to_addr_from_a(cpu, Register16::HL);
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

fn ld_to_a_from_addr_increment(cpu: &mut Cpu, inc: i16) -> u8 {
    ld_to_a_from_addr(cpu, Register16::HL);
    cpu.regs.set_combined(
        Register16::HL,
        ((cpu.regs.combined(Register16::HL) as i16) + inc) as u16,
    );
    8
}

fn cpl(cpu: &mut Cpu) -> u8 {
    cpu.regs.a = cpu.regs.a.not();
    cpu.regs.flags.neg = true;
    cpu.regs.flags.half_carry = true;
    4
}

const fn lo(n: u16) -> u8 {
    (n & 0x00FF) as u8
}

const fn hi(n: u16) -> u8 {
    (n >> 8) as u8
}

const fn set_lo(n: u16, val: u8) -> u16 {
    n & 0xFF00 | (val as u16)
}

const fn set_hi(n: u16, val: u8) -> u16 {
    (val as u16) << 8 | n & 0x00FF
}

#[cfg(test)]
mod tests {

    #[test]
    fn set_hi() {
        let obtained = super::set_hi(0b00000000_01010101, 0b11111111);
        assert_eq!(obtained, 0b11111111_01010101);
    }

    #[test]
    fn set_lo() {
        let obtained = super::set_lo(0b10101010_00000000, 0b11111111);
        assert_eq!(obtained, 0b10101010_11111111);
    }
}
