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

#[cfg(test)]
mod tests {
  #[cfg(test)]
  mod zero_page_addressing {
    use crate::cpu::{
      CPU,
      addressing::zero_page::ZeroPageAddressingTasks,
      tests::{MemoryMock, run_tasks},
    };

    #[test]
    fn should_return_address_in_zero_page_from_next_byte_in_memory_relative_to_program_counter() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x02;

      let mut tasks = Box::new(ZeroPageAddressingTasks::new());
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.addr.value(), Some(0x00CB));
    }

    #[test]
    fn should_advance_program_counter_once() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x02;

      let mut tasks = Box::new(ZeroPageAddressingTasks::new());
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.program_counter, 0x03);
    }

    #[test]
    fn should_take_one_cycle() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x02;
      cpu.cycle = 0;

      let mut tasks = Box::new(ZeroPageAddressingTasks::new());
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 1);
    }
  }

  #[cfg(test)]
  mod zero_page_x_addressing {
    use crate::cpu::{
      CPU,
      addressing::zero_page::ZeroPageOffsetAddressingTasks,
      tests::{MemoryMock, run_tasks},
    };

    #[test]
    fn should_return_address_in_zero_page_from_next_byte_in_memory_relative_to_program_counter_summed_with_index_register_x()
     {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x02;
      cpu.index_register_x = 0x03;

      let mut tasks = Box::new(ZeroPageOffsetAddressingTasks::new_offset_by_x());
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.addr.value(), Some(0x00CE));
    }

    #[test]
    fn should_advance_program_counter_once() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x02;
      cpu.index_register_x = 0x03;

      let mut tasks = Box::new(ZeroPageOffsetAddressingTasks::new_offset_by_x());
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.program_counter, 0x03);
    }

    #[test]
    fn should_take_two_cycles() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x02;
      cpu.index_register_x = 0x03;
      cpu.cycle = 0;

      let mut tasks = Box::new(ZeroPageOffsetAddressingTasks::new_offset_by_x());
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 2);
    }
  }

  #[cfg(test)]
  mod zero_page_y_addressing {
    use crate::cpu::{
      CPU,
      addressing::zero_page::ZeroPageOffsetAddressingTasks,
      tests::{MemoryMock, run_tasks},
    };

    #[test]
    fn should_return_address_in_zero_page_from_next_byte_in_memory_relative_to_program_counter_summed_with_index_register_y()
     {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x03;
      cpu.index_register_y = 0x03;

      let mut tasks = Box::new(ZeroPageOffsetAddressingTasks::new_offset_by_y());
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.addr.value(), Some(0x0055));
    }

    #[test]
    fn should_advance_program_counter_once() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x02;
      cpu.index_register_y = 0x03;

      let mut tasks = Box::new(ZeroPageOffsetAddressingTasks::new_offset_by_y());
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.program_counter, 0x03);
    }

    #[test]
    fn should_take_two_cycles() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x02;
      cpu.index_register_y = 0x03;
      cpu.cycle = 0;

      let mut tasks = Box::new(ZeroPageOffsetAddressingTasks::new_offset_by_y());
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 2);
    }
  }
}
