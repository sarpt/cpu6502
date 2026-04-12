use crate::{
  consts::Byte,
  cpu::{addressing::AddressingTasks, tasks::Tasks},
  memory::Memory,
};

use super::OffsetVariant;

pub struct ZeroPageAddressingTasks {
  done: bool,
}

impl ZeroPageAddressingTasks {
  pub fn new() -> Self {
    ZeroPageAddressingTasks { done: false }
  }
}

impl AddressingTasks for ZeroPageAddressingTasks {
  fn fetch_during_addressing(&self) -> bool {
    false
  }
}

impl Tasks for ZeroPageAddressingTasks {
  fn done(&self) -> bool {
    self.done
  }

  fn tick(&mut self, cpu: &mut super::CPU, memory: &mut dyn Memory) -> bool {
    if self.done {
      return self.done;
    }

    cpu.addr.reset(super::AddressingMode::ZeroPage);
    let addr: Byte = memory[cpu.program_counter];
    cpu.addr.set(addr);
    cpu.increment_program_counter();

    cpu.addr.done = true;
    self.done = true;
    self.done
  }
}

#[derive(Eq, PartialEq)]
enum ZeroPageOffsetStep {
  ZeroPageAccess,
  Offset,
  Done,
}

pub struct ZeroPageOffsetAddressingTasks {
  step: ZeroPageOffsetStep,
  variant: OffsetVariant,
}

impl ZeroPageOffsetAddressingTasks {
  pub fn new_offset_by_x() -> Self {
    ZeroPageOffsetAddressingTasks {
      step: ZeroPageOffsetStep::ZeroPageAccess,
      variant: OffsetVariant::X,
    }
  }

  pub fn new_offset_by_y() -> Self {
    ZeroPageOffsetAddressingTasks {
      step: ZeroPageOffsetStep::ZeroPageAccess,
      variant: OffsetVariant::Y,
    }
  }
}

impl AddressingTasks for ZeroPageOffsetAddressingTasks {
  fn fetch_during_addressing(&self) -> bool {
    false
  }
}

impl Tasks for ZeroPageOffsetAddressingTasks {
  fn done(&self) -> bool {
    self.step == ZeroPageOffsetStep::Done
  }

  fn tick(&mut self, cpu: &mut super::CPU, memory: &mut dyn Memory) -> bool {
    match self.step {
      ZeroPageOffsetStep::ZeroPageAccess => {
        match self.variant {
          OffsetVariant::X => cpu.addr.reset(super::AddressingMode::ZeroPageX),
          OffsetVariant::Y => cpu.addr.reset(super::AddressingMode::ZeroPageY),
        }

        let addr: Byte = memory[cpu.program_counter];
        cpu.addr.set(addr);
        cpu.increment_program_counter();
        self.step = ZeroPageOffsetStep::Offset;

        false
      }
      ZeroPageOffsetStep::Offset => {
        let offset: Byte = match self.variant {
          OffsetVariant::X => cpu.index_register_x,
          OffsetVariant::Y => cpu.index_register_y,
        };
        let addr_output = cpu
          .addr
          .value()
          .expect("unexpected lack of address at Offset step") as Byte;

        _ = memory[addr_output.into()]; // dummy fetch from address
        let final_address = addr_output.wrapping_add(offset);
        cpu.addr.set(final_address);

        cpu.addr.done = true;
        self.step = ZeroPageOffsetStep::Done;

        true
      }
      ZeroPageOffsetStep::Done => {
        panic!("tick mustn't be called when done")
      }
    }
  }
}
