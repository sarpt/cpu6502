use std::{collections::VecDeque, rc::Rc};

use crate::consts::Byte;

use super::CPU;

type ScheduledTask = Rc<dyn Fn(&mut CPU) -> ()>;

pub trait Tasks {
    fn done(&self) -> bool;
    fn tick(&mut self, cpu: &mut CPU) -> bool;
}

pub struct GenericTasks {
    dependency: Option<Box<dyn Tasks>>,
    tasks_queue: VecDeque<ScheduledTask>,
}

impl GenericTasks {
    pub fn new() -> Self {
        return GenericTasks {
            dependency: None,
            tasks_queue: VecDeque::new(),
        };
    }

    pub fn new_dependent(dependency: Box<dyn Tasks>) -> Self {
        return GenericTasks {
            dependency: Some(dependency),
            tasks_queue: VecDeque::new(),
        };
    }

    pub fn push(&mut self, task: ScheduledTask) -> () {
        self.tasks_queue.push_back(task);
    }
}

impl Iterator for GenericTasks {
    type Item = ScheduledTask;

    fn next(&mut self) -> Option<Self::Item> {
        return self.tasks_queue.pop_front();
    }
}

impl Tasks for GenericTasks {
    fn done(&self) -> bool {
        return self.tasks_queue.len() == 0;
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        if let Some(dependency) = &mut self.dependency {
            if !dependency.done() {
                dependency.as_mut().tick(cpu);
                return false;
            }
        }

        if self.done() {
            return true;
        }

        if let Some(task_runner) = self.next() {
            task_runner(cpu);
        }

        return self.done();
    }
}

impl Default for GenericTasks {
    fn default() -> Self {
        Self {
            dependency: None,
            tasks_queue: Default::default(),
        }
    }
}

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
}

impl ReadMemoryTasks {
    pub fn new_with_access_during_addressing(addressing_tasks: Box<dyn Tasks>) -> Self {
        return ReadMemoryTasks {
            addressing_tasks,
            access_during_addressing: true,
            step: ReadMemoryStep::AddressCalculation,
        };
    }

    pub fn new_with_access_in_separate_cycle(addressing_tasks: Box<dyn Tasks>) -> Self {
        return ReadMemoryTasks {
            addressing_tasks,
            access_during_addressing: false,
            step: ReadMemoryStep::AddressCalculation,
        };
    }

    pub fn new_with_immediate_addressing() -> Self {
        return ReadMemoryTasks {
            addressing_tasks: Box::new(GenericTasks::new()),
            access_during_addressing: false,
            step: ReadMemoryStep::ImmediateAccess,
        };
    }

    fn access_memory(&self, cpu: &mut CPU) -> () {
        let value = cpu.access_memory(cpu.address_output);
        cpu.set_ctx_lo(value);
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
            tasks::ReadMemoryTasks,
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
        fn should_not_take_one_cycle() {
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
