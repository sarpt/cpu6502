use super::CPU;

pub mod modify_memory;
pub mod modify_register;
pub mod read_memory;
pub mod transfer_register;

pub trait Tasks {
    fn done(&self) -> bool;
    fn tick(&mut self, cpu: &mut CPU) -> bool;
}

pub struct DummyTasks {}

impl Default for DummyTasks {
    fn default() -> Self {
        return DummyTasks {};
    }
}

impl Tasks for DummyTasks {
    fn done(&self) -> bool {
        panic!("done status on dummy task mustn't be checked")
    }

    fn tick(&mut self, _cpu: &mut CPU) -> bool {
        panic!("tick mustn't be called on dummy tasks")
    }
}
