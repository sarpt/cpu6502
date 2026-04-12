use crate::{
  cpu::{
    CPU, ChipVariant, Tasks,
    addressing::{absolute::AbsoluteAddressingTasks, indirect::IndirectAddressingTasks},
  },
  memory::Memory,
};

#[derive(PartialEq, PartialOrd)]
enum JsrSteps {
  Addressing,
  DecrementProgramCounterLo,
  DecrementProgramCounterHi,
  SetProgramCounter,
  Done,
}

struct JsrTasks {
  step: JsrSteps,
  addressing_tasks: Box<dyn Tasks>,
}

impl JsrTasks {
  pub fn new(addressing_tasks: Box<dyn Tasks>) -> Self {
    JsrTasks {
      step: JsrSteps::Addressing,
      addressing_tasks,
    }
  }
}

impl Tasks for JsrTasks {
  fn done(&self) -> bool {
    self.step == JsrSteps::Done
  }

  fn tick(&mut self, cpu: &mut CPU, memory: &mut dyn Memory) -> bool {
    match self.step {
      JsrSteps::Addressing => {
        let addr_done = self.addressing_tasks.tick(cpu, memory);
        if addr_done {
          self.step = JsrSteps::DecrementProgramCounterHi;
        }

        false
      }
      JsrSteps::DecrementProgramCounterHi => {
        let [_, ret_program_counter_hi] = cpu.program_counter.wrapping_sub(1).to_le_bytes();
        memory[cpu.get_stack_ptr_address()] = ret_program_counter_hi;
        cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(1);

        self.step = JsrSteps::DecrementProgramCounterLo;
        false
      }
      JsrSteps::DecrementProgramCounterLo => {
        let [ret_program_counter_lo, _] = cpu.program_counter.wrapping_sub(1).to_le_bytes();
        memory[cpu.get_stack_ptr_address()] = ret_program_counter_lo;
        cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(1);

        self.step = JsrSteps::SetProgramCounter;
        false
      }
      JsrSteps::SetProgramCounter => {
        cpu.program_counter = cpu
          .addr
          .value()
          .expect("unexpected lack of output address in SetProgramCounter step");

        self.step = JsrSteps::Done;
        true
      }
      JsrSteps::Done => {
        panic!("tick mustn't be called when done")
      }
    }
  }
}

pub fn jsr_a(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(JsrTasks::new(Box::new(AbsoluteAddressingTasks::new())))
}

#[derive(PartialEq, PartialOrd)]
enum RtsSteps {
  DummyFetch,
  PreDecrementStackPointer,
  PopProgramCounterLo,
  PopProgramCounterHi,
  IncrementProgramCounter,
  Done,
}

struct RtsTasks {
  step: RtsSteps,
}

impl RtsTasks {
  fn new() -> Self {
    RtsTasks {
      step: RtsSteps::DummyFetch,
    }
  }
}

impl Tasks for RtsTasks {
  fn done(&self) -> bool {
    self.step == RtsSteps::Done
  }

  fn tick(&mut self, cpu: &mut CPU, memory: &mut dyn Memory) -> bool {
    match self.step {
      RtsSteps::DummyFetch => {
        cpu.dummy_fetch(memory);
        self.step = RtsSteps::PreDecrementStackPointer;
        false
      }
      RtsSteps::PreDecrementStackPointer => {
        cpu.stack_pointer = cpu.stack_pointer.wrapping_add(1);
        self.step = RtsSteps::PopProgramCounterLo;
        false
      }
      RtsSteps::PopProgramCounterLo => {
        let stack_addr = cpu.get_stack_ptr_address();
        let lo = memory[stack_addr];
        cpu.set_program_counter_lo(lo);
        cpu.stack_pointer = cpu.stack_pointer.wrapping_add(1);
        self.step = RtsSteps::PopProgramCounterHi;
        false
      }
      RtsSteps::PopProgramCounterHi => {
        let stack_addr = cpu.get_stack_ptr_address();
        let hi = memory[stack_addr];
        cpu.set_program_counter_hi(hi);
        self.step = RtsSteps::IncrementProgramCounter;
        false
      }
      RtsSteps::IncrementProgramCounter => {
        cpu.increment_program_counter();
        self.step = RtsSteps::Done;
        true
      }
      RtsSteps::Done => {
        panic!("tick mustn't be called when done")
      }
    }
  }
}

pub fn rts(cpu: &mut CPU) -> Box<dyn Tasks> {
  cpu.addr.reset_implicit();
  Box::new(RtsTasks::new())
}

pub struct JmpTasks {
  addressing_tasks: Box<dyn Tasks>,
}

impl JmpTasks {
  fn new(addressing_tasks: Box<dyn Tasks>) -> Self {
    JmpTasks { addressing_tasks }
  }
}

impl Tasks for JmpTasks {
  fn done(&self) -> bool {
    self.addressing_tasks.done()
  }

  fn tick(&mut self, cpu: &mut CPU, memory: &mut dyn Memory) -> bool {
    if self.addressing_tasks.done() {
      return true;
    }

    let done = self.addressing_tasks.tick(cpu, memory);

    if done {
      cpu.program_counter = cpu
        .addr
        .value()
        .expect("unexpected lack of address in JmpTasks");
    }

    done
  }
}

pub fn jmp_a(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(JmpTasks::new(Box::new(AbsoluteAddressingTasks::new())))
}

pub fn jmp_in(cpu: &mut CPU) -> Box<dyn Tasks> {
  let addr_tasks = if cpu.chip_variant == ChipVariant::NMOS {
    Box::new(IndirectAddressingTasks::new_incorrect_addressing())
  } else {
    Box::new(IndirectAddressingTasks::new_fixed_addressing())
  };
  Box::new(JmpTasks::new(addr_tasks))
}

#[cfg(test)]
mod jsr_a {

  use crate::cpu::{
    CPU,
    instructions::jsr_a,
    tests::{MemoryMock, run_tasks},
  };

  #[test]
  fn should_fetch_address_pointed_by_program_counter_and_put_in_program_counter() {
    let mut memory = MemoryMock::new(&[0x44, 0x51, 0x88]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.stack_pointer = 0xFF;

    let mut tasks = jsr_a(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x5144);
  }

  #[test]
  fn should_save_program_counter_shifted_once_into_stack_pointer() {
    let mut memory = MemoryMock::new(&[0x44, 0x51, 0x88]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.stack_pointer = 0xFF;

    let mut tasks = jsr_a(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(memory[0x01FF], 0x00);
    assert_eq!(memory[0x01FE], 0x01);
  }

  #[test]
  fn should_decrement_stack_pointer_twice() {
    let mut memory = MemoryMock::new(&[0x44, 0x51, 0x88]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.stack_pointer = 0xFF;

    let mut tasks = jsr_a(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.stack_pointer, 0xFD);
  }

  #[test]
  fn should_take_five_cycles() {
    let mut memory = MemoryMock::new(&[0x44, 0x51, 0x88]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.cycle = 0;

    let mut tasks = jsr_a(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 5);
  }
}

#[cfg(test)]
mod rts {

  use crate::cpu::{
    CPU,
    instructions::rts,
    tests::{MemoryMock, run_tasks},
  };

  #[test]
  fn should_fetch_address_from_stack_and_put_it_in_program_counter_incremented_by_one() {
    let mut memory = MemoryMock::new(&[0x01, 0x02, 0x03]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    memory[0x01FF] = 0x44;
    memory[0x01FE] = 0x51;
    cpu.stack_pointer = 0xFD;

    let mut tasks = rts(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x4452);
  }

  #[test]
  fn should_increment_stack_pointer_twice() {
    let mut memory = MemoryMock::new(&[0x01, 0x02, 0x03]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    memory[0x01FF] = 0x44;
    memory[0x01FE] = 0x51;
    cpu.stack_pointer = 0xFD;

    let mut tasks = rts(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.stack_pointer, 0xFF);
  }

  #[test]
  fn should_take_five_cycles() {
    let mut memory = MemoryMock::new(&[0x01, 0x02, 0x03]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    memory[0x01FF] = 0x44;
    memory[0x01FE] = 0x51;
    cpu.stack_pointer = 0xFD;
    cpu.cycle = 0;

    let mut tasks = rts(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 5);
  }
}

#[cfg(test)]
mod jmp_a {

  use crate::cpu::{
    CPU,
    instructions::jmp_a,
    tests::{MemoryMock, run_tasks},
  };

  #[test]
  fn should_put_address_stored_in_memory_at_program_counter_as_a_new_program_counter() {
    let mut memory = MemoryMock::new(&[0x44, 0x51, 0x88]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;

    let mut tasks = jmp_a(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x5144);
  }

  #[test]
  fn should_take_two_cycles() {
    let mut memory = MemoryMock::new(&[0x44, 0x51, 0x88]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.cycle = 0;

    let mut tasks = jmp_a(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 2);
  }
}

#[cfg(test)]
mod jmp_in {
  #[cfg(test)]
  mod common {

    use crate::cpu::{
      CPU,
      instructions::jmp_in,
      tests::{MemoryMock, run_tasks},
    };

    #[test]
    fn should_fetch_indirect_address_from_memory_and_put_in_program_counter() {
      let mut memory = MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;

      let mut tasks = jmp_in(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.program_counter, 0x0001);
    }
  }

  #[cfg(test)]
  mod nmos {

    use crate::cpu::{
      CPU,
      instructions::jmp_in,
      tests::{MemoryMock, run_tasks},
    };

    #[test]
    fn should_take_four_cycles() {
      let mut memory = MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = jmp_in(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }

  #[cfg(test)]
  mod cmos {

    use crate::cpu::{
      CPU,
      instructions::jmp_in,
      tests::{MemoryMock, run_tasks},
    };

    #[test]
    fn should_take_five_cycles() {
      let mut memory = MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]);
      let mut cpu = CPU::new_rockwell_cmos();
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = jmp_in(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }
}
