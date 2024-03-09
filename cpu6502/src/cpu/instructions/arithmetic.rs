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

fn adc(val: Byte, acc: Byte) -> (Byte, bool, bool) {
    let (result, carry) = acc.overflowing_add(val);
    // if a sign (0x80) of a result differs from signs of both inputs
    let overflow = (acc ^ result) & (val ^ result) & 0x80 > 0;
    return (result, carry, overflow);
}

fn sbc(val: Byte, acc: Byte) -> (Byte, bool, bool) {
    let (result, carry) = acc.overflowing_sub(val);
    // if a sign (0x80) of a result differs from sign of accumulator
    // and ones-complement of value sign differs from sign of result
    let overflow = (acc ^ result) & ((0xFF - val) ^ result) & 0x80 > 0;
    return (result, carry, overflow);
}

pub fn operations_with_carry(
    cpu: &mut CPU,
    addr_mode: AddressingMode,
    op: fn(val: Byte, acc: Byte) -> (Byte, bool, bool),
) {
    let value = match cpu.read_memory(addr_mode) {
        Some(value) => value,
        None => panic!("arithmetic operation with carry used with incorrect address mode"),
    };

    let accumulator = cpu.get_register(Registers::Accumulator);
    let result = op(value, accumulator);

    cpu.set_register(Registers::Accumulator, result.0);

    if result.1 {
        cpu.processor_status.change_carry_flag(true)
    }
    if result.2 {
        cpu.processor_status.change_overflow_flag(true)
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

#[cfg(test)]
mod tests;
