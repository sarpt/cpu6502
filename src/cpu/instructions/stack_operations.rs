use std::rc::Rc;

use crate::cpu::{
    tasks::{transfer_register::TransferRegistersTasks, GenericTasks},
    Registers, Tasks, CPU,
};

#[derive(PartialEq, PartialOrd)]
enum PushRegisterSteps {
    DummyFetch,
    PushToStack,
    Done,
}

struct PushRegisterTasks {
    register: Registers,
    step: PushRegisterSteps,
}

impl PushRegisterTasks {
    fn new(register: Registers) -> Self {
        return PushRegisterTasks {
            register,
            step: PushRegisterSteps::DummyFetch,
        };
    }
}

impl Tasks for PushRegisterTasks {
    fn done(&self) -> bool {
        return self.step == PushRegisterSteps::Done;
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        match self.step {
            PushRegisterSteps::DummyFetch => {
                cpu.dummy_fetch();

                self.step = PushRegisterSteps::PushToStack;
                return false;
            }
            PushRegisterSteps::PushToStack => {
                let val = cpu.get_register(self.register);
                cpu.push_byte_to_stack(val);

                self.step = PushRegisterSteps::Done;
                return true;
            }
            PushRegisterSteps::Done => {
                panic!("tick mustn't be called when done")
            }
        }
    }
}

fn push_register(_cpu: &mut CPU, register: Registers) -> Box<dyn Tasks> {
    return Box::new(PushRegisterTasks::new(register));
}

pub fn pha(cpu: &mut CPU) -> Box<dyn Tasks> {
    return push_register(cpu, Registers::Accumulator);
}

pub fn php(cpu: &mut CPU) -> Box<dyn Tasks> {
    return push_register(cpu, Registers::ProcessorStatus);
}

#[derive(PartialEq, PartialOrd)]
enum PullRegisterSteps {
    DummyFetch,
    PreDecrementStackPointer,
    PullFromStack,
    Done,
}

struct PullRegisterTasks {
    register: Registers,
    step: PullRegisterSteps,
}

impl PullRegisterTasks {
    fn new(register: Registers) -> Self {
        return PullRegisterTasks {
            register,
            step: PullRegisterSteps::DummyFetch,
        };
    }
}

impl Tasks for PullRegisterTasks {
    fn done(&self) -> bool {
        return self.step == PullRegisterSteps::Done;
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        match self.step {
            PullRegisterSteps::DummyFetch => {
                cpu.dummy_fetch();

                self.step = PullRegisterSteps::PreDecrementStackPointer;
                return false;
            }
            PullRegisterSteps::PreDecrementStackPointer => {
                // dummy tick, simulate separate stack pointer decrement
                // second cycle involves decrement of the stack pointer but poping byte from stack in third cycle does it in a single fn call
                // TODO: dont create dummy cycles, instead of decrementing and poping values in one call separate them into respective cycles
                self.step = PullRegisterSteps::PullFromStack;
                return false;
            }
            PullRegisterSteps::PullFromStack => {
                let value = cpu.pop_byte_from_stack();
                cpu.set_register(self.register, value);

                self.step = PullRegisterSteps::Done;
                return true;
            }
            PullRegisterSteps::Done => {
                panic!("tick mustn't be called when done")
            }
        }
    }
}

fn pull_register(_cpu: &mut CPU, register: Registers) -> Box<dyn Tasks> {
    return Box::new(PullRegisterTasks::new(register));
}

pub fn pla(cpu: &mut CPU) -> Box<dyn Tasks> {
    return pull_register(cpu, Registers::Accumulator);
}

pub fn plp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return pull_register(cpu, Registers::ProcessorStatus);
}

pub fn tsx(_cpu: &mut CPU) -> Box<dyn Tasks> {
    return Box::new(TransferRegistersTasks::new(
        Registers::StackPointer,
        Registers::IndexX,
    ));
}

pub fn txs(_cpu: &mut CPU) -> Box<dyn Tasks> {
    return Box::new(TransferRegistersTasks::new(
        Registers::IndexX,
        Registers::StackPointer,
    ));
}

#[cfg(test)]
mod tests;
