use crate::cpu::{AddressingMode, Registers, TaskCycleVariant, CPU};

pub fn and(cpu: &mut CPU, addr_mode: AddressingMode) {
    let mut cycles = cpu.read_memory(addr_mode);

    cycles.push(Box::new(move |cpu| {
        let value = cpu.get_read_memory_result();
        let result_value = cpu.get_register(Registers::Accumulator) & value;

        cpu.set_register(Registers::Accumulator, result_value);

        return TaskCycleVariant::Partial;
    }));

    cpu.schedule_instruction(cycles);
}

pub fn and_im(cpu: &mut CPU) {
    and(cpu, AddressingMode::Immediate);
}

pub fn and_zp(cpu: &mut CPU) {
    and(cpu, AddressingMode::ZeroPage);
}

pub fn and_zpx(cpu: &mut CPU) {
    and(cpu, AddressingMode::ZeroPageX);
}

pub fn and_a(cpu: &mut CPU) {
    and(cpu, AddressingMode::Absolute);
}

pub fn and_ax(cpu: &mut CPU) {
    and(cpu, AddressingMode::AbsoluteX);
}

pub fn and_ay(cpu: &mut CPU) {
    and(cpu, AddressingMode::AbsoluteY);
}

pub fn and_inx(cpu: &mut CPU) {
    and(cpu, AddressingMode::IndexIndirectX);
}

pub fn and_iny(cpu: &mut CPU) {
    and(cpu, AddressingMode::IndirectIndexY);
}

pub fn eor(cpu: &mut CPU, addr_mode: AddressingMode) {
    let mut cycles = cpu.read_memory(addr_mode);

    cycles.push(Box::new(move |cpu| {
        let value = cpu.get_read_memory_result();
        let result_value = cpu.get_register(Registers::Accumulator) ^ value;

        cpu.set_register(Registers::Accumulator, result_value);

        return TaskCycleVariant::Partial;
    }));

    cpu.schedule_instruction(cycles);
}

pub fn eor_im(cpu: &mut CPU) {
    eor(cpu, AddressingMode::Immediate);
}

pub fn eor_zp(cpu: &mut CPU) {
    eor(cpu, AddressingMode::ZeroPage);
}

pub fn eor_zpx(cpu: &mut CPU) {
    eor(cpu, AddressingMode::ZeroPageX);
}

pub fn eor_a(cpu: &mut CPU) {
    eor(cpu, AddressingMode::Absolute);
}

pub fn eor_ax(cpu: &mut CPU) {
    eor(cpu, AddressingMode::AbsoluteX);
}

pub fn eor_ay(cpu: &mut CPU) {
    eor(cpu, AddressingMode::AbsoluteY);
}

pub fn eor_inx(cpu: &mut CPU) {
    eor(cpu, AddressingMode::IndexIndirectX);
}

pub fn eor_iny(cpu: &mut CPU) {
    eor(cpu, AddressingMode::IndirectIndexY);
}

pub fn ora(cpu: &mut CPU, addr_mode: AddressingMode) {
    let mut cycles = cpu.read_memory(addr_mode);

    cycles.push(Box::new(move |cpu| {
        let value = cpu.get_read_memory_result();
        let result_value = cpu.get_register(Registers::Accumulator) | value;

        cpu.set_register(Registers::Accumulator, result_value);

        return TaskCycleVariant::Partial;
    }));

    cpu.schedule_instruction(cycles);
}

pub fn ora_im(cpu: &mut CPU) {
    ora(cpu, AddressingMode::Immediate);
}

pub fn ora_zp(cpu: &mut CPU) {
    ora(cpu, AddressingMode::ZeroPage);
}

pub fn ora_zpx(cpu: &mut CPU) {
    ora(cpu, AddressingMode::ZeroPageX);
}

pub fn ora_a(cpu: &mut CPU) {
    ora(cpu, AddressingMode::Absolute);
}

pub fn ora_ax(cpu: &mut CPU) {
    ora(cpu, AddressingMode::AbsoluteX);
}

pub fn ora_ay(cpu: &mut CPU) {
    ora(cpu, AddressingMode::AbsoluteY);
}

pub fn ora_inx(cpu: &mut CPU) {
    ora(cpu, AddressingMode::IndexIndirectX);
}

pub fn ora_iny(cpu: &mut CPU) {
    ora(cpu, AddressingMode::IndirectIndexY);
}

pub fn bit(cpu: &mut CPU, addr_mode: AddressingMode) {
    let mut cycles = cpu.read_memory(addr_mode);

    cycles.push(Box::new(move |cpu| {
        let value = cpu.get_read_memory_result();
        cpu.set_bit_status(cpu.accumulator & value);

        return TaskCycleVariant::Partial;
    }));

    cpu.schedule_instruction(cycles);
}

pub fn bit_zp(cpu: &mut CPU) {
    bit(cpu, AddressingMode::ZeroPage);
}

pub fn bit_a(cpu: &mut CPU) {
    bit(cpu, AddressingMode::Absolute);
}

#[cfg(test)]
mod tests;
