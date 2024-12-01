use crate::{
    consts::BRK_INTERRUPT_VECTOR,
    cpu::{ChipVariant, CPU},
};

pub fn nop(cpu: &mut CPU) {
    cpu.schedule_cycle(Box::new(|cpu: &mut CPU| {
        cpu.queued_increment_program_counter();
    }));

    cpu.run_next_cycles(1);
}

pub fn brk(cpu: &mut CPU) {
    cpu.schedule_cycle(Box::new(|cpu: &mut CPU| {
        cpu.access_memory(cpu.program_counter); // fetch and discard
        cpu.queued_increment_program_counter();
    }));

    cpu.schedule_cycle(Box::new(|cpu: &mut CPU| {
        cpu.queued_push_byte_to_stack(cpu.get_program_counter_hi());
    }));

    cpu.schedule_cycle(Box::new(|cpu: &mut CPU| {
        cpu.queued_push_byte_to_stack(cpu.get_program_counter_lo());
    }));

    cpu.schedule_cycle(Box::new(|cpu: &mut CPU| {
        cpu.queued_push_byte_to_stack(cpu.processor_status.into());
    }));

    cpu.schedule_cycle(Box::new(|cpu: &mut CPU| {
        let lo = cpu.access_memory(BRK_INTERRUPT_VECTOR);
        cpu.set_program_counter_lo(lo);
    }));

    cpu.schedule_cycle(Box::new(|cpu: &mut CPU| {
        let hi = cpu.access_memory(BRK_INTERRUPT_VECTOR + 1);
        cpu.set_program_counter_hi(hi);

        cpu.processor_status.change_break_flag(true);
        if cpu.chip_variant == ChipVariant::NMOS {
            return;
        }

        cpu.processor_status.change_decimal_mode_flag(false);
    }));

    cpu.run_next_cycles(6);
}

pub fn rti(cpu: &mut CPU) {
    cpu.schedule_cycle(Box::new(|cpu: &mut CPU| {
        cpu.access_memory(cpu.program_counter); // fetch and discard
    }));

    // dummy tick, simulate separate stack pointer decrement
    // second cycle involves decrement of the stack pointer but poping byte from stack in third cycle does it in a single fn call
    // TODO: dont create dummy cycles, instead of decrementing and poping values in one call separate them into respective cycles
    cpu.schedule_cycle(Box::new(|_: &mut CPU| {}));

    cpu.schedule_cycle(Box::new(|cpu: &mut CPU| {
        cpu.processor_status = cpu.queued_pop_byte_from_stack().into();
    }));

    cpu.schedule_cycle(Box::new(|cpu: &mut CPU| {
        let lo = cpu.queued_pop_byte_from_stack();
        cpu.set_program_counter_lo(lo);
    }));

    cpu.schedule_cycle(Box::new(|cpu: &mut CPU| {
        let hi = cpu.queued_pop_byte_from_stack();
        cpu.set_program_counter_hi(hi);
    }));

    cpu.run_next_cycles(5);
}

#[cfg(test)]
mod tests;
