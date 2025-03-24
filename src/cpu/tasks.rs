use std::{collections::VecDeque, rc::Rc};

use crate::consts::Byte;

use super::CPU;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskCycleVariant {
    Full,
}

type ScheduledTask = Rc<dyn Fn(&mut CPU) -> TaskCycleVariant>;

pub trait Tasks {
    fn done(&self) -> bool;
    fn tick(&mut self, cpu: &mut CPU) -> (bool, bool);
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

    fn tick(&mut self, cpu: &mut CPU) -> (bool, bool) {
        if let Some(dependency) = &mut self.dependency {
            if !dependency.done() {
                let (took_cycles, _) = dependency.as_mut().tick(cpu);
                return (took_cycles, false);
            }
        }

        if self.done() {
            return (false, true);
        }

        let mut took_cycles: bool = false;
        while let Some(task_runner) = self.next() {
            let task_cycle_variant = task_runner(cpu);
            if task_cycle_variant == TaskCycleVariant::Full {
                took_cycles = true;
                break;
            };
        }

        return (took_cycles, self.done());
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
    AddressCalculation,
    SeparateMemoryAccess,
    Done,
}

pub struct ReadMemoryTasks {
    addressing_tasks: Box<dyn Tasks>,
    address_fixing: bool,
    step: ReadMemoryStep,
    value_reader: Option<Box<dyn Fn(&mut CPU, Byte) -> ()>>,
}

impl ReadMemoryTasks {
    pub fn new_with_address_fixing(
        addressing_tasks: Box<dyn Tasks>,
        value_reader: Option<Box<dyn Fn(&mut CPU, Byte) -> ()>>,
    ) -> Self {
        return ReadMemoryTasks {
            addressing_tasks,
            address_fixing: true,
            step: ReadMemoryStep::AddressCalculation,
            value_reader,
        };
    }

    pub fn new_without_address_fixing(
        addressing_tasks: Box<dyn Tasks>,
        value_reader: Option<Box<dyn Fn(&mut CPU, Byte) -> ()>>,
    ) -> Self {
        return ReadMemoryTasks {
            addressing_tasks,
            address_fixing: false,
            step: ReadMemoryStep::AddressCalculation,
            value_reader,
        };
    }

    fn access_memory(&self, cpu: &mut CPU) -> () {
        let value = cpu.access_memory(cpu.address_output);
        cpu.set_ctx_lo(value);

        if let Some(vr) = &self.value_reader {
            vr(cpu, value)
        }
    }
}

impl Tasks for ReadMemoryTasks {
    fn done(&self) -> bool {
        self.step == ReadMemoryStep::Done
    }

    fn tick(&mut self, cpu: &mut CPU) -> (bool, bool) {
        match self.step {
            ReadMemoryStep::AddressCalculation => {
                let (mut addressing_took_cycle, mut addressing_done) = (false, false);
                if !self.addressing_tasks.done() {
                    (addressing_took_cycle, addressing_done) = self.addressing_tasks.tick(cpu);
                }

                if !addressing_done {
                    return (addressing_took_cycle, addressing_done);
                }

                if !self.address_fixing {
                    self.step = ReadMemoryStep::SeparateMemoryAccess;
                    return (addressing_took_cycle, false);
                }

                self.access_memory(cpu);
                self.step = ReadMemoryStep::Done;

                return (addressing_took_cycle, addressing_done);
            }
            ReadMemoryStep::SeparateMemoryAccess => {
                self.access_memory(cpu);
                self.step = ReadMemoryStep::Done;

                return (true, true);
            }
            ReadMemoryStep::Done => {
                return (false, true);
            }
        }
    }
}
