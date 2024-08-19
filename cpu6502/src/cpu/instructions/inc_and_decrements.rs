use crate::cpu::{AddressingMode, Registers, CPU};

fn decrement_cb(value: &u8) -> u8 {
    return value.wrapping_sub(1);
}

fn increment_cb(value: &u8) -> u8 {
    return value.wrapping_add(1);
}

fn decrement_memory(cpu: &mut CPU, addr_mode: AddressingMode) {
    match cpu.modify_memory(addr_mode, &decrement_cb) {
        Some((_, modified_value)) => {
            cpu.set_status_of_value(modified_value);
        }
        None => panic!("decrement_memory used with incorrect addressing mode"),
    };
}

fn decrement_register(cpu: &mut CPU, register: Registers) {
    match register {
        Registers::IndexX | Registers::IndexY => {
            cpu.decrement_register(register);
        }
        _ => panic!("decrement_register used with incorrect register"),
    }
}

pub fn dec_zp(cpu: &mut CPU) {
    decrement_memory(cpu, AddressingMode::ZeroPage);
}

pub fn dec_zpx(cpu: &mut CPU) {
    decrement_memory(cpu, AddressingMode::ZeroPageX);
}

pub fn dec_a(cpu: &mut CPU) {
    decrement_memory(cpu, AddressingMode::Absolute);
}

pub fn dec_ax(cpu: &mut CPU) {
    decrement_memory(cpu, AddressingMode::AbsoluteX);
}

pub fn dex_im(cpu: &mut CPU) {
    decrement_register(cpu, Registers::IndexX);
}

pub fn dey_im(cpu: &mut CPU) {
    decrement_register(cpu, Registers::IndexY);
}

fn increment_memory(cpu: &mut CPU, addr_mode: AddressingMode) {
    match cpu.modify_memory(addr_mode, &increment_cb) {
        Some((_, modified_value)) => {
            cpu.set_status_of_value(modified_value);
        }
        None => panic!("increment_memory used with incorrect addressing mode"),
    };
}

fn increment_register(cpu: &mut CPU, register: Registers) {
    match register {
        Registers::IndexX | Registers::IndexY => {
            cpu.increment_register(register);
        }
        _ => panic!("increment_register used with incorrect register"),
    }
}

pub fn inc_zp(cpu: &mut CPU) {
    increment_memory(cpu, AddressingMode::ZeroPage);
}

pub fn inc_zpx(cpu: &mut CPU) {
    increment_memory(cpu, AddressingMode::ZeroPageX);
}

pub fn inc_a(cpu: &mut CPU) {
    increment_memory(cpu, AddressingMode::Absolute);
}

pub fn inc_ax(cpu: &mut CPU) {
    increment_memory(cpu, AddressingMode::AbsoluteX);
}

pub fn inx_im(cpu: &mut CPU) {
    increment_register(cpu, Registers::IndexX);
}

pub fn iny_im(cpu: &mut CPU) {
    increment_register(cpu, Registers::IndexY);
}

#[cfg(test)]
mod tests;
