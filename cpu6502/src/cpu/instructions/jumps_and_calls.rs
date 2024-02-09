use crate::cpu::{AddressingMode, MemoryOperation, CPU};

pub fn jsr_a(cpu: &mut CPU) {
    let jump_addr = match cpu.get_address(AddressingMode::Absolute, MemoryOperation::Read) {
        Some(address) => address,
        None => panic!("couldn't fetch address during a jsr"),
    };

    cpu.push_word_to_stack(cpu.program_counter - 1);
    cpu.program_counter = jump_addr;
    cpu.cycle += 1;
}

pub fn rts(cpu: &mut CPU) {
    cpu.access_memory(cpu.program_counter); // fetch and discard
    cpu.cycle += 1;

    cpu.program_counter = cpu.pop_word_from_stack();
    cpu.cycle += 1;
    cpu.increment_program_counter();
}

fn jmp(cpu: &mut CPU, addr_mode: AddressingMode) {
    match cpu.get_address(addr_mode, MemoryOperation::Read) {
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
