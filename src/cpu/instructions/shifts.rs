use std::rc::Rc;

use crate::{
    consts::Byte,
    cpu::{
        addressing::get_addressing_tasks,
        tasks::{modify_memory::ModifyMemoryTasks, GenericTasks},
        AddressingMode, Registers, Tasks, CPU,
    },
};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Directions {
    Left,
    Right,
}

fn get_rotate_left_cb(carry: bool) -> Box<dyn Fn(&u8) -> u8> {
    return Box::new(move |value: &u8| {
        let mod_value = value << 1;
        if !carry {
            return mod_value;
        }

        return mod_value | 0b00000001;
    });
}

fn get_rotate_right_cb(carry: bool) -> Box<dyn Fn(&u8) -> u8> {
    return Box::new(move |value: &u8| {
        let mod_value = value >> 1;
        if !carry {
            return mod_value;
        }

        return mod_value | 0b10000000;
    });
}

fn shift_left_cb(value: &u8) -> u8 {
    return value << 1;
}

fn shift_right_cb(value: &u8) -> u8 {
    return value >> 1;
}

fn op_acc(
    _cpu: &mut CPU,
    op: Box<dyn Fn(bool) -> Box<dyn Fn(&u8) -> u8>>,
    dir: Directions,
) -> Box<dyn Tasks> {
    let mut tasks = GenericTasks::new();

    tasks.push(Rc::new(move |cpu| {
        let previous_value: Byte;
        let modified_value: Byte;
        let current_carry = cpu.processor_status.get_carry_flag();

        let cb = op(current_carry);

        previous_value = cpu.get_register(Registers::Accumulator);
        modified_value = cb(&previous_value);
        cpu.accumulator = modified_value;

        let new_carry = match dir {
            Directions::Left => previous_value & 0b10000000 > 0,
            Directions::Right => previous_value & 0b00000001 > 0,
        };
        cpu.processor_status.change_carry_flag(new_carry);
        cpu.set_status_of_value(modified_value);
    }));

    return Box::new(tasks);
}

fn asl(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(&cpu, addr_mode);
    return Box::new(ModifyMemoryTasks::new_shift_left(addr_tasks));
}

pub fn asl_acc(cpu: &mut CPU) -> Box<dyn Tasks> {
    return op_acc(cpu, Box::new(|_| Box::new(shift_left_cb)), Directions::Left);
}

pub fn asl_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return asl(cpu, AddressingMode::ZeroPage);
}

pub fn asl_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return asl(cpu, AddressingMode::ZeroPageX);
}

pub fn asl_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return asl(cpu, AddressingMode::Absolute);
}

pub fn asl_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return asl(cpu, AddressingMode::AbsoluteX);
}

fn lsr(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(&cpu, addr_mode);
    return Box::new(ModifyMemoryTasks::new_shift_right(addr_tasks));
}

pub fn lsr_acc(cpu: &mut CPU) -> Box<dyn Tasks> {
    return op_acc(
        cpu,
        Box::new(|_| Box::new(shift_right_cb)),
        Directions::Right,
    );
}

pub fn lsr_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return lsr(cpu, AddressingMode::ZeroPage);
}

pub fn lsr_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return lsr(cpu, AddressingMode::ZeroPageX);
}

pub fn lsr_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return lsr(cpu, AddressingMode::Absolute);
}

pub fn lsr_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return lsr(cpu, AddressingMode::AbsoluteX);
}

fn rol(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(&cpu, addr_mode);
    return Box::new(ModifyMemoryTasks::new_rotate_left(addr_tasks));
}

pub fn rol_acc(cpu: &mut CPU) -> Box<dyn Tasks> {
    return op_acc(cpu, Box::new(get_rotate_left_cb), Directions::Left);
}

pub fn rol_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return rol(cpu, AddressingMode::ZeroPage);
}

pub fn rol_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return rol(cpu, AddressingMode::ZeroPageX);
}

pub fn rol_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return rol(cpu, AddressingMode::Absolute);
}

pub fn rol_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return rol(cpu, AddressingMode::AbsoluteX);
}

fn ror(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(&cpu, addr_mode);
    return Box::new(ModifyMemoryTasks::new_rotate_right(addr_tasks));
}

pub fn ror_acc(cpu: &mut CPU) -> Box<dyn Tasks> {
    return op_acc(cpu, Box::new(get_rotate_right_cb), Directions::Right);
}

pub fn ror_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ror(cpu, AddressingMode::ZeroPage);
}

pub fn ror_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ror(cpu, AddressingMode::ZeroPageX);
}

pub fn ror_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ror(cpu, AddressingMode::Absolute);
}

pub fn ror_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ror(cpu, AddressingMode::AbsoluteX);
}

#[cfg(test)]
mod tests;
