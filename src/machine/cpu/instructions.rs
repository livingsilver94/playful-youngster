use crate::machine::cpu::{Cpu, Flags};

pub fn execute(cpu: &mut Cpu, opcode: u8) -> u8 {
    match opcode {
        0x00 => nop(cpu),
        0x01 => ld_bc_immediate(cpu),
        0x02 => ld_addr_bc_from_a(cpu),
        0x03 => inc_bc(cpu),
        0x04 => inc_half_register(&mut cpu.regs.bc, &mut cpu.regs.flags, Side::Hi),
        0x05 => dec_b(cpu),
        0x06 => ld_b_immediate(cpu),
        0x07 => rlca(cpu),
        0x08 => ld_from_stack_pointer_immediate(cpu),
        0x09 => add_hl_bc(cpu),
        0x0A => ld_a_from_bc_indirect(cpu),
        0x0B => dec_bc(cpu),
        0x0C => inc_half_register(&mut cpu.regs.bc, &mut cpu.regs.flags, Side::Lo),
        _ => unreachable!(),
    }
}

fn nop(_cpu: &mut Cpu) -> u8 {
    4
}

fn ld_bc_immediate(cpu: &mut Cpu) -> u8 {
    let lsb = cpu.increment_prog_counter();
    let msb = cpu.increment_prog_counter();
    cpu.regs.bc = u16::from_le_bytes([lsb, msb]);
    12
}

fn ld_addr_bc_from_a(cpu: &mut Cpu) -> u8 {
    *cpu.memory.at_mut(cpu.regs.bc) = cpu.regs.a;
    8
}

fn inc_bc(cpu: &mut Cpu) -> u8 {
    cpu.regs.bc += 1;
    8
}

fn dec_b(cpu: &mut Cpu) -> u8 {
    let result = ((cpu.regs.bc & 0xFF00) >> 8) - 1;

    cpu.regs.bc = set_hi(cpu.regs.bc, result as u8);
    cpu.regs.flags.zero = result as u8 == 0;
    cpu.regs.flags.neg = true;
    cpu.regs.flags.half_carry = result >> 8 != 0;
    4
}

fn ld_b_immediate(cpu: &mut Cpu) -> u8 {
    let byte = cpu.increment_prog_counter();
    cpu.regs.bc = set_lo(cpu.regs.bc, byte);
    8
}

fn rlca(cpu: &mut Cpu) -> u8 {
    let result = (cpu.regs.a as u16).rotate_left(1);

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
    let result = cpu.regs.hl as u32 + cpu.regs.bc as u32;
    cpu.regs.flags.zero = false;
    cpu.regs.flags.half_carry = result >> 8 != 0;
    cpu.regs.flags.carry = result >> 16 != 0;
    8
}

fn ld_a_from_bc_indirect(cpu: &mut Cpu) -> u8 {
    cpu.regs.a = cpu.memory.at(cpu.regs.bc);
    8
}

fn dec_bc(cpu: &mut Cpu) -> u8 {
    cpu.regs.bc -= 1;
    8
}

fn inc_half_register(reg: &mut u16, flags: &mut Flags, side: Side) -> u8 {
    let result;
    match side {
        Side::Lo => {
            result = (*reg & 0x00FF) + 1;
            *reg = set_lo(*reg, result as u8);
        }
        Side::Hi => {
            result = ((*reg & 0xFF00) >> 8) + 1;
            *reg = set_hi(*reg, result as u8);
        }
    };
    flags.zero = result as u8 == 0;
    flags.neg = false;
    flags.half_carry = result >> 8 != 0;
    4
}

enum Side {
    Lo,
    Hi,
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
