use std::rc::Rc;

use crate::{
    consts::Byte,
    cpu::{AddressingMode, Registers, TaskCycleVariant, Tasks, CPU},
};

fn ld(cpu: &mut CPU, addr_mode: AddressingMode, register: Registers) -> Tasks {
    let cb: Box<dyn Fn(&mut CPU, Byte) -> ()> = Box::new(move |cpu, value| {
        cpu.set_register(register, value);
    });

    return cpu.read_memory(addr_mode, Some(cb));
}

pub fn lda_im(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::Immediate, Registers::Accumulator);
}

pub fn lda_zp(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::ZeroPage, Registers::Accumulator);
}

pub fn lda_zpx(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::ZeroPageX, Registers::Accumulator);
}

pub fn lda_a(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::Absolute, Registers::Accumulator);
}

pub fn lda_ax(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::AbsoluteX, Registers::Accumulator);
}

pub fn lda_ay(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::AbsoluteY, Registers::Accumulator);
}

pub fn lda_inx(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::IndexIndirectX, Registers::Accumulator);
}

pub fn lda_iny(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::IndirectIndexY, Registers::Accumulator);
}

pub fn ldy_im(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::Immediate, Registers::IndexY);
}

pub fn ldy_zp(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::ZeroPage, Registers::IndexY);
}

pub fn ldy_zpx(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::ZeroPageX, Registers::IndexY);
}

pub fn ldy_a(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::Absolute, Registers::IndexY);
}

pub fn ldy_ax(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::AbsoluteX, Registers::IndexY);
}

pub fn ldx_im(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::Immediate, Registers::IndexX);
}

pub fn ldx_zp(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::ZeroPage, Registers::IndexX);
}

pub fn ldx_zpy(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::ZeroPageY, Registers::IndexX);
}

pub fn ldx_a(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::Absolute, Registers::IndexX);
}

pub fn ldx_ay(cpu: &mut CPU) -> Tasks {
    return ld(cpu, AddressingMode::AbsoluteY, Registers::IndexX);
}

pub fn store(cpu: &mut CPU, addr_mode: AddressingMode, register: Registers) -> Tasks {
    let mut tasks: Tasks = Tasks::new();

    let addr_cycles = cpu.get_address(addr_mode);
    tasks.append(addr_cycles);

    tasks.push(Rc::new(move |cpu| {
        let value = cpu.get_register(register);
        cpu.put_into_memory(cpu.address_output, value);

        return TaskCycleVariant::Full;
    }));

    return tasks;
}

pub fn sta_zp(cpu: &mut CPU) -> Tasks {
    return store(cpu, AddressingMode::ZeroPage, Registers::Accumulator);
}

pub fn sta_zpx(cpu: &mut CPU) -> Tasks {
    return store(cpu, AddressingMode::ZeroPageX, Registers::Accumulator);
}

pub fn sta_a(cpu: &mut CPU) -> Tasks {
    return store(cpu, AddressingMode::Absolute, Registers::Accumulator);
}

pub fn sta_ax(cpu: &mut CPU) -> Tasks {
    return store(cpu, AddressingMode::AbsoluteX, Registers::Accumulator);
}

pub fn sta_ay(cpu: &mut CPU) -> Tasks {
    return store(cpu, AddressingMode::AbsoluteY, Registers::Accumulator);
}

pub fn sta_inx(cpu: &mut CPU) -> Tasks {
    return store(cpu, AddressingMode::IndexIndirectX, Registers::Accumulator);
}

pub fn sta_iny(cpu: &mut CPU) -> Tasks {
    return store(cpu, AddressingMode::IndirectIndexY, Registers::Accumulator);
}

pub fn stx_zp(cpu: &mut CPU) -> Tasks {
    return store(cpu, AddressingMode::ZeroPage, Registers::IndexX);
}

pub fn stx_zpy(cpu: &mut CPU) -> Tasks {
    return store(cpu, AddressingMode::ZeroPageY, Registers::IndexX);
}

pub fn stx_a(cpu: &mut CPU) -> Tasks {
    return store(cpu, AddressingMode::Absolute, Registers::IndexX);
}

pub fn sty_zp(cpu: &mut CPU) -> Tasks {
    return store(cpu, AddressingMode::ZeroPage, Registers::IndexY);
}

pub fn sty_zpx(cpu: &mut CPU) -> Tasks {
    return store(cpu, AddressingMode::ZeroPageX, Registers::IndexY);
}

pub fn sty_a(cpu: &mut CPU) -> Tasks {
    return store(cpu, AddressingMode::Absolute, Registers::IndexY);
}

#[cfg(test)]
mod tests;
