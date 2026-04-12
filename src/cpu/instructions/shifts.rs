use crate::cpu::{
  CPU, Registers, Tasks,
  addressing::{
    OffsetVariant,
    absolute::{AbsoluteAddressingTasks, AbsoluteOffsetAddressingTasks, AccessVariant},
    zero_page::{ZeroPageAddressingTasks, ZeroPageOffsetAddressingTasks},
  },
  tasks::{modify_memory::ModifyMemoryTasks, modify_register::ModifyRegisterTasks},
};

pub fn asl_acc(cpu: &mut CPU) -> Box<dyn Tasks> {
  cpu.addr.reset_acc();
  Box::new(ModifyRegisterTasks::new_shift_left(Registers::Accumulator))
}

pub fn asl_zp(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(ModifyMemoryTasks::new_shift_left(Box::new(
    ZeroPageAddressingTasks::new(),
  )))
}

pub fn asl_zpx(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(ModifyMemoryTasks::new_shift_left(Box::new(
    ZeroPageOffsetAddressingTasks::new_offset_by_x(),
  )))
}

pub fn asl_a(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(ModifyMemoryTasks::new_shift_left(Box::new(
    AbsoluteAddressingTasks::new(),
  )))
}

pub fn asl_ax(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(ModifyMemoryTasks::new_shift_left(Box::new(
    AbsoluteOffsetAddressingTasks::new(OffsetVariant::X, AccessVariant::Modify),
  )))
}

pub fn lsr_acc(cpu: &mut CPU) -> Box<dyn Tasks> {
  cpu.addr.reset_acc();
  Box::new(ModifyRegisterTasks::new_shift_right(Registers::Accumulator))
}

pub fn lsr_zp(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(ModifyMemoryTasks::new_shift_right(Box::new(
    ZeroPageAddressingTasks::new(),
  )))
}

pub fn lsr_zpx(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(ModifyMemoryTasks::new_shift_right(Box::new(
    ZeroPageOffsetAddressingTasks::new_offset_by_x(),
  )))
}

pub fn lsr_a(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(ModifyMemoryTasks::new_shift_right(Box::new(
    AbsoluteAddressingTasks::new(),
  )))
}

pub fn lsr_ax(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(ModifyMemoryTasks::new_shift_right(Box::new(
    AbsoluteOffsetAddressingTasks::new(OffsetVariant::X, AccessVariant::Modify),
  )))
}

pub fn rol_acc(cpu: &mut CPU) -> Box<dyn Tasks> {
  cpu.addr.reset_acc();
  Box::new(ModifyRegisterTasks::new_rotate_left(Registers::Accumulator))
}

pub fn rol_zp(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(ModifyMemoryTasks::new_rotate_left(Box::new(
    ZeroPageAddressingTasks::new(),
  )))
}

pub fn rol_zpx(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(ModifyMemoryTasks::new_rotate_left(Box::new(
    ZeroPageOffsetAddressingTasks::new_offset_by_x(),
  )))
}

pub fn rol_a(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(ModifyMemoryTasks::new_rotate_left(Box::new(
    AbsoluteAddressingTasks::new(),
  )))
}

pub fn rol_ax(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(ModifyMemoryTasks::new_rotate_left(Box::new(
    AbsoluteOffsetAddressingTasks::new(OffsetVariant::X, AccessVariant::Modify),
  )))
}

pub fn ror_acc(cpu: &mut CPU) -> Box<dyn Tasks> {
  cpu.addr.reset_acc();
  Box::new(ModifyRegisterTasks::new_rotate_right(
    Registers::Accumulator,
  ))
}

pub fn ror_zp(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(ModifyMemoryTasks::new_rotate_right(Box::new(
    ZeroPageAddressingTasks::new(),
  )))
}

pub fn ror_zpx(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(ModifyMemoryTasks::new_rotate_right(Box::new(
    ZeroPageOffsetAddressingTasks::new_offset_by_x(),
  )))
}

pub fn ror_a(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(ModifyMemoryTasks::new_rotate_right(Box::new(
    AbsoluteAddressingTasks::new(),
  )))
}

pub fn ror_ax(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(ModifyMemoryTasks::new_rotate_right(Box::new(
    AbsoluteOffsetAddressingTasks::new(OffsetVariant::X, AccessVariant::Modify),
  )))
}

#[cfg(test)]
mod asl {
  #[cfg(test)]
  mod common {
    mod acc {

      use crate::cpu::{
        CPU,
        instructions::shifts::asl_acc,
        tests::{MemoryMock, run_tasks},
      };

      #[test]
      fn should_set_carry_when_bit_7_is_set() {
        let mut memory = MemoryMock::default();
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0b10000000;

        assert!(!cpu.processor_status.get_carry_flag());

        let mut tasks = asl_acc(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_carry_flag());
      }

      #[test]
      fn should_not_change_carry_when_bit_7_is_not_set() {
        let mut memory = MemoryMock::default();
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0b01111111;

        assert!(!cpu.processor_status.get_carry_flag());

        let mut tasks = asl_acc(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(!cpu.processor_status.get_carry_flag());
      }

      #[test]
      fn should_set_zero_when_value_after_shift_is_zero() {
        let mut memory = MemoryMock::default();
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0b10000000;

        assert!(!cpu.processor_status.get_zero_flag());

        let mut tasks = asl_acc(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_zero_flag());
      }

      #[test]
      fn should_set_negative_when_value_after_shift_is_negative() {
        let mut memory = MemoryMock::default();
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0b01000000;

        assert!(!cpu.processor_status.get_negative_flag());

        let mut tasks = asl_acc(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_negative_flag());
      }
    }

    mod mem {

      use crate::{
        consts::Byte,
        cpu::{
          CPU,
          instructions::shifts::asl_zp,
          tests::{MemoryMock, run_tasks},
        },
      };

      const ZERO_PAGE_ADDR: Byte = 0x01;

      #[test]
      fn should_set_carry_when_bit_7_is_set_before_shift() {
        const VALUE: Byte = 0b10000000;
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
        let mut cpu = CPU::new_nmos();
        cpu.program_counter = 0x00;

        assert!(!cpu.processor_status.get_carry_flag());

        let mut tasks = asl_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_carry_flag());
      }

      #[test]
      fn should_not_change_carry_when_bit_7_is_not_set_before_shift() {
        const VALUE: Byte = 0b01111111;
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
        let mut cpu = CPU::new_nmos();
        cpu.program_counter = 0x00;

        assert!(!cpu.processor_status.get_carry_flag());

        let mut tasks = asl_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(!cpu.processor_status.get_carry_flag());
      }

      #[test]
      fn should_set_zero_flag_when_value_after_shift_is_zero() {
        const VALUE: Byte = 0b10000000;
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
        let mut cpu = CPU::new_nmos();
        cpu.program_counter = 0x00;

        assert!(!cpu.processor_status.get_zero_flag());

        let mut tasks = asl_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_zero_flag());
      }

      #[test]
      fn should_set_negative_when_value_after_shift_is_negative() {
        const VALUE: Byte = 0b01000000;
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
        let mut cpu = CPU::new_nmos();
        cpu.program_counter = 0x00;

        assert!(!cpu.processor_status.get_negative_flag());

        let mut tasks = asl_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_negative_flag());
      }
    }
  }

  #[cfg(test)]
  mod asl_acc {

    use crate::{
      consts::Byte,
      cpu::{
        CPU,
        instructions::shifts::asl_acc,
        tests::{MemoryMock, run_tasks},
      },
    };
    const VALUE: Byte = 0x02;

    #[test]
    fn should_shift_left_value_in_accumulator() {
      let mut memory = MemoryMock::default();
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = VALUE;
      cpu.program_counter = 0x00;

      let mut tasks = asl_acc(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x04);
    }

    #[test]
    fn should_take_one_cycle() {
      let mut memory = MemoryMock::default();
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = asl_acc(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 1);
    }
  }

  #[cfg(test)]
  mod asl_zp {

    use crate::{
      consts::{Byte, Word},
      cpu::{
        CPU,
        instructions::shifts::asl_zp,
        tests::{MemoryMock, run_tasks},
      },
    };

    const ZERO_PAGE_ADDR: Byte = 0x01;
    const VALUE: Byte = 0x02;

    #[test]
    fn should_shift_left_value_in_memory_at_zero_page() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;

      let mut tasks = asl_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[ZERO_PAGE_ADDR as Word], 0x04);
    }

    #[test]
    fn should_take_four_cycles() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = asl_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }

  #[cfg(test)]
  mod asl_zpx {

    use crate::{
      consts::{Byte, Word},
      cpu::{
        CPU,
        instructions::shifts::asl_zpx,
        tests::{MemoryMock, run_tasks},
      },
    };

    const ZERO_PAGE_ADDR: Byte = 0x01;
    const OFFSET: Byte = 0x01;
    const VALUE: Byte = 0x02;

    #[test]
    fn should_shift_left_value_in_memory_at_zero_page_summed_with_index_register_x() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;

      let mut tasks = asl_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[(ZERO_PAGE_ADDR + OFFSET) as Word], 0x04);
    }

    #[test]
    fn should_take_five_cycles() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = asl_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }

  #[cfg(test)]
  mod asl_a {

    use crate::{
      consts::{Byte, Word},
      cpu::{
        CPU,
        instructions::shifts::asl_a,
        tests::{MemoryMock, run_tasks},
      },
    };

    const ABSOLUTE_ADDR_HI: Byte = 0x00;
    const ABSOLUTE_ADDR_LO: Byte = 0x03;
    const VALUE: Byte = 0x02;

    #[test]
    fn should_shift_left_value_in_memory_at_absolute_address() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;

      let mut tasks = asl_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[ABSOLUTE_ADDR_LO as Word], 0x04);
    }

    #[test]
    fn should_take_five_cycles() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = asl_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }

  #[cfg(test)]
  mod asl_ax {

    use crate::{
      consts::{Byte, Word},
      cpu::{
        CPU,
        instructions::shifts::asl_ax,
        tests::{MemoryMock, run_tasks},
      },
    };

    const ABSOLUTE_ADDR_HI: Byte = 0x00;
    const ABSOLUTE_ADDR_LO: Byte = 0x03;
    const OFFSET: Byte = 0x01;
    const VALUE: Byte = 0x02;

    #[test]
    fn should_shift_left_value_in_memory_at_absolute_address() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;

      let mut tasks = asl_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[(ABSOLUTE_ADDR_LO + OFFSET) as Word], 0x04);
    }

    #[test]
    fn should_take_six_cycles() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = asl_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 6);
    }
  }
}

#[cfg(test)]
mod lsr {
  #[cfg(test)]
  mod common {
    mod acc {

      use crate::cpu::{
        CPU,
        instructions::shifts::lsr_acc,
        tests::{MemoryMock, run_tasks},
      };

      #[test]
      fn should_set_carry_when_bit_0_is_set_before_shift() {
        let mut memory = MemoryMock::default();
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0b00000001;

        assert!(!cpu.processor_status.get_carry_flag());

        let mut tasks = lsr_acc(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_carry_flag());
      }

      #[test]
      fn should_not_change_carry_when_bit_0_is_not_set_before_shift() {
        let mut memory = MemoryMock::default();
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0b11111110;

        assert!(!cpu.processor_status.get_carry_flag());

        let mut tasks = lsr_acc(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(!cpu.processor_status.get_carry_flag());
      }

      #[test]
      fn should_set_zero_flag_when_value_after_shift_is_zero() {
        let mut memory = MemoryMock::default();
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0b00000001;

        assert!(!cpu.processor_status.get_zero_flag());

        let mut tasks = lsr_acc(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_zero_flag());
      }
    }

    mod mem {

      use crate::{
        consts::Byte,
        cpu::{
          CPU,
          instructions::shifts::lsr_zp,
          tests::{MemoryMock, run_tasks},
        },
      };

      const ZERO_PAGE_ADDR: Byte = 0x01;

      #[test]
      fn should_set_carry_when_bit_0_is_set_before_shift() {
        const VALUE: Byte = 0b00000001;
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
        let mut cpu = CPU::new_nmos();
        cpu.program_counter = 0x00;

        assert!(!cpu.processor_status.get_carry_flag());

        let mut tasks = lsr_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_carry_flag());
      }

      #[test]
      fn should_not_change_carry_when_bit_0_is_not_set_before_shift() {
        const VALUE: Byte = 0b11111110;
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
        let mut cpu = CPU::new_nmos();
        cpu.program_counter = 0x00;

        assert!(!cpu.processor_status.get_carry_flag());

        let mut tasks = lsr_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(!cpu.processor_status.get_carry_flag());
      }

      #[test]
      fn should_set_zero_flag_when_value_after_shift_is_zero() {
        const VALUE: Byte = 0b00000001;
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
        let mut cpu = CPU::new_nmos();
        cpu.program_counter = 0x00;

        assert!(!cpu.processor_status.get_zero_flag());

        let mut tasks = lsr_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_zero_flag());
      }
    }
  }

  #[cfg(test)]
  mod lsr_acc {

    use crate::{
      consts::Byte,
      cpu::{
        CPU,
        instructions::shifts::lsr_acc,
        tests::{MemoryMock, run_tasks},
      },
    };
    const VALUE: Byte = 0x02;

    #[test]
    fn should_shift_right_value_in_accumulator() {
      let mut memory = MemoryMock::default();
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = VALUE;
      cpu.program_counter = 0x00;

      let mut tasks = lsr_acc(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0x01);
    }

    #[test]
    fn should_take_one_cycle() {
      let mut memory = MemoryMock::default();
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = lsr_acc(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 1);
    }
  }

  #[cfg(test)]
  mod lsr_zp {

    use crate::{
      consts::{Byte, Word},
      cpu::{
        CPU,
        instructions::shifts::lsr_zp,
        tests::{MemoryMock, run_tasks},
      },
    };

    const ZERO_PAGE_ADDR: Byte = 0x01;
    const VALUE: Byte = 0x02;

    #[test]
    fn should_shift_right_value_in_memory_at_zero_page() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;

      let mut tasks = lsr_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[ZERO_PAGE_ADDR as Word], 0x01);
    }

    #[test]
    fn should_take_four_cycles() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = lsr_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }

  #[cfg(test)]
  mod asl_zpx {

    use crate::{
      consts::{Byte, Word},
      cpu::{
        CPU,
        instructions::shifts::lsr_zpx,
        tests::{MemoryMock, run_tasks},
      },
    };

    const ZERO_PAGE_ADDR: Byte = 0x01;
    const OFFSET: Byte = 0x01;
    const VALUE: Byte = 0x02;

    #[test]
    fn should_shift_right_value_in_memory_at_zero_page_summed_with_index_register_x() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;

      let mut tasks = lsr_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[(ZERO_PAGE_ADDR + OFFSET) as Word], 0x01);
    }

    #[test]
    fn should_take_five_cycles() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = lsr_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }

  #[cfg(test)]
  mod lsr_a {

    use crate::{
      consts::{Byte, Word},
      cpu::{
        CPU,
        instructions::shifts::lsr_a,
        tests::{MemoryMock, run_tasks},
      },
    };

    const ABSOLUTE_ADDR_HI: Byte = 0x00;
    const ABSOLUTE_ADDR_LO: Byte = 0x03;
    const VALUE: Byte = 0x02;

    #[test]
    fn should_shift_right_value_in_memory_at_absolute_address() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;

      let mut tasks = lsr_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[ABSOLUTE_ADDR_LO as Word], 0x01);
    }

    #[test]
    fn should_take_five_cycles() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = lsr_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }

  #[cfg(test)]
  mod lsr_ax {

    use crate::{
      consts::{Byte, Word},
      cpu::{
        CPU,
        instructions::shifts::lsr_ax,
        tests::{MemoryMock, run_tasks},
      },
    };

    const ABSOLUTE_ADDR_HI: Byte = 0x00;
    const ABSOLUTE_ADDR_LO: Byte = 0x03;
    const OFFSET: Byte = 0x01;
    const VALUE: Byte = 0x02;

    #[test]
    fn should_shift_right_value_in_memory_at_absolute_address() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;

      let mut tasks = lsr_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[(ABSOLUTE_ADDR_LO + OFFSET) as Word], 0x01);
    }

    #[test]
    fn should_take_six_cycles() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = lsr_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 6);
    }
  }
}

#[cfg(test)]
mod rol {
  #[cfg(test)]
  mod common {
    mod acc {

      use crate::cpu::{
        CPU,
        instructions::shifts::rol_acc,
        tests::{MemoryMock, run_tasks},
      };

      #[test]
      fn should_set_carry_when_bit_7_is_set_before_rotation() {
        let mut memory = MemoryMock::default();
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0b10000000;

        assert!(!cpu.processor_status.get_carry_flag());

        let mut tasks = rol_acc(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_carry_flag());
      }

      #[test]
      fn should_not_change_carry_when_bit_7_is_not_set_before_rotation() {
        let mut memory = MemoryMock::default();
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0b01111111;

        assert!(!cpu.processor_status.get_carry_flag());

        let mut tasks = rol_acc(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(!cpu.processor_status.get_carry_flag());
      }

      #[test]
      fn should_set_zero_when_value_after_shift_is_zero() {
        let mut memory = MemoryMock::default();
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0b10000000;

        assert!(!cpu.processor_status.get_zero_flag());

        let mut tasks = rol_acc(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_zero_flag());
      }
    }

    mod mem {

      use crate::{
        consts::Byte,
        cpu::{
          CPU,
          instructions::shifts::rol_zp,
          tests::{MemoryMock, run_tasks},
        },
      };

      const ZERO_PAGE_ADDR: Byte = 0x01;

      #[test]
      fn should_set_carry_when_bit_7_is_set_before_rotation() {
        const VALUE: Byte = 0b10000000;
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
        let mut cpu = CPU::new_nmos();
        cpu.program_counter = 0x00;

        assert!(!cpu.processor_status.get_carry_flag());

        let mut tasks = rol_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_carry_flag());
      }

      #[test]
      fn should_not_change_carry_when_bit_7_is_not_set_before_rotation() {
        const VALUE: Byte = 0b01111111;
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
        let mut cpu = CPU::new_nmos();
        cpu.program_counter = 0x00;

        assert!(!cpu.processor_status.get_carry_flag());

        let mut tasks = rol_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(!cpu.processor_status.get_carry_flag());
      }

      #[test]
      fn should_set_zero_when_value_after_shift_is_zero() {
        const VALUE: Byte = 0b10000000;
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
        let mut cpu = CPU::new_nmos();
        cpu.program_counter = 0x00;

        assert!(!cpu.processor_status.get_zero_flag());

        let mut tasks = rol_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_zero_flag());
      }
    }
  }

  #[cfg(test)]
  mod rol_acc {

    use crate::{
      consts::Byte,
      cpu::{
        CPU,
        instructions::shifts::rol_acc,
        tests::{MemoryMock, run_tasks},
      },
    };

    const VALUE: Byte = 0x02;

    #[test]
    fn should_rotate_value_left_in_accumulator() {
      let mut memory = MemoryMock::default();
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = VALUE;

      let mut tasks = rol_acc(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0b00000100);
    }

    #[test]
    fn should_set_bit_0_when_carry_is_set() {
      let mut memory = MemoryMock::default();
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.accumulator = VALUE;

      let mut tasks = rol_acc(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0b00000101);
    }

    #[test]
    fn should_not_set_bit_0_when_carry_is_not_set() {
      let mut memory = MemoryMock::default();
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(false);
      cpu.accumulator = VALUE;

      let mut tasks = rol_acc(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0b00000100);
    }

    #[test]
    fn should_take_one_cycle() {
      let mut memory = MemoryMock::default();
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = VALUE;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = rol_acc(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 1);
    }
  }

  #[cfg(test)]
  mod rol_zp {

    use crate::{
      consts::{Byte, Word},
      cpu::{
        CPU,
        instructions::shifts::rol_zp,
        tests::{MemoryMock, run_tasks},
      },
    };

    const ZERO_PAGE_ADDR: Byte = 0x01;
    const VALUE: Byte = 0x02;

    #[test]
    fn should_rotate_value_left_in_memory_at_zero_page() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;

      let mut tasks = rol_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[ZERO_PAGE_ADDR as Word], 0b00000100);
    }

    #[test]
    fn should_set_bit_0_when_carry_is_set() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.processor_status.change_carry_flag(true);

      let mut tasks = rol_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[ZERO_PAGE_ADDR as Word], 0b00000101);
    }

    #[test]
    fn should_not_set_bit_0_when_carry_is_not_set() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.processor_status.change_carry_flag(false);

      let mut tasks = rol_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[ZERO_PAGE_ADDR as Word], 0b00000100);
    }

    #[test]
    fn should_take_four_cycles() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = rol_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }

  #[cfg(test)]
  mod rol_zpx {

    use crate::{
      consts::{Byte, Word},
      cpu::{
        CPU,
        instructions::shifts::rol_zpx,
        tests::{MemoryMock, run_tasks},
      },
    };

    const ZERO_PAGE_ADDR: Byte = 0x01;
    const OFFSET: Byte = 0x01;
    const VALUE: Byte = 0x02;

    #[test]
    fn should_rotate_value_left_in_memory_at_zero_page_summed_with_index_register_x() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;

      let mut tasks = rol_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[(ZERO_PAGE_ADDR + OFFSET) as Word], 0b00000100);
    }

    #[test]
    fn should_set_bit_0_when_carry_is_set() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;
      cpu.processor_status.change_carry_flag(true);

      let mut tasks = rol_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[(ZERO_PAGE_ADDR + OFFSET) as Word], 0b00000101);
    }

    #[test]
    fn should_not_set_bit_0_when_carry_is_not_set() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;
      cpu.processor_status.change_carry_flag(false);

      let mut tasks = rol_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[(ZERO_PAGE_ADDR + OFFSET) as Word], 0b00000100);
    }

    #[test]
    fn should_take_five_cycles() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = rol_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }

  #[cfg(test)]
  mod rol_a {

    use crate::{
      consts::{Byte, Word},
      cpu::{
        CPU,
        instructions::shifts::rol_a,
        tests::{MemoryMock, run_tasks},
      },
    };

    const ABSOLUTE_ADDR_HI: Byte = 0x00;
    const ABSOLUTE_ADDR_LO: Byte = 0x03;
    const VALUE: Byte = 0x02;

    #[test]
    fn should_rotate_value_left_in_memory_at_absolute_address() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;

      let mut tasks = rol_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[ABSOLUTE_ADDR_LO as Word], 0b00000100);
    }

    #[test]
    fn should_set_bit_0_when_carry_is_set() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.processor_status.change_carry_flag(true);

      let mut tasks = rol_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[ABSOLUTE_ADDR_LO as Word], 0b00000101);
    }

    #[test]
    fn should_not_set_bit_0_when_carry_is_not_set() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.processor_status.change_carry_flag(false);

      let mut tasks = rol_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[ABSOLUTE_ADDR_LO as Word], 0b00000100);
    }

    #[test]
    fn should_take_five_cycles() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = rol_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }

  #[cfg(test)]
  mod rol_ax {

    use crate::{
      consts::{Byte, Word},
      cpu::{
        CPU,
        instructions::shifts::rol_ax,
        tests::{MemoryMock, run_tasks},
      },
    };

    const ABSOLUTE_ADDR_HI: Byte = 0x00;
    const ABSOLUTE_ADDR_LO: Byte = 0x03;
    const OFFSET: Byte = 0x01;
    const VALUE: Byte = 0x02;

    #[test]
    fn should_rotate_value_left_in_memory_at_absolute_address() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;

      let mut tasks = rol_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[(ABSOLUTE_ADDR_LO + OFFSET) as Word], 0b00000100);
    }

    #[test]
    fn should_set_bit_0_when_carry_is_set() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;
      cpu.processor_status.change_carry_flag(true);

      let mut tasks = rol_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[(ABSOLUTE_ADDR_LO + OFFSET) as Word], 0b00000101);
    }

    #[test]
    fn should_not_set_bit_0_when_carry_is_not_set() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;
      cpu.processor_status.change_carry_flag(false);

      let mut tasks = rol_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[(ABSOLUTE_ADDR_LO + OFFSET) as Word], 0b00000100);
    }

    #[test]
    fn should_take_six_cycles() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = rol_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 6);
    }
  }
}

#[cfg(test)]
mod ror {
  #[cfg(test)]
  mod common {
    mod acc {

      use crate::cpu::{
        CPU,
        instructions::shifts::ror_acc,
        tests::{MemoryMock, run_tasks},
      };

      #[test]
      fn should_set_carry_when_bit_0_is_set() {
        let mut memory = MemoryMock::default();
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0b00000001;

        assert!(!cpu.processor_status.get_carry_flag());

        let mut tasks = ror_acc(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_carry_flag());
      }

      #[test]
      fn should_not_change_carry_when_bit_0_is_not_set() {
        let mut memory = MemoryMock::default();
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0b11111110;

        assert!(!cpu.processor_status.get_carry_flag());

        let mut tasks = ror_acc(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(!cpu.processor_status.get_carry_flag());
      }

      #[test]
      fn should_set_zero_when_value_after_shift_is_zero() {
        let mut memory = MemoryMock::default();
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0b00000001;

        assert!(!cpu.processor_status.get_zero_flag());

        let mut tasks = ror_acc(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_zero_flag());
      }
    }

    mod mem {

      use crate::{
        consts::Byte,
        cpu::{
          CPU,
          instructions::shifts::ror_zp,
          tests::{MemoryMock, run_tasks},
        },
      };

      const ZERO_PAGE_ADDR: Byte = 0x01;

      #[test]
      fn should_set_carry_when_bit_0_is_set_before_rotation() {
        const VALUE: Byte = 0b00000001;
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
        let mut cpu = CPU::new_nmos();
        cpu.program_counter = 0x00;

        assert!(!cpu.processor_status.get_carry_flag());

        let mut tasks = ror_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_carry_flag());
      }

      #[test]
      fn should_not_change_carry_when_bit_0_is_not_set_before_rotation() {
        const VALUE: Byte = 0b11111110;
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
        let mut cpu = CPU::new_nmos();
        cpu.program_counter = 0x00;

        assert!(!cpu.processor_status.get_carry_flag());

        let mut tasks = ror_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(!cpu.processor_status.get_carry_flag());
      }

      #[test]
      fn should_set_zero_when_value_after_shift_is_zero() {
        const VALUE: Byte = 0b00000001;
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
        let mut cpu = CPU::new_nmos();
        cpu.program_counter = 0x00;

        assert!(!cpu.processor_status.get_zero_flag());

        let mut tasks = ror_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(cpu.processor_status.get_zero_flag());
      }
    }
  }

  #[cfg(test)]
  mod ror_acc {

    use crate::{
      consts::Byte,
      cpu::{
        CPU,
        instructions::shifts::ror_acc,
        tests::{MemoryMock, run_tasks},
      },
    };

    const VALUE: Byte = 0x02;

    #[test]
    fn should_rotate_value_right_in_accumulator() {
      let mut memory = MemoryMock::default();
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = VALUE;

      let mut tasks = ror_acc(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0b00000001);
    }

    #[test]
    fn should_set_bit_7_when_carry_is_set() {
      let mut memory = MemoryMock::default();
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(true);
      cpu.accumulator = VALUE;

      let mut tasks = ror_acc(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0b10000001);
    }

    #[test]
    fn should_not_set_bit_7_when_carry_is_not_set() {
      let mut memory = MemoryMock::default();
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.change_carry_flag(false);
      cpu.accumulator = VALUE;

      let mut tasks = ror_acc(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.accumulator, 0b00000001);
    }

    #[test]
    fn should_take_one_cycle() {
      let mut memory = MemoryMock::default();
      let mut cpu = CPU::new_nmos();
      cpu.accumulator = VALUE;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = ror_acc(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 1);
    }
  }

  #[cfg(test)]
  mod ror_zp {

    use crate::{
      consts::{Byte, Word},
      cpu::{
        CPU,
        instructions::shifts::ror_zp,
        tests::{MemoryMock, run_tasks},
      },
    };

    const ZERO_PAGE_ADDR: Byte = 0x01;
    const VALUE: Byte = 0x02;

    #[test]
    fn should_rotate_value_right_in_memory_at_zero_page() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;

      let mut tasks = ror_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[ZERO_PAGE_ADDR as Word], 0b00000001);
    }

    #[test]
    fn should_set_bit_7_when_carry_is_set() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.processor_status.change_carry_flag(true);

      let mut tasks = ror_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[ZERO_PAGE_ADDR as Word], 0b10000001);
    }

    #[test]
    fn should_not_set_bit_0_when_carry_is_not_set() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.processor_status.change_carry_flag(false);

      let mut tasks = ror_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[ZERO_PAGE_ADDR as Word], 0b00000001);
    }

    #[test]
    fn should_take_four_cycles() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = ror_zp(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }

  #[cfg(test)]
  mod ror_zpx {

    use crate::{
      consts::{Byte, Word},
      cpu::{
        CPU,
        instructions::shifts::ror_zpx,
        tests::{MemoryMock, run_tasks},
      },
    };

    const ZERO_PAGE_ADDR: Byte = 0x01;
    const OFFSET: Byte = 0x01;
    const VALUE: Byte = 0x02;

    #[test]
    fn should_rotate_value_right_in_memory_at_zero_page_summed_with_index_register_x() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;

      let mut tasks = ror_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[(ZERO_PAGE_ADDR + OFFSET) as Word], 0b00000001);
    }

    #[test]
    fn should_set_bit_7_when_carry_is_set() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;
      cpu.processor_status.change_carry_flag(true);

      let mut tasks = ror_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[(ZERO_PAGE_ADDR + OFFSET) as Word], 0b10000001);
    }

    #[test]
    fn should_not_set_bit_7_when_carry_is_not_set() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;
      cpu.processor_status.change_carry_flag(false);

      let mut tasks = ror_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[(ZERO_PAGE_ADDR + OFFSET) as Word], 0b00000001);
    }

    #[test]
    fn should_take_five_cycles() {
      let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = ror_zpx(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }

  #[cfg(test)]
  mod ror_a {

    use crate::{
      consts::{Byte, Word},
      cpu::{
        CPU,
        instructions::shifts::ror_a,
        tests::{MemoryMock, run_tasks},
      },
    };

    const ABSOLUTE_ADDR_HI: Byte = 0x00;
    const ABSOLUTE_ADDR_LO: Byte = 0x03;
    const VALUE: Byte = 0x02;

    #[test]
    fn should_rotate_value_right_in_memory_at_absolute_address() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;

      let mut tasks = ror_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[ABSOLUTE_ADDR_LO as Word], 0b00000001);
    }

    #[test]
    fn should_set_bit_7_when_carry_is_set() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.processor_status.change_carry_flag(true);

      let mut tasks = ror_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[ABSOLUTE_ADDR_LO as Word], 0b10000001);
    }

    #[test]
    fn should_not_set_bit_7_when_carry_is_not_set() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.processor_status.change_carry_flag(false);

      let mut tasks = ror_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[ABSOLUTE_ADDR_LO as Word], 0b00000001);
    }

    #[test]
    fn should_take_five_cycles() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = ror_a(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }
  }

  #[cfg(test)]
  mod ror_ax {

    use crate::{
      consts::{Byte, Word},
      cpu::{
        CPU,
        instructions::shifts::ror_ax,
        tests::{MemoryMock, run_tasks},
      },
    };

    const ABSOLUTE_ADDR_HI: Byte = 0x00;
    const ABSOLUTE_ADDR_LO: Byte = 0x03;
    const OFFSET: Byte = 0x01;
    const VALUE: Byte = 0x02;

    #[test]
    fn should_rotate_value_right_in_memory_at_absolute_address() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;

      let mut tasks = ror_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[(ABSOLUTE_ADDR_LO + OFFSET) as Word], 0b00000001);
    }

    #[test]
    fn should_set_bit_7_when_carry_is_set() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;
      cpu.processor_status.change_carry_flag(true);

      let mut tasks = ror_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[(ABSOLUTE_ADDR_LO + OFFSET) as Word], 0b10000001);
    }

    #[test]
    fn should_not_set_bit_7_when_carry_is_not_set() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;
      cpu.processor_status.change_carry_flag(false);

      let mut tasks = ror_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[(ABSOLUTE_ADDR_LO + OFFSET) as Word], 0b00000001);
    }

    #[test]
    fn should_take_six_cycles() {
      let mut memory = MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_x = OFFSET;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = ror_ax(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 6);
    }
  }
}
