use std::{collections::VecDeque, rc::Rc};

use super::CPU;

pub mod modify_memory;
pub mod read_memory;

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
