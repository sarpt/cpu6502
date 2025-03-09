use std::{collections::VecDeque, rc::Rc};

use super::CPU;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskCycleVariant {
    Aborted,
    Partial,
    Full,
}

type ScheduledTask = Rc<dyn Fn(&mut CPU) -> TaskCycleVariant>;

pub trait Tasks {
    fn done(&self) -> bool;
    fn tick(&mut self, cpu: &mut CPU) -> (bool, bool);
}

pub struct GenericTasks {
    tasks_queue: VecDeque<ScheduledTask>,
}

impl GenericTasks {
    pub fn new() -> Self {
        return GenericTasks {
            tasks_queue: VecDeque::new(),
        };
    }

    pub fn push(&mut self, task: ScheduledTask) -> () {
        self.tasks_queue.push_back(task);
    }

    pub fn transfer_queue(&mut self, other: GenericTasks) -> () {
        self.tasks_queue.append(&mut other.collect());
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
            tasks_queue: Default::default(),
        }
    }
}
