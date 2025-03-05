use std::rc::Rc;

use crate::cpu::{processor_status::Flags, TaskCycleVariant, Tasks, CPU};

fn change_flag_value(_cpu: &mut CPU, flag: Flags, value: bool) -> Tasks {
    let mut tasks: Tasks = Tasks::new();
    tasks.push(Rc::new(move |cpu: &mut CPU| {
        cpu.processor_status.change_flag(flag, value);

        return TaskCycleVariant::Full;
    }));

    return tasks;
}

pub fn clc(cpu: &mut CPU) -> Tasks {
    return change_flag_value(cpu, Flags::Carry, false);
}

pub fn cld(cpu: &mut CPU) -> Tasks {
    return change_flag_value(cpu, Flags::DecimalMode, false);
}

pub fn cli(cpu: &mut CPU) -> Tasks {
    return change_flag_value(cpu, Flags::InterruptDisable, false);
}

pub fn clv(cpu: &mut CPU) -> Tasks {
    return change_flag_value(cpu, Flags::Overflow, false);
}

pub fn sec(cpu: &mut CPU) -> Tasks {
    return change_flag_value(cpu, Flags::Carry, true);
}

pub fn sed(cpu: &mut CPU) -> Tasks {
    return change_flag_value(cpu, Flags::DecimalMode, true);
}

pub fn sei(cpu: &mut CPU) -> Tasks {
    return change_flag_value(cpu, Flags::InterruptDisable, true);
}

#[cfg(test)]
mod tests;
