use crate::cpu::CPU;

fn branch(cpu: &mut CPU, condition: fn(&CPU) -> bool) {
    let operand = cpu.access_memory(cpu.program_counter);
    cpu.increment_program_counter();
    if !condition(cpu) {
        return;
    }

    cpu.offset_program_counter(operand)
}

pub fn bcc(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return !cpu.processor_status.get_carry_flag();
    });
}

pub fn bcs(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return cpu.processor_status.get_carry_flag();
    });
}

pub fn bne(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return !cpu.processor_status.get_zero_flag();
    });
}

pub fn beq(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return cpu.processor_status.get_zero_flag();
    });
}

#[cfg(test)]
mod tests;
