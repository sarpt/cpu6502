use crate::cpu::{processor_status::Flags, ScheduledCycle, TaskCycleVariant, CPU};

fn change_flag_value(cpu: &mut CPU, flag: Flags, value: bool) {
    let mut cycles: Vec<ScheduledCycle> = Vec::new();
    cycles.push(Box::new(move |cpu: &mut CPU| {
        cpu.processor_status.change_flag(flag, value);

        return TaskCycleVariant::Full;
    }));

    cpu.schedule_instruction(cycles);
}

pub fn clc(cpu: &mut CPU) {
    change_flag_value(cpu, Flags::Carry, false);
}

pub fn cld(cpu: &mut CPU) {
    change_flag_value(cpu, Flags::DecimalMode, false);
}

pub fn cli(cpu: &mut CPU) {
    change_flag_value(cpu, Flags::InterruptDisable, false);
}

pub fn clv(cpu: &mut CPU) {
    change_flag_value(cpu, Flags::Overflow, false);
}

pub fn sec(cpu: &mut CPU) {
    change_flag_value(cpu, Flags::Carry, true);
}

pub fn sed(cpu: &mut CPU) {
    change_flag_value(cpu, Flags::DecimalMode, true);
}

pub fn sei(cpu: &mut CPU) {
    change_flag_value(cpu, Flags::InterruptDisable, true);
}

#[cfg(test)]
mod tests;
