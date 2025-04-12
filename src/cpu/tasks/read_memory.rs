use crate::{consts::Byte, cpu::CPU};

use super::Tasks;

pub trait ReadMemoryTasks: Tasks {
    fn value(&self) -> Option<Byte>;
}

#[derive(PartialEq, PartialOrd)]
enum AddressingReadMemoryStep {
    AddressCalculation,
    SeparateMemoryAccess,
    Done,
}

pub struct AddressingReadMemoryTasks {
    addressing_tasks: Box<dyn Tasks>,
    access_during_addressing: bool,
    step: AddressingReadMemoryStep,
    value: Option<Byte>,
}

impl AddressingReadMemoryTasks {
    pub fn new_with_access_during_addressing(addressing_tasks: Box<dyn Tasks>) -> Self {
        return AddressingReadMemoryTasks {
            addressing_tasks,
            access_during_addressing: true,
            step: AddressingReadMemoryStep::AddressCalculation,
            value: None,
        };
    }

    pub fn new_with_access_in_separate_cycle(addressing_tasks: Box<dyn Tasks>) -> Self {
        return AddressingReadMemoryTasks {
            addressing_tasks,
            access_during_addressing: false,
            step: AddressingReadMemoryStep::AddressCalculation,
            value: None,
        };
    }

    fn access_memory(&mut self, cpu: &CPU) -> () {
        self.value = Some(cpu.access_memory(cpu.address_output));
    }
}

impl ReadMemoryTasks for AddressingReadMemoryTasks {
    fn value(&self) -> Option<Byte> {
        return self.value;
    }
}

impl Tasks for AddressingReadMemoryTasks {
    fn done(&self) -> bool {
        self.step == AddressingReadMemoryStep::Done
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        match self.step {
            AddressingReadMemoryStep::AddressCalculation => {
                let mut addressing_done = false;
                if !self.addressing_tasks.done() {
                    addressing_done = self.addressing_tasks.tick(cpu);
                }

                if !addressing_done {
                    return addressing_done;
                }

                if !self.access_during_addressing {
                    self.step = AddressingReadMemoryStep::SeparateMemoryAccess;
                    return false;
                }

                self.access_memory(cpu);
                self.step = AddressingReadMemoryStep::Done;

                return addressing_done;
            }
            AddressingReadMemoryStep::SeparateMemoryAccess => {
                self.access_memory(cpu);
                self.step = AddressingReadMemoryStep::Done;

                return true;
            }
            AddressingReadMemoryStep::Done => {
                return true;
            }
        }
    }
}

pub struct ImmediateReadMemoryTasks {
    done: bool,
    value: Option<Byte>,
}

impl ImmediateReadMemoryTasks {
    pub fn new() -> Self {
        return ImmediateReadMemoryTasks {
            done: false,
            value: None,
        };
    }
}

impl Tasks for ImmediateReadMemoryTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        if self.done {
            panic!("tick shouldn't be called when tasks are done")
        }

        self.value = Some(cpu.access_memory(cpu.program_counter));
        cpu.increment_program_counter();
        self.done = true;

        return true;
    }
}

impl ReadMemoryTasks for ImmediateReadMemoryTasks {
    fn value(&self) -> Option<Byte> {
        return self.value;
    }
}

#[cfg(test)]
mod read_memory_tasks {
    #[cfg(test)]
    mod immediate_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            tasks::{
                read_memory::{ImmediateReadMemoryTasks, ReadMemoryTasks},
                Tasks,
            },
            tests::MemoryMock,
            CPU,
        };

        #[test]
        fn should_return_value_at_address_of_program_counter() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.address_output = 0x0;
            cpu.program_counter = 0x02;

            let mut tasks = Box::new(ImmediateReadMemoryTasks::new());
            while !tasks.done() {
                _ = tasks.tick(&mut cpu)
            }

            assert_eq!(tasks.value(), Some(0xCB));
        }

        #[test]
        fn should_advance_program_counter() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0xCB;

            let mut tasks = Box::new(ImmediateReadMemoryTasks::new());
            while !tasks.done() {
                _ = tasks.tick(&mut cpu)
            }

            assert_eq!(cpu.program_counter, 0xCC);
        }

        #[test]
        fn should_take_one_cycle() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0xCB;
            cpu.cycle = 0;

            let mut tasks = Box::new(ImmediateReadMemoryTasks::new());
            while !tasks.done() {
                _ = tasks.tick(&mut cpu);
                cpu.cycle += 1;
            }

            assert_eq!(cpu.cycle, 1);
        }
    }
}
