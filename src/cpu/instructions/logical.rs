use crate::{
    consts::Byte,
    cpu::{AddressingMode, Registers, Tasks, CPU},
};

pub fn and(cpu: &mut CPU, addr_mode: Option<AddressingMode>) -> Box<dyn Tasks> {
    let cb: Box<dyn Fn(&mut CPU, Byte) -> ()> = Box::new(|cpu, value| {
        let result_value = cpu.get_register(Registers::Accumulator) & value;

        cpu.set_register(Registers::Accumulator, result_value);
    });

    return cpu.read_memory(addr_mode, Some(cb));
}

pub fn and_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, None);
}

pub fn and_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, Some(AddressingMode::ZeroPage));
}

pub fn and_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, Some(AddressingMode::ZeroPageX));
}

pub fn and_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, Some(AddressingMode::Absolute));
}

pub fn and_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, Some(AddressingMode::AbsoluteX));
}

pub fn and_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, Some(AddressingMode::AbsoluteY));
}

pub fn and_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, Some(AddressingMode::IndexIndirectX));
}

pub fn and_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, Some(AddressingMode::IndirectIndexY));
}

pub fn eor(cpu: &mut CPU, addr_mode: Option<AddressingMode>) -> Box<dyn Tasks> {
    let cb: Box<dyn Fn(&mut CPU, Byte) -> ()> = Box::new(|cpu, value| {
        let result_value = cpu.get_register(Registers::Accumulator) ^ value;

        cpu.set_register(Registers::Accumulator, result_value);
    });

    return cpu.read_memory(addr_mode, Some(cb));
}

pub fn eor_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, None);
}

pub fn eor_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, Some(AddressingMode::ZeroPage));
}

pub fn eor_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, Some(AddressingMode::ZeroPageX));
}

pub fn eor_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, Some(AddressingMode::Absolute));
}

pub fn eor_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, Some(AddressingMode::AbsoluteX));
}

pub fn eor_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, Some(AddressingMode::AbsoluteY));
}

pub fn eor_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, Some(AddressingMode::IndexIndirectX));
}

pub fn eor_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, Some(AddressingMode::IndirectIndexY));
}

pub fn ora(cpu: &mut CPU, addr_mode: Option<AddressingMode>) -> Box<dyn Tasks> {
    let cb: Box<dyn Fn(&mut CPU, Byte) -> ()> = Box::new(|cpu, value| {
        let result_value = cpu.get_register(Registers::Accumulator) | value;

        cpu.set_register(Registers::Accumulator, result_value);
    });

    return cpu.read_memory(addr_mode, Some(cb));
}

pub fn ora_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, None);
}

pub fn ora_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, Some(AddressingMode::ZeroPage));
}

pub fn ora_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, Some(AddressingMode::ZeroPageX));
}

pub fn ora_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, Some(AddressingMode::Absolute));
}

pub fn ora_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, Some(AddressingMode::AbsoluteX));
}

pub fn ora_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, Some(AddressingMode::AbsoluteY));
}

pub fn ora_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, Some(AddressingMode::IndexIndirectX));
}

pub fn ora_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, Some(AddressingMode::IndirectIndexY));
}

pub fn bit(cpu: &mut CPU, addr_mode: Option<AddressingMode>) -> Box<dyn Tasks> {
    let cb: Box<dyn Fn(&mut CPU, Byte) -> ()> = Box::new(|cpu, value| {
        cpu.set_bit_status(cpu.accumulator & value);
    });

    return cpu.read_memory(addr_mode, Some(cb));
}

pub fn bit_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return bit(cpu, Some(AddressingMode::ZeroPage));
}

pub fn bit_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return bit(cpu, Some(AddressingMode::Absolute));
}

#[cfg(test)]
mod tests;
