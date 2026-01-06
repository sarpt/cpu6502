use crate::memory::Memory;

use super::CPU;

pub mod modify_memory;
pub mod modify_register;
pub mod read_memory;
pub mod transfer_register;

pub trait Tasks {
  fn done(&self) -> bool;
  fn tick(&mut self, cpu: &mut CPU, memory: &mut dyn Memory) -> bool;
}
