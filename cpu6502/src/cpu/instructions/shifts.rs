use crate::{
    consts::Byte,
    cpu::{AddressingMode, MemoryModifications, CPU},
};

#[derive(PartialEq, Eq)]
enum ShiftDirection {
    Left,
    Right,
}

fn shift(cpu: &mut CPU, addr_mode: AddressingMode, dir: ShiftDirection) {
    let value = match cpu.read_memory(addr_mode) {
        Some(value) => value,
        None => panic!("shift used with incorrect address mode"),
    };

    let carry = value & 0b10000000 > 0;
    cpu.processor_status.change_carry_flag(carry);

    if addr_mode != AddressingMode::Accumulator {
        let modification = match dir {
            ShiftDirection::Left => MemoryModifications::ShiftLeft,
            ShiftDirection::Right => MemoryModifications::ShiftRight,
        };
        cpu.modify_memory(addr_mode, modification);
        return;
    }

    let shifted_value: Byte = match dir {
        ShiftDirection::Left => value << 1,
        ShiftDirection::Right => value >> 1,
    };
    cpu.accumulator = shifted_value;
}

fn asl(cpu: &mut CPU, addr_mode: AddressingMode) {
    shift(cpu, addr_mode, ShiftDirection::Left);
}

fn rsl(cpu: &mut CPU, addr_mode: AddressingMode) {
    shift(cpu, addr_mode, ShiftDirection::Right);
}
