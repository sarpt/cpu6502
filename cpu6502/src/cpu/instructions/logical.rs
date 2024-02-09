use crate::cpu::{AddressingMode, Registers, CPU};

pub fn ora(cpu: &mut CPU, addr_mode: AddressingMode) {
    let value = match cpu.read_memory(addr_mode) {
        Some(value) => value,
        None => panic!("ora used with incorrect addressing mode"),
    };

    let result_value = cpu.get_register(Registers::Accumulator) | value;

    cpu.set_register(Registers::Accumulator, result_value);
    cpu.set_status_of_register(Registers::Accumulator);
}

pub fn ora_im(cpu: &mut CPU) {
    ora(cpu, AddressingMode::Immediate);
}

pub fn ora_zp(cpu: &mut CPU) {
    ora(cpu, AddressingMode::ZeroPage);
}

pub fn ora_zpx(cpu: &mut CPU) {
    ora(cpu, AddressingMode::ZeroPageX);
}

pub fn ora_a(cpu: &mut CPU) {
    ora(cpu, AddressingMode::Absolute);
}

pub fn ora_ax(cpu: &mut CPU) {
    ora(cpu, AddressingMode::AbsoluteX);
}

pub fn ora_ay(cpu: &mut CPU) {
    ora(cpu, AddressingMode::AbsoluteY);
}

pub fn ora_inx(cpu: &mut CPU) {
    ora(cpu, AddressingMode::IndexIndirectX);
}

pub fn ora_iny(cpu: &mut CPU) {
    ora(cpu, AddressingMode::IndirectIndexY);
}

pub fn bit(cpu: &mut CPU, addr_mode: AddressingMode) {
    let value = match cpu.read_memory(addr_mode) {
        Some(value) => value,
        None => panic!("bit used with incorrect addressing mode"),
    };

    cpu.set_bit_status(cpu.accumulator & value);
}

pub fn bit_zp(cpu: &mut CPU) {
    bit(cpu, AddressingMode::ZeroPage);
}

pub fn bit_a(cpu: &mut CPU) {
    bit(cpu, AddressingMode::Absolute);
}
