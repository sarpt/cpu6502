use crate::cpu::{AddressingMode, Registers, CPU};

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

#[cfg(test)]
mod tests;
