pub mod absolute;
pub mod address;
pub mod indirect;
pub mod zero_page;

use strum::Display;

use super::{CPU, tasks::Tasks};

#[derive(Debug, Copy, Clone, PartialEq, Display)]
pub enum AddressingMode {
  Absolute,
  AbsoluteX,
  AbsoluteY,
  Accumulator,
  Immediate,
  Implicit,
  IndexIndirectX,
  Indirect,
  IndirectIndexY,
  Relative,
  ZeroPage,
  ZeroPageX,
  ZeroPageY,
}

pub enum OffsetVariant {
  X,
  Y,
}

pub trait AddressingTasks: Tasks {
  fn fetch_during_addressing(&self) -> bool;
}
