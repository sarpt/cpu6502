use crate::{
  consts::Byte,
  cpu::{tasks::read_memory::ReadMemoryTasks, AddressingMode, Registers, Tasks, CPU},
  memory::Memory,
};

fn compare(
  cpu: &mut CPU,
  addr_mode: Option<AddressingMode>,
  register: Registers,
) -> Box<dyn Tasks> {
  let read_memory_tasks = cpu.read_memory(addr_mode);
  Box::new(CompareTasks::new(read_memory_tasks, register))
}

struct CompareTasks {
  done: bool,
  read_memory_tasks: Box<dyn ReadMemoryTasks>,
  register: Registers,
}

impl CompareTasks {
  pub fn new(read_memory_tasks: Box<dyn ReadMemoryTasks>, register: Registers) -> Self {
    CompareTasks {
      read_memory_tasks,
      done: false,
      register,
    }
  }
}

impl Tasks for CompareTasks {
  fn done(&self) -> bool {
    self.done
  }

  fn tick(&mut self, cpu: &mut CPU, memory: &mut dyn Memory) -> bool {
    if self.done {
      panic!("tick mustn't be called when done")
    }

    if !self.read_memory_tasks.done() && !self.read_memory_tasks.tick(cpu, memory) {
      return false;
    }

    let value = match self.read_memory_tasks.value() {
      Some(ctx) => ctx.to_le_bytes()[0],
      None => panic!("unexpected lack of value after memory read"),
    };
    cpu.set_cmp_status(self.register, value);
    self.done = true;

    true
  }
}

pub fn cmp_im(cpu: &mut CPU) -> Box<dyn Tasks> {
  compare(cpu, None, Registers::Accumulator)
}

pub fn cmp_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
  compare(cpu, Some(AddressingMode::ZeroPage), Registers::Accumulator)
}

pub fn cmp_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
  compare(cpu, Some(AddressingMode::ZeroPageX), Registers::Accumulator)
}

pub fn cmp_a(cpu: &mut CPU) -> Box<dyn Tasks> {
  compare(cpu, Some(AddressingMode::Absolute), Registers::Accumulator)
}

pub fn cmp_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
  compare(cpu, Some(AddressingMode::AbsoluteX), Registers::Accumulator)
}

pub fn cmp_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
  compare(cpu, Some(AddressingMode::AbsoluteY), Registers::Accumulator)
}

pub fn cmp_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
  compare(
    cpu,
    Some(AddressingMode::IndexIndirectX),
    Registers::Accumulator,
  )
}

pub fn cmp_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
  compare(
    cpu,
    Some(AddressingMode::IndirectIndexY),
    Registers::Accumulator,
  )
}

pub fn cpx_im(cpu: &mut CPU) -> Box<dyn Tasks> {
  compare(cpu, None, Registers::IndexX)
}

pub fn cpx_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
  compare(cpu, Some(AddressingMode::ZeroPage), Registers::IndexX)
}

pub fn cpx_a(cpu: &mut CPU) -> Box<dyn Tasks> {
  compare(cpu, Some(AddressingMode::Absolute), Registers::IndexX)
}

pub fn cpy_im(cpu: &mut CPU) -> Box<dyn Tasks> {
  compare(cpu, None, Registers::IndexY)
}

pub fn cpy_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
  compare(cpu, Some(AddressingMode::ZeroPage), Registers::IndexY)
}

pub fn cpy_a(cpu: &mut CPU) -> Box<dyn Tasks> {
  compare(cpu, Some(AddressingMode::Absolute), Registers::IndexY)
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FlagOp {
  Unchanged,
  Set,
  Clear,
}

fn adc(val: Byte, acc: Byte, _carry: bool) -> (Byte, FlagOp, FlagOp) {
  let (result, carry) = acc.overflowing_add(val);
  // if a sign (0x80) of a result differs from signs of both inputs
  let overflow = (acc ^ result) & (val ^ result) & 0x80 > 0;

  let carry_op = if carry {
    FlagOp::Set
  } else {
    FlagOp::Unchanged
  };
  let overflow_op = if overflow {
    FlagOp::Set
  } else {
    FlagOp::Unchanged
  };
  (result, carry_op, overflow_op)
}

fn sbc(val: Byte, acc: Byte, carry: bool) -> (Byte, FlagOp, FlagOp) {
  let (result, carry) = acc.overflowing_add(0xFF - val + (carry as u8));
  // if a sign (0x80) of a result differs from sign of accumulator
  // and ones-complement of value sign differs from sign of result
  let overflow = (acc ^ result) & ((0xFF - val) ^ result) & 0x80 > 0;

  let carry_op = if carry {
    FlagOp::Clear
  } else {
    FlagOp::Unchanged
  };
  let overflow_op = if overflow {
    FlagOp::Set
  } else {
    FlagOp::Unchanged
  };
  (result, carry_op, overflow_op)
}

struct OperationsWithCarryTasks {
  done: bool,
  read_memory_tasks: Box<dyn ReadMemoryTasks>,
  op: fn(val: Byte, acc: Byte, carry: bool) -> (Byte, FlagOp, FlagOp),
}

impl OperationsWithCarryTasks {
  pub fn new(
    read_memory_tasks: Box<dyn ReadMemoryTasks>,
    op: fn(val: Byte, acc: Byte, carry: bool) -> (Byte, FlagOp, FlagOp),
  ) -> Self {
    OperationsWithCarryTasks {
      done: false,
      read_memory_tasks,
      op,
    }
  }
}

impl Tasks for OperationsWithCarryTasks {
  fn done(&self) -> bool {
    self.done
  }

  fn tick(&mut self, cpu: &mut CPU, memory: &mut dyn Memory) -> bool {
    if self.done {
      panic!("tick mustn't be called when done")
    }

    if !self.read_memory_tasks.done() && !self.read_memory_tasks.tick(cpu, memory) {
      return false;
    }

    let value = match self.read_memory_tasks.value() {
      Some(ctx) => ctx.to_le_bytes()[0],
      None => panic!("unexpected lack of value after memory read"),
    };
    let accumulator = cpu.get_register(Registers::Accumulator);
    let (value, carry, overflow) =
      (self.op)(value, accumulator, cpu.processor_status.get_carry_flag());

    cpu.set_register(Registers::Accumulator, value);

    if carry != FlagOp::Unchanged {
      cpu.processor_status.change_carry_flag(carry == FlagOp::Set)
    }
    if overflow != FlagOp::Unchanged {
      cpu
        .processor_status
        .change_overflow_flag(overflow == FlagOp::Set)
    }
    self.done = true;

    self.done
  }
}

pub fn operations_with_carry(
  cpu: &mut CPU,
  addr_mode: Option<AddressingMode>,
  op: fn(val: Byte, acc: Byte, carry: bool) -> (Byte, FlagOp, FlagOp),
) -> Box<dyn Tasks> {
  let read_memory_tasks = cpu.read_memory(addr_mode);
  Box::new(OperationsWithCarryTasks::new(read_memory_tasks, op))
}

pub fn adc_im(cpu: &mut CPU) -> Box<dyn Tasks> {
  operations_with_carry(cpu, None, adc)
}

pub fn adc_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
  operations_with_carry(cpu, Some(AddressingMode::ZeroPage), adc)
}

pub fn adc_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
  operations_with_carry(cpu, Some(AddressingMode::ZeroPageX), adc)
}

pub fn adc_a(cpu: &mut CPU) -> Box<dyn Tasks> {
  operations_with_carry(cpu, Some(AddressingMode::Absolute), adc)
}

pub fn adc_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
  operations_with_carry(cpu, Some(AddressingMode::AbsoluteX), adc)
}

pub fn adc_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
  operations_with_carry(cpu, Some(AddressingMode::AbsoluteY), adc)
}

pub fn adc_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
  operations_with_carry(cpu, Some(AddressingMode::IndexIndirectX), adc)
}

pub fn adc_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
  operations_with_carry(cpu, Some(AddressingMode::IndirectIndexY), adc)
}

pub fn sbc_im(cpu: &mut CPU) -> Box<dyn Tasks> {
  operations_with_carry(cpu, None, sbc)
}

pub fn sbc_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
  operations_with_carry(cpu, Some(AddressingMode::ZeroPage), sbc)
}

pub fn sbc_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
  operations_with_carry(cpu, Some(AddressingMode::ZeroPageX), sbc)
}

pub fn sbc_a(cpu: &mut CPU) -> Box<dyn Tasks> {
  operations_with_carry(cpu, Some(AddressingMode::Absolute), sbc)
}

pub fn sbc_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
  operations_with_carry(cpu, Some(AddressingMode::AbsoluteX), sbc)
}

pub fn sbc_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
  operations_with_carry(cpu, Some(AddressingMode::AbsoluteY), sbc)
}

pub fn sbc_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
  operations_with_carry(cpu, Some(AddressingMode::IndexIndirectX), sbc)
}

pub fn sbc_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
  operations_with_carry(cpu, Some(AddressingMode::IndirectIndexY), sbc)
}

#[cfg(test)]
mod cmp {
  #[cfg(test)]
  mod cmp_im {
    use crate::cpu::{
      instructions::cmp_im,
      tests::{run_tasks, MemoryMock},
      CPU,
    };

    #[test]
    fn should_compare_accumulator_with_next_byte_from_memory() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.program_counter = 0x00;
      assert_eq!(cpu.processor_status, 0b00000000);

      let mut tasks = cmp_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b10000000);
    }

    #[test]
    fn should_take_one_cycle() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = cmp_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 1);
    }
  }

  #[cfg(test)]
  mod cmp_zp {
    use crate::cpu::{
      instructions::cmp_zp,
      tests::{run_tasks, MemoryMock},
      CPU,
    };

    #[test]
    fn should_compare_accumulator_with_a_value_from_a_zero_page_address() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, 0x04]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.program_counter = 0x00;
      assert_eq!(cpu.processor_status, 0b00000000);

      let mut tasks = cmp_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b10000000);
    }

    #[test]
    fn should_take_two_cycles() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, 0x04]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = cmp_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 2);
    }
  }

  #[cfg(test)]
  mod cmp_zpx {
    use crate::cpu::{
      instructions::cmp_zpx,
      tests::{run_tasks, MemoryMock},
      CPU,
    };

    #[test]
    fn should_compare_accumulator_with_a_value_from_a_zero_page_summed_with_index_register_x() {
      let mut memory = MemoryMock::new(&[0x01, 0x00, 0x00, 0x03]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;
      assert_eq!(cpu.processor_status, 0b00000000);

      let mut tasks = cmp_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b10000000);
    }

    #[test]
    fn should_take_three_cycles() {
      let mut memory = MemoryMock::new(&[0x01, 0x00, 0x00, 0x03]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = cmp_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }
  }

  #[cfg(test)]
  mod cmp_a {
    use crate::cpu::{
      instructions::cmp_a,
      tests::{run_tasks, MemoryMock},
      CPU,
    };

    #[test]
    fn should_compare_accumulator_with_a_value_from_an_address() {
      let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, 0x03]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.program_counter = 0x00;
      assert_eq!(cpu.processor_status, 0b00000000);

      let mut tasks = cmp_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b10000000);
    }

    #[test]
    fn should_take_three_cycles() {
      let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, 0x03]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = cmp_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }
  }

  #[cfg(test)]
  mod cmp_ax {
    use crate::{
      consts::Byte,
      cpu::{
        instructions::cmp_ax,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };

    const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
    const ADDRESS_LO: Byte = 0x03;
    const ADDRESS_HI: Byte = 0x00;
    const VALUE: Byte = 0x03;

    #[test]
    fn should_compare_accumulator_with_a_value_stored_in_address_ofset_by_x_register() {
      let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.program_counter = 0x00;
      cpu.index_register_x = 0x02;
      assert_eq!(cpu.processor_status, 0b00000000);

      let mut tasks = cmp_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b10000000);
    }

    #[test]
    fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.program_counter = 0x00;
      cpu.index_register_x = 0x02;
      cpu.cycle = 0;

      let mut tasks = cmp_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }

    #[test]
    fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[
        ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY,
        ADDRESS_HI,
        0x45,
        0xAF,
        0xDD,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.program_counter = 0x00;
      cpu.index_register_x = 0x02;
      cpu.cycle = 0;

      let mut tasks = cmp_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }

  #[cfg(test)]
  mod cmp_ay {
    use crate::{
      consts::Byte,
      cpu::{
        instructions::cmp_ay,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };

    const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
    const ADDRESS_LO: Byte = 0x03;
    const ADDRESS_HI: Byte = 0x00;
    const VALUE: Byte = 0x03;

    #[test]
    fn should_compare_accumulator_with_a_value_stored_in_address_ofset_by_y_register() {
      let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.program_counter = 0x00;
      cpu.index_register_y = 0x02;
      assert_eq!(cpu.processor_status, 0b00000000);

      let mut tasks = cmp_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b10000000);
    }

    #[test]
    fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.program_counter = 0x00;
      cpu.index_register_y = 0x02;
      cpu.cycle = 0;

      let mut tasks = cmp_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }

    #[test]
    fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[
        ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY,
        ADDRESS_HI,
        0x45,
        0xAF,
        0xDD,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.program_counter = 0x00;
      cpu.index_register_y = 0x02;
      cpu.cycle = 0;

      let mut tasks = cmp_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }

  #[cfg(test)]
  mod cmp_iny {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::cmp_iny,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };

    const INDIRECT_ZERO_PAGE_ADDRESS_PLACE: Byte = 0x01;
    const ADDRESS_LO: Byte = 0x03;
    const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
    const ADDRESS_HI: Byte = 0x00;
    const VALUE: Byte = 0x03;

    #[test]
    fn should_compare_accumulator_with_a_value_from_an_indirect_adress_stored_in_memory_at_zero_page_and_offset_with_value_from_index_register_y(
    ) {
      let mut memory = MemoryMock::new(&[
        INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
        ADDRESS_LO,
        ADDRESS_HI,
        0x45,
        0xAF,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      assert_eq!(cpu.processor_status, 0b00000000);

      let mut tasks = cmp_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b10000000);
    }

    #[test]
    fn should_take_four_cycles_when_summing_indirect_address_with_index_y_does_not_cross_page_flip()
    {
      let mut memory = MemoryMock::new(&[
        INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
        ADDRESS_LO,
        ADDRESS_HI,
        0x45,
        0xAF,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = cmp_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }

    #[test]
    fn should_take_five_cycles_when_summing_indirect_address_with_index_y_crosses_page_flip() {
      let mut payload: [Byte; 512] = [0x00; 512];
      payload[0x0000] = INDIRECT_ZERO_PAGE_ADDRESS_PLACE;
      payload[0x0001] = ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY;
      payload[0x0002] = ADDRESS_HI;
      payload[0x0101] = VALUE;

      let mut memory = MemoryMock::new(&payload);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = cmp_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }
}

#[cfg(test)]
mod cpy {
  #[cfg(test)]
  mod cpy_im {

    use crate::cpu::{
      instructions::cpy_im,
      tests::{run_tasks, MemoryMock},
      CPU,
    };

    #[test]
    fn should_compare_y_register_with_next_byte_from_memory() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      assert_eq!(cpu.processor_status, 0b00000000);

      let mut tasks = cpy_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b10000000);
    }

    #[test]
    fn should_take_one_cycle() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = cpy_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 1);
    }
  }

  #[cfg(test)]
  mod cpy_zp {

    use crate::cpu::{
      instructions::cpy_zp,
      tests::{run_tasks, MemoryMock},
      CPU,
    };

    #[test]
    fn should_compare_y_register_with_a_value_from_a_zero_page_address() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, 0x04]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      assert_eq!(cpu.processor_status, 0b00000000);

      let mut tasks = cpy_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b10000000);
    }

    #[test]
    fn should_take_two_cycles() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, 0x04]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = cpy_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 2);
    }
  }

  #[cfg(test)]
  mod cpy_a {

    use crate::cpu::{
      instructions::cpy_a,
      tests::{run_tasks, MemoryMock},
      CPU,
    };

    #[test]
    fn should_compare_y_register_with_a_value_from_an_address() {
      let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, 0x03]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      assert_eq!(cpu.processor_status, 0b00000000);

      let mut tasks = cpy_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b10000000);
    }

    #[test]
    fn should_take_three_cycles() {
      let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, 0x03]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = cpy_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }
  }
}

#[cfg(test)]
mod cpx {
  #[cfg(test)]
  mod cpx_im {

    use crate::cpu::{
      instructions::cpx_im,
      tests::{run_tasks, MemoryMock},
      CPU,
    };

    #[test]
    fn should_compare_x_register_with_next_byte_from_memory() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;
      assert_eq!(cpu.processor_status, 0b00000000);

      let mut tasks = cpx_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b10000000);
    }

    #[test]
    fn should_take_one_cycle() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = cpx_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 1);
    }
  }

  #[cfg(test)]
  mod cpx_zp {

    use crate::cpu::{
      instructions::cpx_zp,
      tests::{run_tasks, MemoryMock},
      CPU,
    };

    #[test]
    fn should_compare_x_register_with_a_value_from_a_zero_page_address() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, 0x04]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;
      assert_eq!(cpu.processor_status, 0b00000000);

      let mut tasks = cpx_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b10000000);
    }

    #[test]
    fn should_take_two_cycles() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, 0x04]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = cpx_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 2);
    }
  }

  #[cfg(test)]
  mod cpx_a {

    use crate::cpu::{
      instructions::cpx_a,
      tests::{run_tasks, MemoryMock},
      CPU,
    };

    #[test]
    fn should_compare_x_register_with_a_value_from_an_address() {
      let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, 0x03]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;
      assert_eq!(cpu.processor_status, 0b00000000);

      let mut tasks = cpx_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b10000000);
    }

    #[test]
    fn should_take_three_cycles() {
      let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, 0x03]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = cpx_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }
  }
}

#[cfg(test)]
mod adc {
  #[cfg(test)]
  mod common {
    use crate::cpu::instructions::{arithmetic::adc, FlagOp};

    #[test]
    fn should_return_sum_with_carry_as_unchanged_when_sum_does_not_overflows_a_byte() {
      let memory_value = 0x05;
      let acc_value = 0xF0;
      let initial_carry = false;
      let (value, carry, _overflow) = adc(memory_value, acc_value, initial_carry);

      assert_eq!(value, 0xF5);
      assert_eq!(carry, FlagOp::Unchanged);
    }

    #[test]
    fn should_return_sum_with_carry_as_set_when_sum_overflows_a_byte() {
      let memory_value = 0x05;
      let acc_value = 0xFE;
      let initial_carry = false;
      let (value, carry, _overflow) = adc(memory_value, acc_value, initial_carry);

      assert_eq!(value, 0x03);
      assert_eq!(carry, FlagOp::Set);
    }

    #[test]
    fn should_return_sum_with_overflow_as_unchanged_when_sum_result_and_both_inputs_are_unsigned() {
      let memory_value = 0x05;
      let acc_value = 0x03;
      let initial_carry = false;
      let (value, _carry, overflow) = adc(memory_value, acc_value, initial_carry);

      assert_eq!(value, 0x08);
      assert_eq!(overflow, FlagOp::Unchanged);
    }

    #[test]
    fn should_return_sum_with_overflow_as_unchanged_when_sum_result_is_unsigned_and_one_of_the_inputs_is_signed(
    ) {
      let memory_value = 0x50;
      let acc_value = 0xd0;
      let initial_carry = false;
      let (value, _carry, overflow) = adc(memory_value, acc_value, initial_carry);

      assert_eq!(value, 0x20);
      assert_eq!(overflow, FlagOp::Unchanged);
    }

    #[test]
    fn should_return_sum_with_overflow_as_unchanged_when_sum_result_is_signed_and_one_of_the_inputs_is_unsigned(
    ) {
      let memory_value = 0x50;
      let acc_value = 0x90;
      let initial_carry = false;
      let (value, _carry, overflow) = adc(memory_value, acc_value, initial_carry);

      assert_eq!(value, 0xe0);
      assert_eq!(overflow, FlagOp::Unchanged);
    }

    #[test]
    fn should_return_sum_with_overflow_as_unchanged_when_sum_result_and_both_inputs_are_signed() {
      let memory_value = 0xd0;
      let acc_value = 0xd0;
      let initial_carry = false;
      let (value, _carry, overflow) = adc(memory_value, acc_value, initial_carry);

      assert_eq!(value, 0xa0);
      assert_eq!(overflow, FlagOp::Unchanged);
    }

    #[test]
    fn should_return_sum_with_overflow_as_set_when_sum_result_is_signed_but_both_inputs_are_unsigned(
    ) {
      let memory_value = 0x50;
      let acc_value = 0x50;
      let initial_carry = false;
      let (value, _carry, overflow) = adc(memory_value, acc_value, initial_carry);

      assert_eq!(value, 0xa0);
      assert_eq!(overflow, FlagOp::Set);
    }

    #[test]
    fn should_return_sum_with_overflow_as_set_when_sum_result_is_unsigned_but_both_inputs_are_signed(
    ) {
      let memory_value = 0xd0;
      let acc_value = 0x90;
      let initial_carry = false;
      let (value, _carry, overflow) = adc(memory_value, acc_value, initial_carry);

      assert_eq!(value, 0x60);
      assert_eq!(overflow, FlagOp::Set);
    }
  }

  #[cfg(test)]
  mod adc_im {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::adc_im,
        processor_status::ProcessorStatus,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };
    const VALUE: Byte = 0x03;

    #[test]
    fn should_sum_accumulator_with_next_byte_from_memory() {
      let mut memory = MemoryMock::new(&[VALUE, 0xFF]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.program_counter = 0x00;

      let mut tasks = adc_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x05);
    }

    #[test]
    fn should_set_overflow_and_carry_flags() {
      const VALUE: Byte = 0x90;
      let mut memory = MemoryMock::new(&[VALUE, 0xFF]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0xd0;
      cpu.program_counter = 0x00;
      cpu.processor_status = ProcessorStatus::from(0b00000000);

      let mut tasks = adc_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000001);
    }

    #[test]
    fn should_take_one_cycle() {
      let mut memory = MemoryMock::new(&[VALUE, 0xFF]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = adc_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 1);
    }
  }

  #[cfg(test)]
  mod adc_zp {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::adc_zp,
        processor_status::ProcessorStatus,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };
    const VALUE: Byte = 0x03;

    #[test]
    fn should_sum_accumulator_with_a_value_stored_in_a_zero_page_address() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x02;

      let mut tasks = adc_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x05);
    }

    #[test]
    fn should_set_overflow_and_carry_flags() {
      const VALUE: Byte = 0x90;
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0xd0;
      cpu.program_counter = 0x00;
      cpu.processor_status = ProcessorStatus::from(0b00000000);

      let mut tasks = adc_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000001);
    }

    #[test]
    fn should_take_two_cycles() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = adc_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 2);
    }
  }

  #[cfg(test)]
  mod adc_zpx {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::adc_zpx,
        processor_status::ProcessorStatus,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };
    const VALUE: Byte = 0x03;

    #[test]
    fn should_sum_accumulator_with_value_stored_in_zero_page_summed_with_index_register_x() {
      let mut memory = MemoryMock::new(&[0x01, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x02;

      let mut tasks = adc_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x05);
    }

    #[test]
    fn should_set_overflow_and_carry_flags() {
      const VALUE: Byte = 0x90;
      let mut memory = MemoryMock::new(&[0x01, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;
      cpu.accumulator = 0xd0;
      cpu.processor_status = ProcessorStatus::from(0b00000000);

      let mut tasks = adc_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000001);
    }

    #[test]
    fn should_take_three_cycles() {
      let mut memory = MemoryMock::new(&[0x01, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x02;
      cpu.cycle = 0;

      let mut tasks = adc_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }
  }

  #[cfg(test)]
  mod adc_a {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::adc_a,
        processor_status::ProcessorStatus,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };
    const VALUE: Byte = 0x03;

    #[test]
    fn should_sum_accumulalator_with_a_value_from_absolute_address() {
      let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x02;

      let mut tasks = adc_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x05);
    }

    #[test]
    fn should_set_overflow_and_carry_flags() {
      const VALUE: Byte = 0x90;
      let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0xd0;
      cpu.processor_status = ProcessorStatus::from(0b00000000);

      let mut tasks = adc_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000001);
    }

    #[test]
    fn should_take_three_cycles() {
      let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x02;
      cpu.cycle = 0;

      let mut tasks = adc_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }
  }

  #[cfg(test)]
  mod adc_ax {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::adc_ax,
        processor_status::ProcessorStatus,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };

    const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
    const ADDRESS_LO: Byte = 0x03;
    const ADDRESS_HI: Byte = 0x00;
    const VALUE: Byte = 0x03;

    #[test]
    fn should_sum_accumulator_with_a_value_in_absolute_address_offset_by_index_register_x() {
      let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.index_register_x = 0x02;
      cpu.accumulator = 0x02;

      let mut tasks = adc_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x05);
    }

    #[test]
    fn should_set_overflow_and_carry_flags() {
      const VALUE: Byte = 0x90;
      let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.index_register_x = 0x02;
      cpu.accumulator = 0xd0;
      cpu.processor_status = ProcessorStatus::from(0b00000000);

      let mut tasks = adc_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000001);
    }

    #[test]
    fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.index_register_x = 0x02;
      cpu.accumulator = 0x02;
      cpu.cycle = 0;

      let mut tasks = adc_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }

    #[test]
    fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut payload: [Byte; 512] = [0x00; 512];
      payload[0x0000] = ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY;
      payload[0x0001] = ADDRESS_HI;
      payload[0x0101] = VALUE;

      let mut memory = MemoryMock::new(&payload);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.index_register_x = 0x02;
      cpu.accumulator = 0x02;
      cpu.cycle = 0;

      let mut tasks = adc_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }

  #[cfg(test)]
  mod adc_ay {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::adc_ay,
        processor_status::ProcessorStatus,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };

    const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
    const ADDRESS_LO: Byte = 0x03;
    const ADDRESS_HI: Byte = 0x00;
    const VALUE: Byte = 0x03;

    #[test]
    fn should_sum_accumulator_with_value_in_an_absolute_address_offset_by_index_register_y() {
      let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.index_register_y = 0x02;
      cpu.accumulator = 0x02;

      let mut tasks = adc_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x05);
    }

    #[test]
    fn should_set_overflow_and_carry_flags() {
      const VALUE: Byte = 0x90;
      let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.index_register_y = 0x02;
      cpu.accumulator = 0xd0;
      cpu.processor_status = ProcessorStatus::from(0b00000000);

      let mut tasks = adc_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000001);
    }

    #[test]
    fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.index_register_y = 0x02;
      cpu.accumulator = 0x02;
      cpu.cycle = 0;

      let mut tasks = adc_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }

    #[test]
    fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut payload: [Byte; 512] = [0x00; 512];
      payload[0x0000] = ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY;
      payload[0x0001] = ADDRESS_HI;
      payload[0x0101] = VALUE;

      let mut memory = MemoryMock::new(&payload);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.index_register_y = 0x02;
      cpu.accumulator = 0x02;
      cpu.cycle = 0;

      let mut tasks = adc_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }

  #[cfg(test)]
  mod adc_iny {
    use crate::{
      consts::Byte,
      cpu::{
        instructions::adc_iny,
        processor_status::ProcessorStatus,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };

    const INDIRECT_ZERO_PAGE_ADDRESS_PLACE: Byte = 0x01;
    const ADDRESS_LO: Byte = 0x03;
    const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
    const ADDRESS_HI: Byte = 0x00;
    const VALUE: Byte = 0x03;

    #[test]
    fn should_sum_accumulator_with_a_value_from_an_indirect_adress_stored_in_memory_at_zero_page_and_offset_with_value_from_index_register_y(
    ) {
      let mut memory = MemoryMock::new(&[
        INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
        ADDRESS_LO,
        ADDRESS_HI,
        0x45,
        0xAF,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x02;

      let mut tasks = adc_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x05);
    }

    #[test]
    fn should_set_overflow_and_carry_flags() {
      const VALUE: Byte = 0x90;
      let mut memory = MemoryMock::new(&[
        INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
        ADDRESS_LO,
        ADDRESS_HI,
        0x45,
        0xAF,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.accumulator = 0xd0;
      cpu.processor_status = ProcessorStatus::from(0b00000000);

      let mut tasks = adc_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000001);
    }

    #[test]
    fn should_take_four_cycles_when_summing_indirect_address_with_index_y_does_not_cross_page_flip()
    {
      let mut memory = MemoryMock::new(&[
        INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
        ADDRESS_LO,
        ADDRESS_HI,
        0x45,
        0xAF,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x02;
      cpu.cycle = 0;

      let mut tasks = adc_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }

    #[test]
    fn should_take_five_cycles_when_summing_indirect_address_with_index_y_crosses_page_flip() {
      let mut payload: [Byte; 512] = [0x00; 512];
      payload[0x0000] = INDIRECT_ZERO_PAGE_ADDRESS_PLACE;
      payload[0x0001] = ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY;
      payload[0x0002] = ADDRESS_HI;
      payload[0x0101] = VALUE;

      let mut memory = MemoryMock::new(&payload);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x02;
      cpu.cycle = 0;

      let mut tasks = adc_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }

  #[cfg(test)]
  mod adc_inx {

    use crate::cpu::{
      instructions::adc_inx,
      processor_status::ProcessorStatus,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ZP_ADDRESS: Byte = 0x02;
    const OFFSET: Byte = 0x01;
    const EFFECTIVE_ADDRESS_LO: Byte = 0x05;
    const EFFECTIVE_ADDRESS_HI: Byte = 0x00;
    const VALUE: Byte = 0x03;

    #[test]
    fn should_sum_accumulator_with_a_value_in_an_indirect_adress_stored_in_zero_page_offset_with_index_register_x(
    ) {
      let mut memory = MemoryMock::new(&[
        ZP_ADDRESS,
        0x00,
        0x00,
        EFFECTIVE_ADDRESS_LO,
        EFFECTIVE_ADDRESS_HI,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x02;
      cpu.index_register_x = OFFSET;

      let mut tasks = adc_inx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x05);
    }

    #[test]
    fn should_set_overflow_and_carry_flags() {
      const VALUE: Byte = 0x90;
      let mut memory = MemoryMock::new(&[
        ZP_ADDRESS,
        0x00,
        0x00,
        EFFECTIVE_ADDRESS_LO,
        EFFECTIVE_ADDRESS_HI,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0xd0;
      cpu.index_register_x = OFFSET;
      cpu.processor_status = ProcessorStatus::from(0b00000000);

      let mut tasks = adc_inx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000001);
    }

    #[test]
    fn should_take_five_cycles() {
      let mut memory = MemoryMock::new(&[
        ZP_ADDRESS,
        0x00,
        0x00,
        EFFECTIVE_ADDRESS_LO,
        EFFECTIVE_ADDRESS_HI,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x02;
      cpu.index_register_x = OFFSET;
      cpu.cycle = 0;

      let mut tasks = adc_inx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }
}

#[cfg(test)]
mod sbc {
  #[cfg(test)]
  mod common {
    use crate::cpu::instructions::{arithmetic::sbc, FlagOp};

    #[test]
    fn should_return_subtraction_with_carry_as_clear_when_subtraction_overflows_a_byte() {
      let acc_value = 0x50;
      let memory_value = 0x30;
      let initial_carry = true;
      let (value, carry, _overflow) = sbc(memory_value, acc_value, initial_carry);

      assert_eq!(value, 0x20);
      assert_eq!(carry, FlagOp::Clear);
    }

    #[test]
    fn should_return_subtraction_with_carry_as_unchanged_when_subtraction_does_not_overflow_a_byte()
    {
      let acc_value = 0xd0;
      let memory_value = 0xf0;
      let initial_carry = false;
      let (value, carry, _overflow) = sbc(memory_value, acc_value, initial_carry);

      assert_eq!(value, 0xdf);
      assert_eq!(carry, FlagOp::Unchanged);
    }

    #[test]
    fn should_return_subtraction_with_overflow_unchanged_when_result_with_accumulator_and_ones_complement_of_value_are_unsigned(
    ) {
      let acc_value = 0x50;
      let memory_value = 0xf0;
      let initial_carry = true;
      let (value, _carry, overflow) = sbc(memory_value, acc_value, initial_carry);

      assert_eq!(value, 0x60);
      assert_eq!(overflow, FlagOp::Unchanged);
    }

    #[test]
    fn should_return_subtraction_with_overflow_unchanged_when_result_is_unsigned_with_ones_complement_of_value_unsigned_and_accumulator_is_signed(
    ) {
      let acc_value = 0xd0;
      let memory_value = 0xf0;
      let initial_carry = true;
      let (value, _carry, overflow) = sbc(memory_value, acc_value, initial_carry);

      assert_eq!(value, 0xe0);
      assert_eq!(overflow, FlagOp::Unchanged);
    }

    #[test]
    fn should_return_subtraction_with_overflow_unchanged_when_result_is_signed_with_ones_complement_of_value_signed_and_accumulator_is_unsigned(
    ) {
      let acc_value = 0x50;
      let memory_value = 0x70;
      let initial_carry = false;
      let (value, _carry, overflow) = sbc(memory_value, acc_value, initial_carry);

      assert_eq!(value, 0xdf);
      assert_eq!(overflow, FlagOp::Unchanged);
    }

    #[test]
    fn should_return_subtraction_with_overflow_set_as_unchanged_when_result_with_accumulator_and_ones_complement_of_value_are_signed(
    ) {
      let acc_value = 0xd0;
      let memory_value = 0x30;
      let initial_carry = true;
      let (value, _carry, overflow) = sbc(memory_value, acc_value, initial_carry);

      assert_eq!(value, 0xa0);
      assert_eq!(overflow, FlagOp::Unchanged);
    }

    #[test]
    fn should_return_subtraction_with_overflow_set_when_result_is_signed_but_both_accumulator_and_ones_complement_of_value_are_unsigned(
    ) {
      let acc_value = 0x50;
      let memory_value = 0xb0;
      let initial_carry = false;
      let (value, _carry, overflow) = sbc(memory_value, acc_value, initial_carry);

      assert_eq!(value, 0x9f);
      assert_eq!(overflow, FlagOp::Set);
    }

    #[test]
    fn should_return_subtraction_with_overflow_set_when_result_is_unsigned_but_both_accumulator_and_ones_complement_of_value_are_signed(
    ) {
      let acc_value = 0xd0;
      let memory_value = 0x70;
      let initial_carry = true;
      let (value, _carry, overflow) = sbc(memory_value, acc_value, initial_carry);

      assert_eq!(value, 0x60);
      assert_eq!(overflow, FlagOp::Set);
    }
  }

  #[cfg(test)]
  mod sbc_im {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::sbc_im,
        processor_status::ProcessorStatus,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };
    const VALUE: Byte = 0x30;

    #[test]
    fn should_sub_accumulator_with_next_byte_from_memory() {
      let mut memory = MemoryMock::new(&[VALUE, 0xFF]);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.accumulator = 0x50;
      cpu.program_counter = 0x00;

      let mut tasks = sbc_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x20);
    }

    #[test]
    fn should_set_overflow_and_clear_carry_flag() {
      const VALUE: Byte = 0x70;
      let mut memory = MemoryMock::new(&[VALUE, 0xFF]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0xd0;
      cpu.program_counter = 0x00;
      cpu.processor_status = ProcessorStatus::from(0b01000001);

      let mut tasks = sbc_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000000);
    }

    #[test]
    fn should_take_one_cycle() {
      let mut memory = MemoryMock::new(&[VALUE, 0xFF]);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.accumulator = 0x50;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = sbc_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 1);
    }
  }

  #[cfg(test)]
  mod sbc_zp {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::sbc_zp,
        processor_status::ProcessorStatus,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };
    const VALUE: Byte = 0x30;

    #[test]
    fn should_sub_accumulator_with_a_value_stored_in_a_zero_page_address() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x50;

      let mut tasks = sbc_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x20);
    }

    #[test]
    fn should_set_overflow_and_clear_carry_flag() {
      const VALUE: Byte = 0x70;
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0xd0;
      cpu.processor_status = ProcessorStatus::from(0b01000001);

      let mut tasks = sbc_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000000);
    }

    #[test]
    fn should_take_two_cycles() {
      let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.accumulator = 0x50;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = sbc_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 2);
    }
  }

  #[cfg(test)]
  mod sbc_zpx {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::sbc_zpx,
        processor_status::ProcessorStatus,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };
    const VALUE: Byte = 0x30;

    #[test]
    fn should_sub_accumulator_with_value_stored_in_zero_page_summed_with_index_register_x() {
      let mut memory = MemoryMock::new(&[0x01, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x50;

      let mut tasks = sbc_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x20);
    }

    #[test]
    fn should_set_overflow_and_clear_carry_flag() {
      const VALUE: Byte = 0x70;
      let mut memory = MemoryMock::new(&[0x01, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;
      cpu.accumulator = 0xd0;
      cpu.processor_status = ProcessorStatus::from(0b01000001);

      let mut tasks = sbc_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000000);
    }
    #[test]
    fn should_take_three_cycles() {
      let mut memory = MemoryMock::new(&[0x01, 0x00, 0x00, 0x55]);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x50;
      cpu.cycle = 0;

      let mut tasks = sbc_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }
  }

  #[cfg(test)]
  mod sbc_a {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::sbc_a,
        processor_status::ProcessorStatus,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };
    const VALUE: Byte = 0x30;

    #[test]
    fn should_sub_accumulalator_with_a_value_from_absolute_address() {
      let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x50;

      let mut tasks = sbc_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x20);
    }

    #[test]
    fn should_set_overflow_and_clear_carry_flag() {
      const VALUE: Byte = 0x70;
      let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0xd0;
      cpu.processor_status = ProcessorStatus::from(0b01000001);

      let mut tasks = sbc_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000000);
    }

    #[test]
    fn should_take_three_cycles() {
      let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x50;
      cpu.cycle = 0;

      let mut tasks = sbc_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }
  }

  #[cfg(test)]
  mod sbc_ax {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::sbc_ax,
        processor_status::ProcessorStatus,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };

    const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
    const ADDRESS_LO: Byte = 0x03;
    const ADDRESS_HI: Byte = 0x00;
    const VALUE: Byte = 0x30;

    #[test]
    fn should_sub_accumulator_with_a_value_in_absolute_address_offset_by_index_register_x() {
      let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.program_counter = 0x00;
      cpu.index_register_x = 0x02;
      cpu.accumulator = 0x50;

      let mut tasks = sbc_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x20);
    }

    #[test]
    fn should_set_overflow_and_clear_carry_flag() {
      const VALUE: Byte = 0x70;
      let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.index_register_x = 0x02;
      cpu.accumulator = 0xd0;
      cpu.processor_status = ProcessorStatus::from(0b01000001);

      let mut tasks = sbc_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000000);
    }

    #[test]
    fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.program_counter = 0x00;
      cpu.index_register_x = 0x02;
      cpu.accumulator = 0x50;
      cpu.cycle = 0;

      let mut tasks = sbc_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }

    #[test]
    fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut payload: [Byte; 512] = [0x00; 512];
      payload[0x0000] = ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY;
      payload[0x0001] = ADDRESS_HI;
      payload[0x0101] = VALUE;

      let mut memory = MemoryMock::new(&payload);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.program_counter = 0x00;
      cpu.index_register_x = 0x02;
      cpu.accumulator = 0x50;
      cpu.cycle = 0;

      let mut tasks = sbc_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }

  #[cfg(test)]
  mod sbc_ay {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::sbc_ay,
        processor_status::ProcessorStatus,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };

    const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
    const ADDRESS_LO: Byte = 0x03;
    const ADDRESS_HI: Byte = 0x00;
    const VALUE: Byte = 0x30;

    #[test]
    fn should_sub_accumulator_with_value_in_an_absolute_address_offset_by_index_register_y() {
      let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.program_counter = 0x00;
      cpu.index_register_y = 0x02;
      cpu.accumulator = 0x50;

      let mut tasks = sbc_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x20);
    }

    #[test]
    fn should_set_overflow_and_clear_carry_flag() {
      const VALUE: Byte = 0x70;
      let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.index_register_y = 0x02;
      cpu.accumulator = 0xd0;
      cpu.processor_status = ProcessorStatus::from(0b01000001);

      let mut tasks = sbc_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000000);
    }

    #[test]
    fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.program_counter = 0x00;
      cpu.index_register_y = 0x02;
      cpu.accumulator = 0x50;
      cpu.cycle = 0;

      let mut tasks = sbc_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }

    #[test]
    fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut payload: [Byte; 512] = [0x00; 512];
      payload[0x0000] = ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY;
      payload[0x0001] = ADDRESS_HI;
      payload[0x0101] = VALUE;

      let mut memory = MemoryMock::new(&payload);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.program_counter = 0x00;
      cpu.index_register_y = 0x02;
      cpu.accumulator = 0x50;
      cpu.cycle = 0;

      let mut tasks = sbc_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }

  #[cfg(test)]
  mod sbc_iny {
    use crate::{
      consts::Byte,
      cpu::{
        instructions::sbc_iny,
        processor_status::ProcessorStatus,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };

    const INDIRECT_ZERO_PAGE_ADDRESS_PLACE: Byte = 0x01;
    const ADDRESS_LO: Byte = 0x03;
    const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
    const ADDRESS_HI: Byte = 0x00;
    const VALUE: Byte = 0x30;

    #[test]
    fn should_sub_accumulator_with_a_value_from_an_indirect_adress_stored_in_memory_at_zero_page_and_offset_with_value_from_index_register_y(
    ) {
      let mut memory = MemoryMock::new(&[
        INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
        ADDRESS_LO,
        ADDRESS_HI,
        0x45,
        0xAF,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x50;

      let mut tasks = sbc_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x20);
    }

    #[test]
    fn should_set_overflow_and_clear_carry_flag() {
      const VALUE: Byte = 0x70;
      let mut memory = MemoryMock::new(&[
        INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
        ADDRESS_LO,
        ADDRESS_HI,
        0x45,
        0xAF,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.accumulator = 0xd0;
      cpu.processor_status = ProcessorStatus::from(0b01000001);

      let mut tasks = sbc_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000000);
    }

    #[test]
    fn should_take_four_cycles_when_summing_indirect_address_with_index_y_does_not_cross_page_flip()
    {
      let mut memory = MemoryMock::new(&[
        INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
        ADDRESS_LO,
        ADDRESS_HI,
        0x45,
        0xAF,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x50;
      cpu.cycle = 0;

      let mut tasks = sbc_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }

    #[test]
    fn should_take_five_cycles_when_summing_indirect_address_with_index_y_crosses_page_flip() {
      let mut payload: [Byte; 512] = [0x00; 512];
      payload[0x0000] = INDIRECT_ZERO_PAGE_ADDRESS_PLACE;
      payload[0x0001] = ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY;
      payload[0x0002] = ADDRESS_HI;
      payload[0x0101] = VALUE;

      let mut memory = MemoryMock::new(&payload);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x50;
      cpu.cycle = 0;

      let mut tasks = sbc_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }

  #[cfg(test)]
  mod sbc_inx {

    use crate::cpu::{
      instructions::sbc_inx,
      processor_status::ProcessorStatus,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ZP_ADDRESS: Byte = 0x02;
    const OFFSET: Byte = 0x01;
    const EFFECTIVE_ADDRESS_LO: Byte = 0x05;
    const EFFECTIVE_ADDRESS_HI: Byte = 0x00;
    const VALUE: Byte = 0x30;

    #[test]
    fn should_sub_accumulator_with_a_value_in_an_indirect_adress_stored_in_zero_page_offset_with_index_register_x(
    ) {
      let mut memory = MemoryMock::new(&[
        ZP_ADDRESS,
        0x00,
        0x00,
        EFFECTIVE_ADDRESS_LO,
        EFFECTIVE_ADDRESS_HI,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x50;
      cpu.index_register_x = OFFSET;

      let mut tasks = sbc_inx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x20);
    }

    #[test]
    fn should_set_overflow_and_clear_carry_flag() {
      const VALUE: Byte = 0x70;
      let mut memory = MemoryMock::new(&[
        ZP_ADDRESS,
        0x00,
        0x00,
        EFFECTIVE_ADDRESS_LO,
        EFFECTIVE_ADDRESS_HI,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0xd0;
      cpu.index_register_x = OFFSET;
      cpu.processor_status = ProcessorStatus::from(0b01000001);

      let mut tasks = sbc_inx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000000);
    }

    #[test]
    fn should_take_five_cycles() {
      let mut memory = MemoryMock::new(&[
        ZP_ADDRESS,
        0x00,
        0x00,
        EFFECTIVE_ADDRESS_LO,
        EFFECTIVE_ADDRESS_HI,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x50;
      cpu.index_register_x = OFFSET;
      cpu.cycle = 0;

      let mut tasks = sbc_inx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }
}
