use crate::machine::cpu::*;

pub fn execute(cpu: &mut Cpu, opcode: u8) -> u8 {
    match opcode {
        0x00 => nop(cpu),
        0x01 => ld_register16_immediate(cpu, Register16::BC),
        0x02 => ld_addr_bc_from_a(cpu),
        0x03 => inc_bc(cpu),
        0x04 => inc_register8(cpu, Register8::B),
        0x05 => dec_register8(cpu, Register8::B),
        0x06 => ld_register8_immediate(cpu, Register8::B),
        0x07 => rotate_a(cpu, Direction::Left),
        0x08 => ld_from_stack_pointer_immediate(cpu),
        0x09 => add_hl_bc(cpu),
        0x0A => ld_a_from_bc_indirect(cpu),
        0x0B => dec_bc(cpu),
        0x0C => inc_register8(cpu, Register8::C),
        0x0D => dec_register8(cpu, Register8::C),
        0x0E => ld_register8_immediate(cpu, Register8::C),
        0x0F => rotate_a(cpu, Direction::Right),
        0x10 => stop(cpu),
        0x11 => ld_register16_immediate(cpu, Register16::DE),
        _ => unreachable!(),
    }
}

fn nop(_cpu: &mut Cpu) -> u8 {
    4
}

fn ld_register16_immediate(cpu: &mut Cpu, reg: Register16) -> u8 {
    let lsb = cpu.increment_prog_counter();
    let msb = cpu.increment_prog_counter();
    cpu.regs
        .set_combined(reg, u16::from_le_bytes([lsb, msb]));
    12
}

fn ld_addr_bc_from_a(cpu: &mut Cpu) -> u8 {
    *cpu.memory.at_mut(cpu.regs.combined(Register16::BC)) = cpu.regs.a;
    8
}

fn inc_bc(cpu: &mut Cpu) -> u8 {
    cpu.regs
        .set_combined(Register16::BC, cpu.regs.combined(Register16::BC) + 1);
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

fn rotate_a(cpu: &mut Cpu, dir: Direction) -> u8 {
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

fn add_hl_bc(cpu: &mut Cpu) -> u8 {
    let result =
        cpu.regs.combined(Register16::HL) as u32 + cpu.regs.combined(Register16::BC) as u32;
    cpu.regs.set_combined(Register16::HL, result as u16);
    cpu.regs.flags.zero = false;
    cpu.regs.flags.half_carry = result >> 8 != 0;
    cpu.regs.flags.carry = result >> 16 != 0;
    8
}

fn ld_a_from_bc_indirect(cpu: &mut Cpu) -> u8 {
    cpu.regs.a = cpu.memory.at(cpu.regs.combined(Register16::BC));
    8
}

fn dec_bc(cpu: &mut Cpu) -> u8 {
    cpu.regs
        .set_combined(Register16::BC, cpu.regs.combined(Register16::BC) - 1);
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
