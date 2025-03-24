use std::rc::Rc;

use crate::cpu::{processor_status::Flags, tasks::GenericTasks, Tasks, CPU};

fn change_flag_value(_cpu: &mut CPU, flag: Flags, value: bool) -> Box<dyn Tasks> {
    let mut tasks = GenericTasks::new();
    tasks.push(Rc::new(move |cpu: &mut CPU| {
        cpu.processor_status.change_flag(flag, value);
    }));

    return Box::new(tasks);
}

pub fn clc(cpu: &mut CPU) -> Box<dyn Tasks> {
    return change_flag_value(cpu, Flags::Carry, false);
}

pub fn cld(cpu: &mut CPU) -> Box<dyn Tasks> {
    return change_flag_value(cpu, Flags::DecimalMode, false);
}

pub fn cli(cpu: &mut CPU) -> Box<dyn Tasks> {
    return change_flag_value(cpu, Flags::InterruptDisable, false);
}

pub fn clv(cpu: &mut CPU) -> Box<dyn Tasks> {
    return change_flag_value(cpu, Flags::Overflow, false);
}

pub fn sec(cpu: &mut CPU) -> Box<dyn Tasks> {
    return change_flag_value(cpu, Flags::Carry, true);
}

pub fn sed(cpu: &mut CPU) -> Box<dyn Tasks> {
    return change_flag_value(cpu, Flags::DecimalMode, true);
}

pub fn sei(cpu: &mut CPU) -> Box<dyn Tasks> {
    return change_flag_value(cpu, Flags::InterruptDisable, true);
}

#[cfg(test)]
mod tests;
