use crate::{
  cpu::{tasks::read_memory::ReadMemoryTasks, AddressingMode, Registers, Tasks, CPU},
  memory::Memory,
};

enum Variant {
  And,
  Eor,
  Ora,
  Bit,
}

struct LogicalTasks {
  done: bool,
  read_memory_tasks: Box<dyn ReadMemoryTasks>,
  variant: Variant,
}

impl LogicalTasks {
  pub fn new_and(read_memory_tasks: Box<dyn ReadMemoryTasks>) -> Self {
    LogicalTasks {
      done: false,
      read_memory_tasks,
      variant: Variant::And,
    }
  }

  pub fn new_eor(read_memory_tasks: Box<dyn ReadMemoryTasks>) -> Self {
    LogicalTasks {
      done: false,
      read_memory_tasks,
      variant: Variant::Eor,
    }
  }

  pub fn new_ora(read_memory_tasks: Box<dyn ReadMemoryTasks>) -> Self {
    LogicalTasks {
      done: false,
      read_memory_tasks,
      variant: Variant::Ora,
    }
  }

  pub fn new_bit(read_memory_tasks: Box<dyn ReadMemoryTasks>) -> Self {
    LogicalTasks {
      done: false,
      read_memory_tasks,
      variant: Variant::Bit,
    }
  }
}

impl Tasks for LogicalTasks {
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

    match self.variant {
      Variant::And => {
        let result_value = cpu.get_register(Registers::Accumulator) & value;

        cpu.set_register(Registers::Accumulator, result_value);
      }
      Variant::Eor => {
        let result_value = cpu.get_register(Registers::Accumulator) ^ value;

        cpu.set_register(Registers::Accumulator, result_value);
      }
      Variant::Ora => {
        let result_value = cpu.get_register(Registers::Accumulator) | value;

        cpu.set_register(Registers::Accumulator, result_value);
      }
      Variant::Bit => {
        cpu.set_bit_status(cpu.accumulator & value);
      }
    }
    self.done = true;

    self.done
  }
}

pub fn and(cpu: &mut CPU, addr_mode: Option<AddressingMode>) -> Box<dyn Tasks> {
  let read_memory_tasks = cpu.read_memory(addr_mode);
  Box::new(LogicalTasks::new_and(read_memory_tasks))
}

pub fn and_im(cpu: &mut CPU) -> Box<dyn Tasks> {
  and(cpu, None)
}

pub fn and_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
  and(cpu, Some(AddressingMode::ZeroPage))
}

pub fn and_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
  and(cpu, Some(AddressingMode::ZeroPageX))
}

pub fn and_a(cpu: &mut CPU) -> Box<dyn Tasks> {
  and(cpu, Some(AddressingMode::Absolute))
}

pub fn and_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
  and(cpu, Some(AddressingMode::AbsoluteX))
}

pub fn and_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
  and(cpu, Some(AddressingMode::AbsoluteY))
}

pub fn and_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
  and(cpu, Some(AddressingMode::IndexIndirectX))
}

pub fn and_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
  and(cpu, Some(AddressingMode::IndirectIndexY))
}

pub fn eor(cpu: &mut CPU, addr_mode: Option<AddressingMode>) -> Box<dyn Tasks> {
  let read_memory_tasks = cpu.read_memory(addr_mode);
  Box::new(LogicalTasks::new_eor(read_memory_tasks))
}

pub fn eor_im(cpu: &mut CPU) -> Box<dyn Tasks> {
  eor(cpu, None)
}

pub fn eor_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
  eor(cpu, Some(AddressingMode::ZeroPage))
}

pub fn eor_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
  eor(cpu, Some(AddressingMode::ZeroPageX))
}

pub fn eor_a(cpu: &mut CPU) -> Box<dyn Tasks> {
  eor(cpu, Some(AddressingMode::Absolute))
}

pub fn eor_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
  eor(cpu, Some(AddressingMode::AbsoluteX))
}

pub fn eor_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
  eor(cpu, Some(AddressingMode::AbsoluteY))
}

pub fn eor_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
  eor(cpu, Some(AddressingMode::IndexIndirectX))
}

pub fn eor_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
  eor(cpu, Some(AddressingMode::IndirectIndexY))
}

pub fn ora(cpu: &mut CPU, addr_mode: Option<AddressingMode>) -> Box<dyn Tasks> {
  let read_memory_tasks = cpu.read_memory(addr_mode);
  Box::new(LogicalTasks::new_ora(read_memory_tasks))
}

pub fn ora_im(cpu: &mut CPU) -> Box<dyn Tasks> {
  ora(cpu, None)
}

pub fn ora_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
  ora(cpu, Some(AddressingMode::ZeroPage))
}

pub fn ora_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
  ora(cpu, Some(AddressingMode::ZeroPageX))
}

pub fn ora_a(cpu: &mut CPU) -> Box<dyn Tasks> {
  ora(cpu, Some(AddressingMode::Absolute))
}

pub fn ora_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
  ora(cpu, Some(AddressingMode::AbsoluteX))
}

pub fn ora_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
  ora(cpu, Some(AddressingMode::AbsoluteY))
}

pub fn ora_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
  ora(cpu, Some(AddressingMode::IndexIndirectX))
}

pub fn ora_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
  ora(cpu, Some(AddressingMode::IndirectIndexY))
}

pub fn bit(cpu: &mut CPU, addr_mode: Option<AddressingMode>) -> Box<dyn Tasks> {
  let read_memory_tasks = cpu.read_memory(addr_mode);
  Box::new(LogicalTasks::new_bit(read_memory_tasks))
}

pub fn bit_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
  bit(cpu, Some(AddressingMode::ZeroPage))
}

pub fn bit_a(cpu: &mut CPU) -> Box<dyn Tasks> {
  bit(cpu, Some(AddressingMode::Absolute))
}

#[cfg(test)]
mod ora {
  #[cfg(test)]
  mod ora_im {

    use crate::cpu::{
      instructions::ora_im,
      tests::{run_tasks, MemoryMock},
      CPU,
    };

    #[test]
    fn should_or_accumulator_with_a_value_from_immediate_address() {
      let mut memory = MemoryMock::new(&[0x22, 0x00]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x16;

      let mut tasks = ora_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x36);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[0x22, 0x00]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x86;

      let mut tasks = ora_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_one_cycle() {
      let mut memory = MemoryMock::new(&[0x22, 0x00]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x14;
      cpu.cycle = 0;

      let mut tasks = ora_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 1);
    }
  }

  #[cfg(test)]
  mod ora_zp {

    use crate::cpu::{
      instructions::ora_zp,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ZERO_PAGE_ADDR: Byte = 0x03;
    const VALUE: Byte = 0x22;

    #[test]
    fn should_or_accumulator_with_a_value_from_zero_page_address() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x16;
      cpu.program_counter = 0x00;

      let mut tasks = ora_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x36);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x86;
      cpu.program_counter = 0x00;

      let mut tasks = ora_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_two_cycles() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x16;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = ora_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 2);
    }
  }

  #[cfg(test)]
  mod ora_zpx {

    use crate::cpu::{
      instructions::ora_zpx,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ZERO_PAGE_ADDR: Byte = 0x01;
    const VALUE: Byte = 0x22;

    #[test]
    fn should_or_accumulator_with_a_value_at_a_zero_page_address_summed_with_index_register_x() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x16;
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;

      let mut tasks = ora_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x36);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x86;
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;

      let mut tasks = ora_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_three_cycles() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x16;
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = ora_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }
  }

  #[cfg(test)]
  mod ora_a {

    use crate::cpu::{
      instructions::ora_a,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ADDR_LO: Byte = 0x04;
    const ADDR_HI: Byte = 0x00;
    const VALUE: Byte = 0x22;

    #[test]
    fn should_or_accumulator_with_a_value_in_memory_at_an_absolute_address() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x16;
      cpu.program_counter = 0x00;

      let mut tasks = ora_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x36);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x86;
      cpu.program_counter = 0x00;

      let mut tasks = ora_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_three_cycles() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x16;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = ora_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }
  }

  #[cfg(test)]
  mod ora_ax {

    use crate::cpu::{
      instructions::ora_ax,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ADDR_LO: Byte = 0x02;
    const ADDR_HI: Byte = 0x00;
    const OFFSET: Byte = 0x02;
    const VALUE: Byte = 0x22;
    const ADDR_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;

    #[test]
    fn should_or_accumulator_with_a_value_in_memory_at_an_absolute_address_offset_by_index_register_x(
    ) {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x16;
      cpu.program_counter = 0x00;
      cpu.index_register_x = OFFSET;

      let mut tasks = ora_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x36);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x86;
      cpu.program_counter = 0x00;
      cpu.index_register_x = OFFSET;

      let mut tasks = ora_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x16;
      cpu.program_counter = 0x00;
      cpu.index_register_x = OFFSET;
      cpu.cycle = 0;

      let mut tasks = ora_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }

    #[test]
    fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[
        ADDR_LO_ON_ZERO_PAGE_BOUNDARY,
        ADDR_HI,
        0x45,
        0xAF,
        0xDD,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x16;
      cpu.program_counter = 0x00;
      cpu.index_register_x = OFFSET;
      cpu.cycle = 0;

      let mut tasks = ora_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }

  #[cfg(test)]
  mod ora_ay {

    use crate::cpu::{
      instructions::ora_ay,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ADDR_LO: Byte = 0x02;
    const ADDR_HI: Byte = 0x00;
    const OFFSET: Byte = 0x02;
    const VALUE: Byte = 0x22;
    const ADDR_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;

    #[test]
    fn should_or_accumulator_with_a_value_in_memory_at_an_absolute_address_offset_by_index_register_y(
    ) {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x16;
      cpu.program_counter = 0x00;
      cpu.index_register_y = OFFSET;

      let mut tasks = ora_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x36);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x86;
      cpu.program_counter = 0x00;
      cpu.index_register_y = OFFSET;

      let mut tasks = ora_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x16;
      cpu.program_counter = 0x00;
      cpu.index_register_y = OFFSET;
      cpu.cycle = 0;

      let mut tasks = ora_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }

    #[test]
    fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[
        ADDR_LO_ON_ZERO_PAGE_BOUNDARY,
        ADDR_HI,
        0x45,
        0xAF,
        0xDD,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x16;
      cpu.program_counter = 0x00;
      cpu.index_register_y = OFFSET;
      cpu.cycle = 0;

      let mut tasks = ora_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }

  #[cfg(test)]
  mod ora_inx {

    use crate::cpu::{
      instructions::ora_inx,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ZP_ADDRESS: Byte = 0x02;
    const OFFSET: Byte = 0x01;
    const EFFECTIVE_ADDRESS_LO: Byte = 0x05;
    const EFFECTIVE_ADDRESS_HI: Byte = 0x00;
    const VALUE: Byte = 0x22;

    #[test]
    fn should_or_accumulator_with_a_value_in_an_indirect_adress_stored_in_zero_page_offset_with_index_register_x(
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
      cpu.accumulator = 0x16;
      cpu.index_register_x = OFFSET;

      let mut tasks = ora_inx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x36);
    }

    #[test]
    fn should_set_processor_status() {
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
      cpu.accumulator = 0x86;
      cpu.index_register_x = OFFSET;

      let mut tasks = ora_inx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
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
      cpu.accumulator = 0x16;
      cpu.index_register_x = OFFSET;
      cpu.cycle = 0;

      let mut tasks = ora_inx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }

  #[cfg(test)]
  mod ora_iny {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::ora_iny,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };

    const INDIRECT_ZERO_PAGE_ADDRESS_PLACE: Byte = 0x01;
    const ADDRESS_LO: Byte = 0x03;
    const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
    const ADDRESS_HI: Byte = 0x00;
    const VALUE: Byte = 0x22;

    #[test]
    fn should_or_accumulator_with_a_value_from_an_indirect_adress_stored_in_memory_at_zero_page_and_offset_with_value_from_index_register_y(
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
      cpu.accumulator = 0x16;
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;

      let mut tasks = ora_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x36);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[
        INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
        ADDRESS_LO,
        ADDRESS_HI,
        0x45,
        0xAF,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x86;
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;

      let mut tasks = ora_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
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
      cpu.accumulator = 0x16;
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = ora_iny(&mut cpu);
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
      cpu.accumulator = 0x16;
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = ora_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }
}

#[cfg(test)]
mod bit {
  #[cfg(test)]
  mod bit_zp {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::bit_zp,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };

    const ZERO_PAGE_ADDR_LO: Byte = 0x01;

    #[test]
    fn should_set_zero_flag_when_logic_and_on_accumulator_and_value_from_zero_page_is_zero() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR_LO, 0x0F]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0xF0;

      let mut tasks = bit_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b00000010);
    }

    #[test]
    fn should_set_carry_flag_when_logic_and_on_accumulator_and_value_from_zero_page_is_has_seventh_bit_set(
    ) {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR_LO, 0b01000000]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0xF0;

      let mut tasks = bit_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000000);
    }

    #[test]
    fn should_set_negative_flag_when_logic_and_on_accumulator_and_value_from_zero_page_is_has_eight_bit_set(
    ) {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR_LO, 0b10000000]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0xF0;

      let mut tasks = bit_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b10000000);
    }

    #[test]
    fn should_take_two_cycles() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR_LO, 0x0F]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0xF0;

      let mut tasks = bit_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 2);
    }
  }

  #[cfg(test)]
  mod bit_a {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::bit_a,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };

    const ABSOLUTE_ADDR_LO: Byte = 0x03;
    const ABSOLUTE_ADDR_HI: Byte = 0x00;

    #[test]
    fn should_set_zero_flag_when_logic_and_on_accumulator_and_value_from_absolute_address_is_zero()
    {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x0F]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0xF0;

      let mut tasks = bit_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b00000010);
    }

    #[test]
    fn should_set_carry_flag_when_logic_and_on_accumulator_and_value_from_absolute_address_is_has_seventh_bit_set(
    ) {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0b01000000]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0xF0;

      let mut tasks = bit_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b01000000);
    }

    #[test]
    fn should_set_negative_flag_when_logic_and_on_accumulator_and_value_from_absolute_address_is_has_eight_bit_set(
    ) {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0b10000000]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0xF0;

      let mut tasks = bit_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0b10000000);
    }

    #[test]
    fn should_take_two_cycles() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x0F]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0xF0;

      let mut tasks = bit_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }
  }
}

#[cfg(test)]
mod and {
  #[cfg(test)]
  mod and_im {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::and_im,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };

    const VALUE: Byte = 0x82;

    #[test]
    fn should_and_accumulator_with_a_value_from_immediate_address() {
      let mut memory = MemoryMock::new(&[VALUE, 0x00]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x07;

      let mut tasks = and_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x02);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[VALUE, 0x00]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x86;

      let mut tasks = and_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_one_cycle() {
      let mut memory = MemoryMock::new(&[VALUE, 0x00]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x07;
      cpu.cycle = 0;

      let mut tasks = and_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 1);
    }
  }

  #[cfg(test)]
  mod and_zp {

    use crate::cpu::{
      instructions::and_zp,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ZERO_PAGE_ADDR: Byte = 0x03;
    const VALUE: Byte = 0x82;

    #[test]
    fn should_and_accumulator_with_a_value_from_zero_page_address() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;

      let mut tasks = and_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x02);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x86;
      cpu.program_counter = 0x00;

      let mut tasks = and_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_two_cycles() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = and_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 2);
    }
  }

  #[cfg(test)]
  mod and_zpx {

    use crate::cpu::{
      instructions::and_zpx,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ZERO_PAGE_ADDR: Byte = 0x01;
    const VALUE: Byte = 0x82;

    #[test]
    fn should_and_accumulator_with_a_value_at_a_zero_page_address_summed_with_index_register_x() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;

      let mut tasks = and_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x02);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x86;
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;

      let mut tasks = and_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_three_cycles() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = and_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }
  }

  #[cfg(test)]
  mod and_a {

    use crate::cpu::{
      instructions::and_a,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ADDR_LO: Byte = 0x04;
    const ADDR_HI: Byte = 0x00;
    const VALUE: Byte = 0x82;

    #[test]
    fn should_and_accumulator_with_a_value_in_memory_at_an_absolute_address() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;

      let mut tasks = and_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x02);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x86;
      cpu.program_counter = 0x00;

      let mut tasks = and_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_three_cycles() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = and_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }
  }

  #[cfg(test)]
  mod and_ax {

    use crate::cpu::{
      instructions::and_ax,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ADDR_LO: Byte = 0x02;
    const ADDR_HI: Byte = 0x00;
    const OFFSET: Byte = 0x02;
    const VALUE: Byte = 0x82;
    const ADDR_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;

    #[test]
    fn should_and_accumulator_with_a_value_in_memory_at_an_absolute_address_offset_by_index_register_x(
    ) {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.index_register_x = OFFSET;

      let mut tasks = and_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x02);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x86;
      cpu.program_counter = 0x00;
      cpu.index_register_x = OFFSET;

      let mut tasks = and_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.index_register_x = OFFSET;
      cpu.cycle = 0;

      let mut tasks = and_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }

    #[test]
    fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[
        ADDR_LO_ON_ZERO_PAGE_BOUNDARY,
        ADDR_HI,
        0x45,
        0xAF,
        0xDD,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.index_register_x = OFFSET;
      cpu.cycle = 0;

      let mut tasks = and_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }

  #[cfg(test)]
  mod and_ay {

    use crate::cpu::{
      instructions::and_ay,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ADDR_LO: Byte = 0x02;
    const ADDR_HI: Byte = 0x00;
    const OFFSET: Byte = 0x02;
    const VALUE: Byte = 0x82;
    const ADDR_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;

    #[test]
    fn should_and_accumulator_with_a_value_in_memory_at_an_absolute_address_offset_by_index_register_y(
    ) {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.index_register_y = OFFSET;

      let mut tasks = and_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x02);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x86;
      cpu.program_counter = 0x00;
      cpu.index_register_y = OFFSET;

      let mut tasks = and_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.index_register_y = OFFSET;
      cpu.cycle = 0;

      let mut tasks = and_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }

    #[test]
    fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[
        ADDR_LO_ON_ZERO_PAGE_BOUNDARY,
        ADDR_HI,
        0x45,
        0xAF,
        0xDD,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.index_register_y = OFFSET;
      cpu.cycle = 0;

      let mut tasks = and_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }

  #[cfg(test)]
  mod and_inx {

    use crate::cpu::{
      instructions::and_inx,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ZP_ADDRESS: Byte = 0x02;
    const OFFSET: Byte = 0x01;
    const EFFECTIVE_ADDRESS_LO: Byte = 0x05;
    const EFFECTIVE_ADDRESS_HI: Byte = 0x00;
    const VALUE: Byte = 0x82;

    #[test]
    fn should_and_accumulator_with_a_value_in_an_indirect_adress_stored_in_zero_page_offset_with_index_register_x(
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
      cpu.accumulator = 0x07;
      cpu.index_register_x = OFFSET;

      let mut tasks = and_inx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x02);
    }

    #[test]
    fn should_set_processor_status() {
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
      cpu.accumulator = 0x86;
      cpu.index_register_x = OFFSET;

      let mut tasks = and_inx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
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
      cpu.accumulator = 0x07;
      cpu.index_register_x = OFFSET;
      cpu.cycle = 0;

      let mut tasks = and_inx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }

  #[cfg(test)]
  mod and_iny {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::and_iny,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };

    const INDIRECT_ZERO_PAGE_ADDRESS_PLACE: Byte = 0x01;
    const ADDRESS_LO: Byte = 0x03;
    const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
    const ADDRESS_HI: Byte = 0x00;
    const VALUE: Byte = 0x82;

    #[test]
    fn should_and_accumulator_with_a_value_from_an_indirect_adress_stored_in_memory_at_zero_page_and_offset_with_value_from_index_register_y(
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
      cpu.accumulator = 0x07;
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;

      let mut tasks = and_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x02);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[
        INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
        ADDRESS_LO,
        ADDRESS_HI,
        0x45,
        0xAF,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x86;
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;

      let mut tasks = and_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
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
      cpu.accumulator = 0x07;
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = and_iny(&mut cpu);
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
      cpu.accumulator = 0x07;
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = and_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }
}

#[cfg(test)]
mod eor {
  #[cfg(test)]
  mod eor_im {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::eor_im,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };

    const VALUE: Byte = 0x85;

    #[test]
    fn should_eor_accumulator_with_a_value_from_immediate_address() {
      let mut memory = MemoryMock::new(&[VALUE, 0x00]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x07;

      let mut tasks = eor_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x82);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[VALUE, 0x00]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x07;

      let mut tasks = eor_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_one_cycle() {
      let mut memory = MemoryMock::new(&[VALUE, 0x00]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.accumulator = 0x07;
      cpu.cycle = 0;

      let mut tasks = eor_im(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 1);
    }
  }

  #[cfg(test)]
  mod eor_zp {

    use crate::cpu::{
      instructions::eor_zp,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ZERO_PAGE_ADDR: Byte = 0x03;
    const VALUE: Byte = 0x85;

    #[test]
    fn should_eor_accumulator_with_a_value_from_zero_page_address() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;

      let mut tasks = eor_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x82);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;

      let mut tasks = eor_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_two_cycles() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = eor_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 2);
    }
  }

  #[cfg(test)]
  mod eor_zpx {

    use crate::cpu::{
      instructions::eor_zpx,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ZERO_PAGE_ADDR: Byte = 0x01;
    const VALUE: Byte = 0x85;

    #[test]
    fn should_eor_accumulator_with_a_value_at_a_zero_page_address_summed_with_index_register_x() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;

      let mut tasks = eor_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x82);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;

      let mut tasks = eor_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_three_cycles() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.index_register_x = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = eor_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }
  }

  #[cfg(test)]
  mod eor_a {

    use crate::cpu::{
      instructions::eor_a,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ADDR_LO: Byte = 0x04;
    const ADDR_HI: Byte = 0x00;
    const VALUE: Byte = 0x85;

    #[test]
    fn should_eor_accumulator_with_a_value_in_memory_at_an_absolute_address() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;

      let mut tasks = eor_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x82);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;

      let mut tasks = eor_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_three_cycles() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = eor_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }
  }

  #[cfg(test)]
  mod eor_ax {

    use crate::cpu::{
      instructions::eor_ax,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ADDR_LO: Byte = 0x02;
    const ADDR_HI: Byte = 0x00;
    const OFFSET: Byte = 0x02;
    const VALUE: Byte = 0x85;
    const ADDR_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;

    #[test]
    fn should_eor_accumulator_with_a_value_in_memory_at_an_absolute_address_offset_by_index_register_x(
    ) {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.index_register_x = OFFSET;

      let mut tasks = eor_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x82);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.index_register_x = OFFSET;

      let mut tasks = eor_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.index_register_x = OFFSET;
      cpu.cycle = 0;

      let mut tasks = eor_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }

    #[test]
    fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[
        ADDR_LO_ON_ZERO_PAGE_BOUNDARY,
        ADDR_HI,
        0x45,
        0xAF,
        0xDD,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.index_register_x = OFFSET;
      cpu.cycle = 0;

      let mut tasks = eor_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }

  #[cfg(test)]
  mod eor_ay {

    use crate::cpu::{
      instructions::eor_ay,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ADDR_LO: Byte = 0x02;
    const ADDR_HI: Byte = 0x00;
    const OFFSET: Byte = 0x02;
    const VALUE: Byte = 0x85;
    const ADDR_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;

    #[test]
    fn should_eor_accumulator_with_a_value_in_memory_at_an_absolute_address_offset_by_index_register_y(
    ) {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.index_register_y = OFFSET;

      let mut tasks = eor_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x82);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.index_register_y = OFFSET;

      let mut tasks = eor_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.index_register_y = OFFSET;
      cpu.cycle = 0;

      let mut tasks = eor_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 3);
    }

    #[test]
    fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
      let mut memory = MemoryMock::new(&[
        ADDR_LO_ON_ZERO_PAGE_BOUNDARY,
        ADDR_HI,
        0x45,
        0xAF,
        0xDD,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.program_counter = 0x00;
      cpu.index_register_y = OFFSET;
      cpu.cycle = 0;

      let mut tasks = eor_ay(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }

  #[cfg(test)]
  mod eor_inx {

    use crate::cpu::{
      instructions::eor_inx,
      tests::{run_tasks, MemoryMock},
      Byte, CPU,
    };

    const ZP_ADDRESS: Byte = 0x02;
    const OFFSET: Byte = 0x01;
    const EFFECTIVE_ADDRESS_LO: Byte = 0x05;
    const EFFECTIVE_ADDRESS_HI: Byte = 0x00;
    const VALUE: Byte = 0x85;

    #[test]
    fn should_eor_accumulator_with_a_value_in_an_indirect_adress_stored_in_zero_page_offset_with_index_register_x(
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
      cpu.accumulator = 0x07;
      cpu.index_register_x = OFFSET;

      let mut tasks = eor_inx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x82);
    }

    #[test]
    fn should_set_processor_status() {
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
      cpu.accumulator = 0x07;
      cpu.index_register_x = OFFSET;

      let mut tasks = eor_inx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
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
      cpu.accumulator = 0x07;
      cpu.index_register_x = OFFSET;
      cpu.cycle = 0;

      let mut tasks = eor_inx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }

  #[cfg(test)]
  mod eor_iny {

    use crate::{
      consts::Byte,
      cpu::{
        instructions::eor_iny,
        tests::{run_tasks, MemoryMock},
        CPU,
      },
    };

    const INDIRECT_ZERO_PAGE_ADDRESS_PLACE: Byte = 0x01;
    const ADDRESS_LO: Byte = 0x03;
    const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
    const ADDRESS_HI: Byte = 0x00;
    const VALUE: Byte = 0x85;

    #[test]
    fn should_eor_accumulator_with_a_value_from_an_indirect_adress_stored_in_memory_at_zero_page_and_offset_with_value_from_index_register_y(
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
      cpu.accumulator = 0x07;
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;

      let mut tasks = eor_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x82);
    }

    #[test]
    fn should_set_processor_status() {
      let mut memory = MemoryMock::new(&[
        INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
        ADDRESS_LO,
        ADDRESS_HI,
        0x45,
        0xAF,
        VALUE,
      ]);
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x07;
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;

      let mut tasks = eor_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.processor_status, 0x80);
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
      cpu.accumulator = 0x07;
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = eor_iny(&mut cpu);
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
      cpu.accumulator = 0x07;
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = eor_iny(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }
}
