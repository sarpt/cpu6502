use crate::{
    consts::Byte,
    cpu::{AddressingMode, Registers, Tasks, CPU},
};

pub fn and(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let cb: Box<dyn Fn(&mut CPU, Byte) -> ()> = Box::new(|cpu, value| {
        let result_value = cpu.get_register(Registers::Accumulator) & value;

        cpu.set_register(Registers::Accumulator, result_value);
    });

    return cpu.read_memory(addr_mode, Some(cb));
}

pub fn and_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, AddressingMode::Immediate);
}

pub fn and_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, AddressingMode::ZeroPage);
}

pub fn and_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, AddressingMode::ZeroPageX);
}

pub fn and_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, AddressingMode::Absolute);
}

pub fn and_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, AddressingMode::AbsoluteX);
}

pub fn and_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, AddressingMode::AbsoluteY);
}

pub fn and_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, AddressingMode::IndexIndirectX);
}

pub fn and_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, AddressingMode::IndirectIndexY);
}

pub fn eor(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let cb: Box<dyn Fn(&mut CPU, Byte) -> ()> = Box::new(|cpu, value| {
        let result_value = cpu.get_register(Registers::Accumulator) ^ value;

        cpu.set_register(Registers::Accumulator, result_value);
    });

    return cpu.read_memory(addr_mode, Some(cb));
}

pub fn eor_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, AddressingMode::Immediate);
}

pub fn eor_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, AddressingMode::ZeroPage);
}

pub fn eor_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, AddressingMode::ZeroPageX);
}

pub fn eor_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, AddressingMode::Absolute);
}

pub fn eor_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, AddressingMode::AbsoluteX);
}

pub fn eor_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, AddressingMode::AbsoluteY);
}

pub fn eor_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, AddressingMode::IndexIndirectX);
}

pub fn eor_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, AddressingMode::IndirectIndexY);
}

pub fn ora(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let cb: Box<dyn Fn(&mut CPU, Byte) -> ()> = Box::new(|cpu, value| {
        let result_value = cpu.get_register(Registers::Accumulator) | value;

        cpu.set_register(Registers::Accumulator, result_value);
    });

    return cpu.read_memory(addr_mode, Some(cb));
}

pub fn ora_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, AddressingMode::Immediate);
}

pub fn ora_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, AddressingMode::ZeroPage);
}

pub fn ora_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, AddressingMode::ZeroPageX);
}

pub fn ora_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, AddressingMode::Absolute);
}

pub fn ora_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, AddressingMode::AbsoluteX);
}

pub fn ora_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, AddressingMode::AbsoluteY);
}

pub fn ora_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, AddressingMode::IndexIndirectX);
}

pub fn ora_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, AddressingMode::IndirectIndexY);
}

pub fn bit(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let cb: Box<dyn Fn(&mut CPU, Byte) -> ()> = Box::new(|cpu, value| {
        cpu.set_bit_status(cpu.accumulator & value);
    });

    return cpu.read_memory(addr_mode, Some(cb));
}

pub fn bit_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return bit(cpu, AddressingMode::ZeroPage);
}

pub fn bit_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return bit(cpu, AddressingMode::Absolute);
}

#[cfg(test)]
mod tests;
