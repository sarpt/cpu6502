use std::rc::Rc;

use crate::{
    consts::BRK_INTERRUPT_VECTOR,
    cpu::{tasks::GenericTasks, ChipVariant, Tasks, CPU},
};

struct NopTasks {
    done: bool,
}

impl NopTasks {
    fn new() -> Self {
        return NopTasks { done: false };
    }
}

impl Tasks for NopTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        if self.done() {
            panic!("tick mustn't be called when done")
        }

        cpu.increment_program_counter();
        self.done = true;
        return true;
    }
}

pub fn nop(_cpu: &mut CPU) -> Box<dyn Tasks> {
    return Box::new(NopTasks::new());
}

pub fn brk(_cpu: &mut CPU) -> Box<dyn Tasks> {
    let mut tasks = GenericTasks::new();
    tasks.push(Rc::new(|cpu: &mut CPU| {
        cpu.access_memory(cpu.program_counter); // fetch and discard
        cpu.increment_program_counter();
    }));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        cpu.push_byte_to_stack(cpu.get_program_counter_hi());
    }));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        cpu.push_byte_to_stack(cpu.get_program_counter_lo());
    }));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        cpu.push_byte_to_stack(cpu.processor_status.into());
    }));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        let lo = cpu.access_memory(BRK_INTERRUPT_VECTOR);
        cpu.set_program_counter_lo(lo);
    }));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        let hi = cpu.access_memory(BRK_INTERRUPT_VECTOR + 1);
        cpu.set_program_counter_hi(hi);

        cpu.processor_status.change_break_flag(true);
        if cpu.chip_variant != ChipVariant::NMOS {
            cpu.processor_status.change_decimal_mode_flag(false);
        }
    }));

    return Box::new(tasks);
}

#[derive(PartialEq, PartialOrd)]
enum RtiSteps {
    DummyFetch,
    StackPointerPreDecrement,
    PopProcessorStatus,
    PopProgramCounterLo,
    PopProgramCounterHi,
    Done,
}

struct RtiTasks {
    step: RtiSteps,
}

impl RtiTasks {
    fn new() -> Self {
        return RtiTasks {
            step: RtiSteps::DummyFetch,
        };
    }
}

impl Tasks for RtiTasks {
    fn done(&self) -> bool {
        return self.step == RtiSteps::Done;
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        match self.step {
            RtiSteps::DummyFetch => {
                cpu.dummy_fetch();

                self.step = RtiSteps::StackPointerPreDecrement;
                return false;
            }
            RtiSteps::StackPointerPreDecrement => {
                // dummy tick, simulate separate stack pointer decrement
                // second cycle involves decrement of the stack pointer but poping byte from stack in third cycle does it in a single fn call
                // TODO: dont create dummy cycles, instead of decrementing and poping values in one call separate them into respective cycles
                self.step = RtiSteps::PopProcessorStatus;
                return false;
            }
            RtiSteps::PopProcessorStatus => {
                cpu.processor_status = cpu.pop_byte_from_stack().into();
                self.step = RtiSteps::PopProgramCounterLo;
                return false;
            }
            RtiSteps::PopProgramCounterLo => {
                let lo = cpu.pop_byte_from_stack();
                cpu.set_program_counter_lo(lo);
                self.step = RtiSteps::PopProgramCounterHi;
                return false;
            }
            RtiSteps::PopProgramCounterHi => {
                let hi = cpu.pop_byte_from_stack();
                cpu.set_program_counter_hi(hi);
                self.step = RtiSteps::Done;
                return true;
            }
            RtiSteps::Done => {
                panic!("tick mustn't be called when done")
            }
        }
    }
}

pub fn rti(_cpu: &mut CPU) -> Box<dyn Tasks> {
    return Box::new(RtiTasks::new());
}

#[cfg(test)]
mod tests;
