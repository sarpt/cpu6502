use crate::{consts::BRK_INTERRUPT_VECTOR, cpu::CPU};

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
}
