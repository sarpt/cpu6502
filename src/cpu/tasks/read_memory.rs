use crate::{consts::Byte, cpu::CPU};

use super::{GenericTasks, Tasks};

#[derive(PartialEq, PartialOrd)]
enum ReadMemoryStep {
    ImmediateAccess,
    AddressCalculation,
    SeparateMemoryAccess,
    Done,
}

pub struct ReadMemoryTasks {
    addressing_tasks: Box<dyn Tasks>,
    access_during_addressing: bool,
    step: ReadMemoryStep,
    value: Option<Byte>,
}

impl ReadMemoryTasks {
    pub fn new_with_access_during_addressing(addressing_tasks: Box<dyn Tasks>) -> Self {
        return ReadMemoryTasks {
            addressing_tasks,
            access_during_addressing: true,
            step: ReadMemoryStep::AddressCalculation,
            value: None,
        };
    }

    pub fn new_with_access_in_separate_cycle(addressing_tasks: Box<dyn Tasks>) -> Self {
        return ReadMemoryTasks {
            addressing_tasks,
            access_during_addressing: false,
            step: ReadMemoryStep::AddressCalculation,
            value: None,
        };
    }

    pub fn new_with_immediate_addressing() -> Self {
        return ReadMemoryTasks {
            addressing_tasks: Box::new(GenericTasks::new()),
            access_during_addressing: false,
            step: ReadMemoryStep::ImmediateAccess,
            value: None,
        };
    }

    fn access_memory(&mut self, cpu: &CPU) -> () {
        self.value = Some(cpu.access_memory(cpu.address_output));
    }

    pub fn value(&self) -> Option<Byte> {
        return self.value;
    }
}

impl Tasks for ReadMemoryTasks {
    fn done(&self) -> bool {
        self.step == ReadMemoryStep::Done
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        match self.step {
            ReadMemoryStep::ImmediateAccess => {
                cpu.address_output = cpu.program_counter;
                cpu.increment_program_counter();
                self.access_memory(cpu);
                self.step = ReadMemoryStep::Done;

                return true;
            }
            ReadMemoryStep::AddressCalculation => {
                let mut addressing_done = false;
                if !self.addressing_tasks.done() {
                    addressing_done = self.addressing_tasks.tick(cpu);
                }

                if !addressing_done {
                    return addressing_done;
                }

                if !self.access_during_addressing {
                    self.step = ReadMemoryStep::SeparateMemoryAccess;
                    return false;
                }

                self.access_memory(cpu);
                self.step = ReadMemoryStep::Done;

                return addressing_done;
            }
            ReadMemoryStep::SeparateMemoryAccess => {
                self.access_memory(cpu);
                self.step = ReadMemoryStep::Done;

                return true;
            }
            ReadMemoryStep::Done => {
                return true;
            }
        }
    }
}

#[cfg(test)]
mod read_memory_tasks {
    #[cfg(test)]
    mod immediate_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            tasks::read_memory::ReadMemoryTasks,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_set_program_counter_as_address_output() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.address_output = 0x0;
            cpu.program_counter = 0xCB;

            let tasks = Box::new(ReadMemoryTasks::new_with_immediate_addressing());
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0xCB);
        }

        #[test]
        fn should_advance_program_counter() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0xCB;

            let tasks = Box::new(ReadMemoryTasks::new_with_immediate_addressing());
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0xCC);
        }

        #[test]
        fn should_take_one_cycle() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0xCB;
            cpu.cycle = 0;

            let tasks = Box::new(ReadMemoryTasks::new_with_immediate_addressing());
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 1);
        }
    }
}
