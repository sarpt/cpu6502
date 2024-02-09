use crate::cpu::{AddressingMode, Registers, CPU};

fn ld(cpu: &mut CPU, addr_mode: AddressingMode, register: Registers) {
    let value = match cpu.read_memory(addr_mode) {
        Some(value) => value,
        None => panic!("ld used with incorrect address mode"),
    };

    cpu.set_register(register, value);
    cpu.set_status_of_register(register);
}

pub fn lda_im(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Immediate, Registers::Accumulator);
}

pub fn lda_zp(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPage, Registers::Accumulator);
}

pub fn lda_zpx(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPageX, Registers::Accumulator);
}

pub fn lda_a(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Absolute, Registers::Accumulator);
}

pub fn lda_ax(cpu: &mut CPU) {
    ld(cpu, AddressingMode::AbsoluteX, Registers::Accumulator);
}

pub fn lda_ay(cpu: &mut CPU) {
    ld(cpu, AddressingMode::AbsoluteY, Registers::Accumulator);
}

pub fn lda_inx(cpu: &mut CPU) {
    ld(cpu, AddressingMode::IndexIndirectX, Registers::Accumulator);
}

pub fn lda_iny(cpu: &mut CPU) {
    ld(cpu, AddressingMode::IndirectIndexY, Registers::Accumulator);
}

pub fn ldy_im(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Immediate, Registers::IndexY);
}

pub fn ldy_zp(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPage, Registers::IndexY);
}

pub fn ldy_zpx(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPageX, Registers::IndexY);
}

pub fn ldy_a(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Absolute, Registers::IndexY);
}

pub fn ldy_ax(cpu: &mut CPU) {
    ld(cpu, AddressingMode::AbsoluteX, Registers::IndexY);
}

pub fn ldx_im(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Immediate, Registers::IndexX);
}

pub fn ldx_zp(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPage, Registers::IndexX);
}

pub fn ldx_zpy(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPageY, Registers::IndexX);
}

pub fn ldx_a(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Absolute, Registers::IndexX);
}

pub fn ldx_ay(cpu: &mut CPU) {
    ld(cpu, AddressingMode::AbsoluteY, Registers::IndexX);
}

pub fn store(cpu: &mut CPU, addr_mode: AddressingMode, register: Registers) {
    let value = cpu.get_register(register);
    match cpu.write_memory(addr_mode, value) {
        Some(()) => (),
        None => panic!("store_in_memory used with incorrect address mode"),
    }
}

pub fn sta_zp(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPage, Registers::Accumulator);
}

pub fn sta_zpx(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPageX, Registers::Accumulator);
}

pub fn sta_a(cpu: &mut CPU) {
    store(cpu, AddressingMode::Absolute, Registers::Accumulator);
}

pub fn sta_ax(cpu: &mut CPU) {
    store(cpu, AddressingMode::AbsoluteX, Registers::Accumulator);
}

pub fn sta_ay(cpu: &mut CPU) {
    store(cpu, AddressingMode::AbsoluteY, Registers::Accumulator);
}

pub fn sta_inx(cpu: &mut CPU) {
    store(cpu, AddressingMode::IndexIndirectX, Registers::Accumulator);
}

pub fn sta_iny(cpu: &mut CPU) {
    store(cpu, AddressingMode::IndirectIndexY, Registers::Accumulator);
}

pub fn stx_zp(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPage, Registers::IndexX);
}

pub fn stx_zpy(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPageY, Registers::IndexX);
}

pub fn stx_a(cpu: &mut CPU) {
    store(cpu, AddressingMode::Absolute, Registers::IndexX);
}

pub fn sty_zp(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPage, Registers::IndexY);
}

pub fn sty_zpx(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPageX, Registers::IndexY);
}

pub fn sty_a(cpu: &mut CPU) {
    store(cpu, AddressingMode::Absolute, Registers::IndexY);
}
