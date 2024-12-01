use crate::cpu::{Registers, CPU};

fn push_register(cpu: &mut CPU, register: Registers) {
    cpu.schedule_cycle(Box::new(|cpu| {
        cpu.dummy_fetch();
    }));

    cpu.schedule_cycle(Box::new(move |cpu| {
        let val = cpu.get_register(register);
        cpu.queued_push_byte_to_stack(val);
    }));

    cpu.run_next_cycles(2);
}

pub fn pha(cpu: &mut CPU) {
    push_register(cpu, Registers::Accumulator);
}

pub fn php(cpu: &mut CPU) {
    push_register(cpu, Registers::ProcessorStatus);
}

fn pull_register(cpu: &mut CPU, register: Registers) {
    cpu.schedule_cycle(Box::new(|cpu| {
        cpu.dummy_fetch();
    }));

    // dummy tick, simulate separate stack pointer decrement
    // second cycle involves decrement of the stack pointer but poping byte from stack in third cycle does it in a single fn call
    // TODO: dont create dummy cycles, instead of decrementing and poping values in one call separate them into respective cycles
    cpu.schedule_cycle(Box::new(|_| {}));

    cpu.schedule_cycle(Box::new(move |cpu| {
        let value = cpu.queued_pop_byte_from_stack();
        cpu.set_register(register, value);
    }));

    cpu.run_next_cycles(3);
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
