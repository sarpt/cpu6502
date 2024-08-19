use crate::{
    consts::Byte,
    cpu::{AddressingMode, Registers, CPU},
};

#[derive(PartialEq, Eq)]
enum Directions {
    Left,
    Right,
}

fn get_rotate_left_cb(carry: bool) -> impl Fn(&u8) -> u8 {
    return move |value: &u8| {
        let mod_value = value << 1;
        if !carry {
            return mod_value;
        }

        return mod_value | 0b00000001;
    };
}

fn get_rotate_right_cb(carry: bool) -> impl Fn(&u8) -> u8 {
    return move |value: &u8| {
        let mod_value = value >> 1;
        if !carry {
            return mod_value;
        }

        return mod_value | 0b10000000;
    };
}

fn shift_left_cb(value: &u8) -> u8 {
    return value << 1;
}

fn shift_right_cb(value: &u8) -> u8 {
    return value >> 1;
}

fn shift(cpu: &mut CPU, addr_mode: AddressingMode, dir: Directions) {
    let previous_value: Byte;
    let modified_value: Byte;

    let cb: Box<dyn Fn(&u8) -> u8> = match dir {
        Directions::Left => Box::new(shift_left_cb),
        Directions::Right => Box::new(shift_right_cb),
    };
    if addr_mode != AddressingMode::Accumulator {
        let modification_result = cpu.modify_memory(addr_mode, &cb);

        match modification_result {
            Some((previous, modified)) => {
                previous_value = previous;
                modified_value = modified;
            }
            None => panic!("could not shift value in memory"),
        };
    } else {
        previous_value = cpu.get_register(Registers::Accumulator);
        modified_value = cb(&previous_value);
        cpu.accumulator = modified_value;
        cpu.cycle += 1;
    }

    let carry = match dir {
        Directions::Left => previous_value & 0b10000000 > 0,
        Directions::Right => previous_value & 0b00000001 > 0,
    };
    cpu.processor_status.change_carry_flag(carry);
    cpu.set_status_of_value(modified_value);
}

fn asl(cpu: &mut CPU, addr_mode: AddressingMode) {
    shift(cpu, addr_mode, Directions::Left);
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
    shift(cpu, addr_mode, Directions::Right);
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

fn rotate(cpu: &mut CPU, addr_mode: AddressingMode, dir: Directions) {
    let previous_value: Byte;
    let modified_value: Byte;
    let current_carry = cpu.processor_status.get_carry_flag();

    let cb: Box<dyn Fn(&u8) -> u8> = match dir {
        Directions::Left => Box::new(get_rotate_left_cb(current_carry)),
        Directions::Right => Box::new(get_rotate_right_cb(current_carry)),
    };
    if addr_mode != AddressingMode::Accumulator {
        let modification_result = cpu.modify_memory(addr_mode, &cb);
        match modification_result {
            Some((previous, modified)) => {
                previous_value = previous;
                modified_value = modified;
            }
            None => panic!("could not rotate value in memory"),
        };
    } else {
        previous_value = cpu.get_register(Registers::Accumulator);
        modified_value = cb(&previous_value);
        cpu.accumulator = modified_value;
        cpu.cycle += 1;
    }

    let new_carry = match dir {
        Directions::Left => previous_value & 0b10000000 > 0,
        Directions::Right => previous_value & 0b00000001 > 0,
    };
    cpu.processor_status.change_carry_flag(new_carry);
    cpu.set_status_of_value(modified_value);
}

fn rol(cpu: &mut CPU, addr_mode: AddressingMode) {
    rotate(cpu, addr_mode, Directions::Left);
}

pub fn rol_acc(cpu: &mut CPU) {
    rol(cpu, AddressingMode::Accumulator);
}

pub fn rol_zp(cpu: &mut CPU) {
    rol(cpu, AddressingMode::ZeroPage);
}

pub fn rol_zpx(cpu: &mut CPU) {
    rol(cpu, AddressingMode::ZeroPageX);
}

pub fn rol_a(cpu: &mut CPU) {
    rol(cpu, AddressingMode::Absolute);
}

pub fn rol_ax(cpu: &mut CPU) {
    rol(cpu, AddressingMode::AbsoluteX);
}

fn ror(cpu: &mut CPU, addr_mode: AddressingMode) {
    rotate(cpu, addr_mode, Directions::Right);
}

pub fn ror_acc(cpu: &mut CPU) {
    ror(cpu, AddressingMode::Accumulator);
}

pub fn ror_zp(cpu: &mut CPU) {
    ror(cpu, AddressingMode::ZeroPage);
}

pub fn ror_zpx(cpu: &mut CPU) {
    ror(cpu, AddressingMode::ZeroPageX);
}

pub fn ror_a(cpu: &mut CPU) {
    ror(cpu, AddressingMode::Absolute);
}

pub fn ror_ax(cpu: &mut CPU) {
    ror(cpu, AddressingMode::AbsoluteX);
}

#[cfg(test)]
mod tests;
