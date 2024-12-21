use crate::cpu::{AddressingMode, ScheduledCycle, TaskCycleVariant, CPU};

pub fn jsr_a(cpu: &mut CPU) {
    let mut cycles = cpu.queued_get_address(AddressingMode::Absolute);

    cycles.push(Box::new(|cpu: &mut CPU| {
        let [_, ret_program_counter_hi] = cpu.program_counter.clone().wrapping_sub(1).to_le_bytes();
        cpu.push_byte_to_stack(ret_program_counter_hi);

        return TaskCycleVariant::Full;
    }));

    cycles.push(Box::new(|cpu: &mut CPU| {
        let [ret_program_counter_lo, _] = cpu.program_counter.clone().wrapping_sub(1).to_le_bytes();
        cpu.push_byte_to_stack(ret_program_counter_lo);

        return TaskCycleVariant::Full;
    }));

    cycles.push(Box::new(|cpu| {
        cpu.program_counter = cpu.address_output;

        return TaskCycleVariant::Full;
    }));

    cpu.schedule_instruction(cycles);
}

pub fn rts(cpu: &mut CPU) {
    let mut cycles: Vec<ScheduledCycle> = Vec::new();
    cycles.push(Box::new(|cpu| {
        cpu.dummy_fetch();

        return TaskCycleVariant::Full;
    }));

    // dummy tick, simulate separate stack pointer decrement
    // second cycle involves decrement of the stack pointer but poping byte from stack in third cycle does it in a single fn call
    // TODO: dont create dummy cycles, instead of decrementing and poping values in one call separate them into respective cycles
    cycles.push(Box::new(|_| TaskCycleVariant::Full));

    cycles.push(Box::new(|cpu: &mut CPU| {
        let lo = cpu.pop_byte_from_stack();
        cpu.set_program_counter_lo(lo);

        return TaskCycleVariant::Full;
    }));

    cycles.push(Box::new(|cpu: &mut CPU| {
        let hi = cpu.pop_byte_from_stack();
        cpu.set_program_counter_hi(hi);

        return TaskCycleVariant::Full;
    }));

    cycles.push(Box::new(|cpu| {
        cpu.queued_increment_program_counter();

        return TaskCycleVariant::Full;
    }));

    cpu.schedule_instruction(cycles);
}

fn jmp(cpu: &mut CPU, addr_mode: AddressingMode) {
    let mut cycles = cpu.queued_get_address(addr_mode);
    cycles.push(Box::new(|cpu| {
        cpu.program_counter = cpu.address_output;

        return TaskCycleVariant::Partial;
    }));

    cpu.schedule_instruction(cycles);
}

pub fn jmp_a(cpu: &mut CPU) {
    jmp(cpu, AddressingMode::Absolute);
}

pub fn jmp_in(cpu: &mut CPU) {
    jmp(cpu, AddressingMode::Indirect);
}

#[cfg(test)]
mod tests;
