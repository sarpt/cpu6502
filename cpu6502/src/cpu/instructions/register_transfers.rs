use crate::cpu::{Registers, CPU};

pub fn tax(cpu: &mut CPU) {
    cpu.transfer_registers(Registers::Accumulator, Registers::IndexX);
}

pub fn txa(cpu: &mut CPU) {
    cpu.transfer_registers(Registers::IndexX, Registers::Accumulator);
}

pub fn tay(cpu: &mut CPU) {
    cpu.transfer_registers(Registers::Accumulator, Registers::IndexY);
}

pub fn tya(cpu: &mut CPU) {
    cpu.transfer_registers(Registers::IndexY, Registers::Accumulator);
}

#[cfg(test)]
mod tests;
