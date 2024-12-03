use crate::{
    consts::BRK_INTERRUPT_VECTOR,
    cpu::{ChipVariant, ScheduledCycle, CPU},
};

pub fn nop(cpu: &mut CPU) {
    let mut cycles: Vec<ScheduledCycle> = Vec::new();
    cycles.push(Box::new(|cpu: &mut CPU| {
        cpu.queued_increment_program_counter();
    }));

    cpu.schedule_instruction(cycles);
}

pub fn brk(cpu: &mut CPU) {
    let mut cycles: Vec<ScheduledCycle> = Vec::new();
    cycles.push(Box::new(|cpu: &mut CPU| {
        cpu.access_memory(cpu.program_counter); // fetch and discard
        cpu.queued_increment_program_counter();
    }));

    cycles.push(Box::new(|cpu: &mut CPU| {
        cpu.push_byte_to_stack(cpu.get_program_counter_hi());
    }));

    cycles.push(Box::new(|cpu: &mut CPU| {
        cpu.push_byte_to_stack(cpu.get_program_counter_lo());
    }));

    cycles.push(Box::new(|cpu: &mut CPU| {
        cpu.push_byte_to_stack(cpu.processor_status.into());
    }));

    cycles.push(Box::new(|cpu: &mut CPU| {
        let lo = cpu.access_memory(BRK_INTERRUPT_VECTOR);
        cpu.set_program_counter_lo(lo);
    }));

    cycles.push(Box::new(|cpu: &mut CPU| {
        let hi = cpu.access_memory(BRK_INTERRUPT_VECTOR + 1);
        cpu.set_program_counter_hi(hi);

        cpu.processor_status.change_break_flag(true);
        if cpu.chip_variant == ChipVariant::NMOS {
            return;
        }

        cpu.processor_status.change_decimal_mode_flag(false);
    }));

    cpu.schedule_instruction(cycles);
}

pub fn rti(cpu: &mut CPU) {
    let mut cycles: Vec<ScheduledCycle> = Vec::new();
    cycles.push(Box::new(|cpu: &mut CPU| {
        cpu.dummy_fetch();
    }));

    // dummy tick, simulate separate stack pointer decrement
    // second cycle involves decrement of the stack pointer but poping byte from stack in third cycle does it in a single fn call
    // TODO: dont create dummy cycles, instead of decrementing and poping values in one call separate them into respective cycles
    cycles.push(Box::new(|_: &mut CPU| {}));

    cycles.push(Box::new(|cpu: &mut CPU| {
        cpu.processor_status = cpu.pop_byte_from_stack().into();
    }));

    cycles.push(Box::new(|cpu: &mut CPU| {
        let lo = cpu.pop_byte_from_stack();
        cpu.set_program_counter_lo(lo);
    }));

    cycles.push(Box::new(|cpu: &mut CPU| {
        let hi = cpu.pop_byte_from_stack();
        cpu.set_program_counter_hi(hi);
    }));

    cpu.schedule_instruction(cycles);
}

#[cfg(test)]
mod tests;
