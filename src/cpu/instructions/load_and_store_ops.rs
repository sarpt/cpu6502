use std::rc::Rc;

use crate::{
    consts::Byte,
    cpu::{
        addressing::get_addressing_tasks, tasks::GenericTasks, AddressingMode, Registers, Tasks,
        CPU,
    },
};

fn ld(cpu: &mut CPU, addr_mode: Option<AddressingMode>, register: Registers) -> Box<dyn Tasks> {
    let cb: Box<dyn Fn(&mut CPU, Byte) -> ()> = Box::new(move |cpu, value| {
        cpu.set_register(register, value);
    });

    return cpu.read_memory(addr_mode, Some(cb));
}

pub fn lda_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(cpu, None, Registers::Accumulator);
}

pub fn lda_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(cpu, Some(AddressingMode::ZeroPage), Registers::Accumulator);
}

pub fn lda_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(cpu, Some(AddressingMode::ZeroPageX), Registers::Accumulator);
}

pub fn lda_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(cpu, Some(AddressingMode::Absolute), Registers::Accumulator);
}

pub fn lda_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(cpu, Some(AddressingMode::AbsoluteX), Registers::Accumulator);
}

pub fn lda_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(cpu, Some(AddressingMode::AbsoluteY), Registers::Accumulator);
}

pub fn lda_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(
        cpu,
        Some(AddressingMode::IndexIndirectX),
        Registers::Accumulator,
    );
}

pub fn lda_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(
        cpu,
        Some(AddressingMode::IndirectIndexY),
        Registers::Accumulator,
    );
}

pub fn ldy_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(cpu, None, Registers::IndexY);
}

pub fn ldy_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(cpu, Some(AddressingMode::ZeroPage), Registers::IndexY);
}

pub fn ldy_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(cpu, Some(AddressingMode::ZeroPageX), Registers::IndexY);
}

pub fn ldy_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(cpu, Some(AddressingMode::Absolute), Registers::IndexY);
}

pub fn ldy_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(cpu, Some(AddressingMode::AbsoluteX), Registers::IndexY);
}

pub fn ldx_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(cpu, None, Registers::IndexX);
}

pub fn ldx_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(cpu, Some(AddressingMode::ZeroPage), Registers::IndexX);
}

pub fn ldx_zpy(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(cpu, Some(AddressingMode::ZeroPageY), Registers::IndexX);
}

pub fn ldx_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(cpu, Some(AddressingMode::Absolute), Registers::IndexX);
}

pub fn ldx_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ld(cpu, Some(AddressingMode::AbsoluteY), Registers::IndexX);
}

pub fn store(cpu: &mut CPU, addr_mode: AddressingMode, register: Registers) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(&cpu, addr_mode);
    let mut tasks = GenericTasks::new_dependent(addr_tasks);

    tasks.push(Rc::new(move |cpu| {
        let value = cpu.get_register(register);
        cpu.put_into_memory(cpu.address_output, value);
    }));

    return Box::new(tasks);
}

pub fn sta_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return store(cpu, AddressingMode::ZeroPage, Registers::Accumulator);
}

pub fn sta_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return store(cpu, AddressingMode::ZeroPageX, Registers::Accumulator);
}

pub fn sta_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return store(cpu, AddressingMode::Absolute, Registers::Accumulator);
}

pub fn sta_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return store(cpu, AddressingMode::AbsoluteX, Registers::Accumulator);
}

pub fn sta_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    return store(cpu, AddressingMode::AbsoluteY, Registers::Accumulator);
}

pub fn sta_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return store(cpu, AddressingMode::IndexIndirectX, Registers::Accumulator);
}

pub fn sta_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
    return store(cpu, AddressingMode::IndirectIndexY, Registers::Accumulator);
}

pub fn stx_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return store(cpu, AddressingMode::ZeroPage, Registers::IndexX);
}

pub fn stx_zpy(cpu: &mut CPU) -> Box<dyn Tasks> {
    return store(cpu, AddressingMode::ZeroPageY, Registers::IndexX);
}

pub fn stx_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return store(cpu, AddressingMode::Absolute, Registers::IndexX);
}

pub fn sty_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return store(cpu, AddressingMode::ZeroPage, Registers::IndexY);
}

pub fn sty_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return store(cpu, AddressingMode::ZeroPageX, Registers::IndexY);
}

pub fn sty_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return store(cpu, AddressingMode::Absolute, Registers::IndexY);
}

#[cfg(test)]
mod tests;
