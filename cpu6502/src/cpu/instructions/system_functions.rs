use crate::{
    consts::BRK_INTERRUPT_VECTOR,
    cpu::{ChipVariant, CPU},
};

pub fn nop(cpu: &mut CPU) {
    cpu.increment_program_counter();
}

pub fn brk(cpu: &mut CPU) {
    cpu.access_memory(cpu.program_counter); // fetch and discard
    cpu.increment_program_counter();

    cpu.push_word_to_stack(cpu.program_counter);
    cpu.push_byte_to_stack(cpu.processor_status.into());
    cpu.program_counter = cpu.fetch_address_from(BRK_INTERRUPT_VECTOR);

    cpu.processor_status.change_break_flag(true);
    if cpu.chip_variant == ChipVariant::NMOS {
        return;
    }

    cpu.processor_status.change_decimal_mode_flag(false);
}

pub fn rti(cpu: &mut CPU) {
    cpu.dummy_fetch();
    cpu.processor_status = cpu.pop_byte_from_stack().into();
    cpu.program_counter = cpu.pop_word_from_stack();
    cpu.tick();
}

#[cfg(test)]
mod tests;
