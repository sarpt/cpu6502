use crate::cpu::{AddressingMode, CPU};

pub fn jsr_a(cpu: &mut CPU) {
    let addr_fetch_cycles = cpu.queued_get_address(AddressingMode::Absolute);
    addr_fetch_cycles.into_iter().for_each(|cycle_cb| {
        cpu.schedule_cycle(cycle_cb);
    });

    cpu.schedule_cycle(Box::new(|cpu: &mut CPU| {
        let [_, ret_program_counter_hi] = cpu.program_counter.clone().wrapping_sub(1).to_le_bytes();
        cpu.push_byte_to_stack(ret_program_counter_hi);
    }));

    cpu.schedule_cycle(Box::new(|cpu: &mut CPU| {
        let [ret_program_counter_lo, _] = cpu.program_counter.clone().wrapping_sub(1).to_le_bytes();
        cpu.push_byte_to_stack(ret_program_counter_lo);
    }));

    cpu.schedule_cycle(Box::new(|cpu| {
        cpu.program_counter = cpu.tmp;
    }));

    cpu.run_next_cycles(5);
}

pub fn rts(cpu: &mut CPU) {
    cpu.schedule_cycle(Box::new(|cpu| {
        cpu.dummy_fetch();
    }));

    // dummy tick, simulate separate stack pointer decrement
    // second cycle involves decrement of the stack pointer but poping byte from stack in third cycle does it in a single fn call
    // TODO: dont create dummy cycles, instead of decrementing and poping values in one call separate them into respective cycles
    cpu.schedule_cycle(Box::new(|_| {}));

    cpu.schedule_cycle(Box::new(|cpu: &mut CPU| {
        let lo = cpu.pop_byte_from_stack();
        cpu.set_program_counter_lo(lo);
    }));

    cpu.schedule_cycle(Box::new(|cpu: &mut CPU| {
        let hi = cpu.pop_byte_from_stack();
        cpu.set_program_counter_hi(hi);
    }));

    cpu.schedule_cycle(Box::new(|cpu| {
        cpu.queued_increment_program_counter();
    }));

    cpu.run_next_cycles(5);
}

fn jmp(cpu: &mut CPU, addr_mode: AddressingMode) {
    match cpu.get_address(addr_mode) {
        Some(address) => cpu.program_counter = address,
        None => panic!("jmp used with incorrect addressing mode"),
    }
}

pub fn jmp_a(cpu: &mut CPU) {
    jmp(cpu, AddressingMode::Absolute);
}

pub fn jmp_in(cpu: &mut CPU) {
    jmp(cpu, AddressingMode::Indirect);
}

#[cfg(test)]
mod tests;
