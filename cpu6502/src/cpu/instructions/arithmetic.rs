use crate::{
    consts::Byte,
    cpu::{AddressingMode, Registers, CPU},
};

fn compare(cpu: &mut CPU, addr_mode: AddressingMode, register: Registers) {
    let value = match cpu.read_memory(addr_mode) {
        Some(value) => value,
        None => panic!("compare used with incorrect address mode"),
    };

    cpu.set_cmp_status(register, value);
}

pub fn cmp_im(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Immediate, Registers::Accumulator);
}

pub fn cmp_zp(cpu: &mut CPU) {
    compare(cpu, AddressingMode::ZeroPage, Registers::Accumulator);
}

pub fn cmp_zpx(cpu: &mut CPU) {
    compare(cpu, AddressingMode::ZeroPageX, Registers::Accumulator);
}

pub fn cmp_a(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Absolute, Registers::Accumulator);
}

pub fn cmp_ax(cpu: &mut CPU) {
    compare(cpu, AddressingMode::AbsoluteX, Registers::Accumulator);
}

pub fn cmp_ay(cpu: &mut CPU) {
    compare(cpu, AddressingMode::AbsoluteY, Registers::Accumulator);
}

pub fn cmp_inx(cpu: &mut CPU) {
    compare(cpu, AddressingMode::IndexIndirectX, Registers::Accumulator);
}

pub fn cmp_iny(cpu: &mut CPU) {
    compare(cpu, AddressingMode::IndirectIndexY, Registers::Accumulator);
}

pub fn cpx_im(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Immediate, Registers::IndexX);
}

pub fn cpx_zp(cpu: &mut CPU) {
    compare(cpu, AddressingMode::ZeroPage, Registers::IndexX);
}

pub fn cpx_a(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Absolute, Registers::IndexX);
}

pub fn cpy_im(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Immediate, Registers::IndexY);
}

pub fn cpy_zp(cpu: &mut CPU) {
    compare(cpu, AddressingMode::ZeroPage, Registers::IndexY);
}

pub fn cpy_a(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Absolute, Registers::IndexY);
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FlagOp {
    Unchanged,
    Set,
    Clear,
}

fn adc(val: Byte, acc: Byte, carry: bool) -> (Byte, FlagOp, FlagOp) {
    let (result, carry) = acc.overflowing_add(val);
    // if a sign (0x80) of a result differs from signs of both inputs
    let overflow = (acc ^ result) & (val ^ result) & 0x80 > 0;

    let carry_op = if carry {
        FlagOp::Set
    } else {
        FlagOp::Unchanged
    };
    let overflow_op = if overflow {
        FlagOp::Set
    } else {
        FlagOp::Unchanged
    };
    return (result, carry_op, overflow_op);
}

fn sbc(val: Byte, acc: Byte, carry: bool) -> (Byte, FlagOp, FlagOp) {
    let (result, carry) = acc.overflowing_add(0xFF - val + (carry as u8));
    // if a sign (0x80) of a result differs from sign of accumulator
    // and ones-complement of value sign differs from sign of result
    let overflow = (acc ^ result) & ((0xFF - val) ^ result) & 0x80 > 0;

    let carry_op = if carry {
        FlagOp::Clear
    } else {
        FlagOp::Unchanged
    };
    let overflow_op = if overflow {
        FlagOp::Set
    } else {
        FlagOp::Unchanged
    };
    return (result, carry_op, overflow_op);
}

pub fn operations_with_carry(
    cpu: &mut CPU,
    addr_mode: AddressingMode,
    op: fn(val: Byte, acc: Byte, carry: bool) -> (Byte, FlagOp, FlagOp),
) {
    let value = match cpu.read_memory(addr_mode) {
        Some(value) => value,
        None => panic!("arithmetic operation with carry used with incorrect address mode"),
    };

    let accumulator = cpu.get_register(Registers::Accumulator);
    let (value, carry, overflow) = op(value, accumulator, cpu.processor_status.get_carry_flag());

    cpu.set_register(Registers::Accumulator, value);

    if carry != FlagOp::Unchanged {
        cpu.processor_status.change_carry_flag(carry == FlagOp::Set)
    }
    if overflow != FlagOp::Unchanged {
        cpu.processor_status
            .change_overflow_flag(overflow == FlagOp::Set)
    }
}

pub fn adc_im(cpu: &mut CPU) {
    operations_with_carry(cpu, AddressingMode::Immediate, adc);
}

pub fn adc_zp(cpu: &mut CPU) {
    operations_with_carry(cpu, AddressingMode::ZeroPage, adc);
}

pub fn adc_zpx(cpu: &mut CPU) {
    operations_with_carry(cpu, AddressingMode::ZeroPageX, adc);
}

pub fn adc_a(cpu: &mut CPU) {
    operations_with_carry(cpu, AddressingMode::Absolute, adc);
}

pub fn adc_ax(cpu: &mut CPU) {
    operations_with_carry(cpu, AddressingMode::AbsoluteX, adc);
}

pub fn adc_ay(cpu: &mut CPU) {
    operations_with_carry(cpu, AddressingMode::AbsoluteY, adc);
}

pub fn adc_inx(cpu: &mut CPU) {
    operations_with_carry(cpu, AddressingMode::IndexIndirectX, adc);
}

pub fn adc_iny(cpu: &mut CPU) {
    operations_with_carry(cpu, AddressingMode::IndirectIndexY, adc);
}

pub fn sbc_im(cpu: &mut CPU) {
    operations_with_carry(cpu, AddressingMode::Immediate, sbc);
}

pub fn sbc_zp(cpu: &mut CPU) {
    operations_with_carry(cpu, AddressingMode::ZeroPage, sbc);
}

pub fn sbc_zpx(cpu: &mut CPU) {
    operations_with_carry(cpu, AddressingMode::ZeroPageX, sbc);
}

pub fn sbc_a(cpu: &mut CPU) {
    operations_with_carry(cpu, AddressingMode::Absolute, sbc);
}

pub fn sbc_ax(cpu: &mut CPU) {
    operations_with_carry(cpu, AddressingMode::AbsoluteX, sbc);
}

pub fn sbc_ay(cpu: &mut CPU) {
    operations_with_carry(cpu, AddressingMode::AbsoluteY, sbc);
}

pub fn sbc_inx(cpu: &mut CPU) {
    operations_with_carry(cpu, AddressingMode::IndexIndirectX, sbc);
}

pub fn sbc_iny(cpu: &mut CPU) {
    operations_with_carry(cpu, AddressingMode::IndirectIndexY, sbc);
}

#[cfg(test)]
mod tests;
