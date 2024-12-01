use crate::cpu::{AddressingMode, CPU};

pub fn jsr_a(cpu: &mut CPU) {
    let jump_addr = match cpu.get_address(AddressingMode::Absolute) {
        Some(address) => address,
        None => panic!("couldn't fetch address during a jsr"),
    };

    cpu.push_word_to_stack(cpu.program_counter - 1);
    cpu.program_counter = jump_addr;
    cpu.tick();
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
        let lo = cpu.queued_pop_byte_from_stack();
        cpu.set_program_counter_lo(lo);
    }));

    cpu.schedule_cycle(Box::new(|cpu: &mut CPU| {
        let hi = cpu.queued_pop_byte_from_stack();
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
