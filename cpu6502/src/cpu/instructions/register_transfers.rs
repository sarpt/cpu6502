use crate::cpu::{Registers, CPU};

pub fn tax(cpu: &mut CPU) {
    cpu.schedule_cycle(|cpu| {
        cpu.transfer_registers(Registers::Accumulator, Registers::IndexX);
    });

    cpu.run_next_cycles(1);
}

pub fn txa(cpu: &mut CPU) {
    cpu.schedule_cycle(|cpu| {
        cpu.transfer_registers(Registers::IndexX, Registers::Accumulator);
    });

    cpu.run_next_cycles(1);
}

pub fn tay(cpu: &mut CPU) {
    cpu.schedule_cycle(|cpu| {
        cpu.transfer_registers(Registers::Accumulator, Registers::IndexY);
    });

    cpu.run_next_cycles(1);
}

pub fn tya(cpu: &mut CPU) {
    cpu.schedule_cycle(|cpu| {
        cpu.transfer_registers(Registers::IndexY, Registers::Accumulator);
    });

    cpu.run_next_cycles(1);
}

#[cfg(test)]
mod tests;
