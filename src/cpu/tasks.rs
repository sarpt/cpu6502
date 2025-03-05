use std::{collections::VecDeque, rc::Rc};

use super::CPU;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskCycleVariant {
    Aborted,
    Partial,
    Full,
}

type ScheduledTask = Rc<dyn Fn(&mut CPU) -> TaskCycleVariant>;

pub struct Tasks {
    tasks_queue: VecDeque<ScheduledTask>,
}

impl Tasks {
    pub fn new() -> Self {
        return Tasks {
            tasks_queue: VecDeque::new(),
        };
    }

    pub fn push(&mut self, task: ScheduledTask) {
        self.tasks_queue.push_back(task);
    }

    pub fn append(&mut self, mut other: Tasks) {
        self.tasks_queue.append(&mut other.tasks_queue);
    }

    fn done(&self) -> bool {
        return self.tasks_queue.len() == 0;
    }

    pub fn tick(&mut self, cpu: &mut CPU) -> (bool, bool) {
        if self.done() {
            return (false, true);
        }

        let mut ran_tasks_count: usize = 0;
        let mut took_cycles: bool = false;
        for task_runner in &self.tasks_queue {
            ran_tasks_count += 1;
            let task_cycle_variant = task_runner(cpu);
            if task_cycle_variant == TaskCycleVariant::Full {
                took_cycles = true;
                break;
            };
        }

        for idx in (0..=ran_tasks_count - 1).rev() {
            self.tasks_queue.remove(idx);
        }

        return (took_cycles, self.done());
    }
}

impl Default for Tasks {
    fn default() -> Self {
        Self {
            tasks_queue: Default::default(),
        }
    }
}
