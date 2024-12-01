use crate::cpu::{Registers, CPU};

fn push_register(cpu: &mut CPU, register: Registers) {
    cpu.dummy_fetch();
    cpu.push_byte_to_stack(cpu.get_register(register));
}

pub fn pha(cpu: &mut CPU) {
    push_register(cpu, Registers::Accumulator);
}

pub fn php(cpu: &mut CPU) {
    push_register(cpu, Registers::ProcessorStatus);
}

fn pull_register(cpu: &mut CPU, register: Registers) {
    cpu.dummy_fetch();
    let value = cpu.pop_byte_from_stack();
    cpu.tick();
    cpu.set_register(register, value);
}

pub fn pla(cpu: &mut CPU) {
    pull_register(cpu, Registers::Accumulator);
}

pub fn plp(cpu: &mut CPU) {
    pull_register(cpu, Registers::ProcessorStatus);
}

pub fn tsx(cpu: &mut CPU) {
    cpu.schedule_cycle(Box::new(|cpu| {
        cpu.transfer_registers(Registers::StackPointer, Registers::IndexX);
    }));

    cpu.run_next_cycles(1);
}

pub fn txs(cpu: &mut CPU) {
    cpu.schedule_cycle(Box::new(|cpu| {
        cpu.transfer_registers(Registers::IndexX, Registers::StackPointer);
    }));

    cpu.run_next_cycles(1);
}

#[cfg(test)]
mod tests;
