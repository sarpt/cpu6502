use crate::{
  cpu::{tasks::transfer_register::TransferRegistersTasks, Registers, Tasks, CPU},
  memory::Memory,
};

#[derive(PartialEq, PartialOrd)]
enum PushRegisterSteps {
  DummyFetch,
  PushToStack,
  Done,
}

struct PushRegisterTasks {
  register: Registers,
  step: PushRegisterSteps,
}

impl PushRegisterTasks {
  fn new(register: Registers) -> Self {
    PushRegisterTasks {
      register,
      step: PushRegisterSteps::DummyFetch,
    }
  }
}

impl Tasks for PushRegisterTasks {
  fn done(&self) -> bool {
    self.step == PushRegisterSteps::Done
  }

  fn tick(&mut self, cpu: &mut CPU, memory: &mut dyn Memory) -> bool {
    match self.step {
      PushRegisterSteps::DummyFetch => {
        cpu.dummy_fetch(memory);

        self.step = PushRegisterSteps::PushToStack;
        false
      }
      PushRegisterSteps::PushToStack => {
        let val = cpu.get_register(self.register);
        cpu.push_byte_to_stack(val, memory);

        self.step = PushRegisterSteps::Done;
        true
      }
      PushRegisterSteps::Done => {
        panic!("tick mustn't be called when done")
      }
    }
  }
}

fn push_register(_cpu: &mut CPU, register: Registers) -> Box<dyn Tasks> {
  Box::new(PushRegisterTasks::new(register))
}

pub fn pha(cpu: &mut CPU) -> Box<dyn Tasks> {
  push_register(cpu, Registers::Accumulator)
}

pub fn php(cpu: &mut CPU) -> Box<dyn Tasks> {
  push_register(cpu, Registers::ProcessorStatus)
}

#[derive(PartialEq, PartialOrd)]
enum PullRegisterSteps {
  DummyFetch,
  PreDecrementStackPointer,
  PullFromStack,
  Done,
}

struct PullRegisterTasks {
  register: Registers,
  step: PullRegisterSteps,
}

impl PullRegisterTasks {
  fn new(register: Registers) -> Self {
    PullRegisterTasks {
      register,
      step: PullRegisterSteps::DummyFetch,
    }
  }
}

impl Tasks for PullRegisterTasks {
  fn done(&self) -> bool {
    self.step == PullRegisterSteps::Done
  }

  fn tick(&mut self, cpu: &mut CPU, memory: &mut dyn Memory) -> bool {
    match self.step {
      PullRegisterSteps::DummyFetch => {
        cpu.dummy_fetch(memory);

        self.step = PullRegisterSteps::PreDecrementStackPointer;
        false
      }
      PullRegisterSteps::PreDecrementStackPointer => {
        // dummy tick, simulate separate stack pointer decrement
        // second cycle involves decrement of the stack pointer but poping byte from stack in third cycle does it in a single fn call
        // TODO: dont create dummy cycles, instead of decrementing and poping values in one call separate them into respective cycles
        self.step = PullRegisterSteps::PullFromStack;
        false
      }
      PullRegisterSteps::PullFromStack => {
        let value = cpu.pop_byte_from_stack(memory);
        cpu.set_register(self.register, value);

        self.step = PullRegisterSteps::Done;
        true
      }
      PullRegisterSteps::Done => {
        panic!("tick mustn't be called when done")
      }
    }
  }
}

fn pull_register(_cpu: &mut CPU, register: Registers) -> Box<dyn Tasks> {
  Box::new(PullRegisterTasks::new(register))
}

pub fn pla(cpu: &mut CPU) -> Box<dyn Tasks> {
  pull_register(cpu, Registers::Accumulator)
}

pub fn plp(cpu: &mut CPU) -> Box<dyn Tasks> {
  pull_register(cpu, Registers::ProcessorStatus)
}

pub fn tsx(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(TransferRegistersTasks::new(
    Registers::StackPointer,
    Registers::IndexX,
  ))
}

pub fn txs(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(TransferRegistersTasks::new(
    Registers::IndexX,
    Registers::StackPointer,
  ))
}

#[cfg(test)]
mod pha {

  use crate::cpu::{
    instructions::pha,
    tests::{run_tasks, MemoryMock},
    CPU,
  };

  #[test]
  fn should_push_accumulator_into_stack() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.stack_pointer = 0xFF;
    cpu.accumulator = 0xDE;

    let mut tasks = pha(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(memory[0x01FF], 0xDE);
  }

  #[test]
  fn should_take_two_cycles() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.accumulator = 0xDE;
    cpu.stack_pointer = 0xFF;
    cpu.cycle = 0;

    let mut tasks = pha(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 2);
  }
}

#[cfg(test)]
mod pla {

  use crate::cpu::{
    instructions::pla,
    tests::{run_tasks, MemoryMock},
    CPU,
  };

  #[test]
  fn should_pull_stack_into_accumulator() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.stack_pointer = 0xFE;
    memory[0x01FF] = 0xDE;
    cpu.accumulator = 0x00;

    let mut tasks = pla(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.accumulator, 0xDE);
  }

  #[test]
  fn should_take_three_cycles() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.stack_pointer = 0xFE;
    memory[0x01FF] = 0xDE;
    cpu.cycle = 0;

    let mut tasks = pla(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 3);
  }

  #[test]
  fn should_set_processor_status_based_on_accumulator_value() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.stack_pointer = 0xFE;
    memory[0x01FF] = 0xDE;
    cpu.processor_status = 0x00_u8.into();

    let mut tasks = pla(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b10000000);
  }
}

#[cfg(test)]
mod php {

  use crate::cpu::{
    instructions::php,
    tests::{run_tasks, MemoryMock},
    CPU,
  };

  #[test]
  fn should_push_processor_status_into_stack() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.processor_status = 0b10101010_u8.into();
    cpu.stack_pointer = 0xFF;

    let mut tasks = php(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(memory[0x01FF], 0b10101010);
  }

  #[test]
  fn should_take_two_cycles() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.processor_status = 0b10101010_u8.into();
    cpu.stack_pointer = 0xFF;
    cpu.cycle = 0;

    let mut tasks = php(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 2);
  }
}

#[cfg(test)]
mod plp {

  use crate::cpu::{
    instructions::plp,
    tests::{run_tasks, MemoryMock},
    CPU,
  };

  #[test]
  fn should_pull_stack_into_accumulator() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.stack_pointer = 0xFE;
    memory[0x01FF] = 0xDE;
    cpu.processor_status = 0x00_u8.into();

    let mut tasks = plp(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0xDE);
  }

  #[test]
  fn should_take_three_cycles() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.stack_pointer = 0xFE;
    memory[0x01FF] = 0xDE;
    cpu.processor_status = 0x00_u8.into();
    cpu.cycle = 0;

    let mut tasks = plp(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 3);
  }
}

#[cfg(test)]
mod txs {

  use crate::cpu::{
    instructions::txs,
    tests::{run_tasks, MemoryMock},
    CPU,
  };

  #[test]
  fn should_push_index_x_register_into_stack_pointer_register() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_x = 0xDE;

    let mut tasks = txs(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.stack_pointer, 0xDE);
  }

  #[test]
  fn should_take_one_cycle() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_x = 0xDE;
    cpu.cycle = 0;

    let mut tasks = txs(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }
}

#[cfg(test)]
mod tsx {

  use crate::cpu::{
    instructions::tsx,
    tests::{run_tasks, MemoryMock},
    CPU,
  };

  #[test]
  fn should_push_stack_pointer_into_index_x_register_register() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.stack_pointer = 0xDE;

    let mut tasks = tsx(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.index_register_x, 0xDE);
  }

  #[test]
  fn should_take_one_cycle() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.stack_pointer = 0xDE;
    cpu.cycle = 0;

    let mut tasks = tsx(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }

  #[test]
  fn should_set_processor_status_based_on_index_x_register_value() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.stack_pointer = 0xDE;
    cpu.processor_status = 0x00_u8.into();

    let mut tasks = tsx(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b10000000);
  }
}
