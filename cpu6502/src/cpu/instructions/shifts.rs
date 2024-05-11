use crate::{
    consts::Byte,
    cpu::{AddressingMode, MemoryModifications, Registers, CPU},
};

#[derive(PartialEq, Eq)]
enum ShiftDirection {
    Left,
    Right,
}

fn shift(cpu: &mut CPU, addr_mode: AddressingMode, dir: ShiftDirection) {
    let previous_value: Byte;
    let modified_value: Byte;

    if addr_mode != AddressingMode::Accumulator {
        let modification = match dir {
            ShiftDirection::Left => MemoryModifications::ShiftLeft,
            ShiftDirection::Right => MemoryModifications::ShiftRight,
        };
        let modification_result = cpu.modify_memory(addr_mode, modification);

        match modification_result {
            Some((previous, modified)) => {
                previous_value = previous;
                modified_value = modified;
            }
            None => panic!("could not modify memory"),
        };
    } else {
        previous_value = cpu.get_register(Registers::Accumulator);
        modified_value = match dir {
            ShiftDirection::Left => previous_value << 1,
            ShiftDirection::Right => previous_value >> 1,
        };
        cpu.accumulator = modified_value;
        cpu.cycle += 1;
    }

    let carry = match dir {
        ShiftDirection::Left => previous_value & 0b10000000 > 0,
        ShiftDirection::Right => previous_value & 0b00000001 > 0,
    };
    cpu.processor_status.change_carry_flag(carry);
    cpu.set_status_of_value(modified_value);
}

fn asl(cpu: &mut CPU, addr_mode: AddressingMode) {
    shift(cpu, addr_mode, ShiftDirection::Left);
}

pub fn asl_acc(cpu: &mut CPU) {
    asl(cpu, AddressingMode::Accumulator);
}

pub fn asl_zp(cpu: &mut CPU) {
    asl(cpu, AddressingMode::ZeroPage);
}

pub fn asl_zpx(cpu: &mut CPU) {
    asl(cpu, AddressingMode::ZeroPageX);
}

pub fn asl_a(cpu: &mut CPU) {
    asl(cpu, AddressingMode::Absolute);
}

pub fn asl_ax(cpu: &mut CPU) {
    asl(cpu, AddressingMode::AbsoluteX);
}

fn lsr(cpu: &mut CPU, addr_mode: AddressingMode) {
    shift(cpu, addr_mode, ShiftDirection::Right);
}

pub fn lsr_acc(cpu: &mut CPU) {
    lsr(cpu, AddressingMode::Accumulator);
}

pub fn lsr_zp(cpu: &mut CPU) {
    lsr(cpu, AddressingMode::ZeroPage);
}

pub fn lsr_zpx(cpu: &mut CPU) {
    lsr(cpu, AddressingMode::ZeroPageX);
}

pub fn lsr_a(cpu: &mut CPU) {
    lsr(cpu, AddressingMode::Absolute);
}

pub fn lsr_ax(cpu: &mut CPU) {
    lsr(cpu, AddressingMode::AbsoluteX);
}

#[cfg(test)]
mod tests;
