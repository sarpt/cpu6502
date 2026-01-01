use crate::cpu::{
    addressing::get_addressing_tasks,
    tasks::{modify_memory::ModifyMemoryTasks, modify_register::ModifyRegisterTasks},
    AddressingMode, Registers, Tasks, CPU,
};

fn asl(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(cpu, addr_mode);
    Box::new(ModifyMemoryTasks::new_shift_left(addr_tasks))
}

pub fn asl_acc(_cpu: &mut CPU) -> Box<dyn Tasks> {
    Box::new(ModifyRegisterTasks::new_shift_left(Registers::Accumulator))
}

pub fn asl_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    asl(cpu, AddressingMode::ZeroPage)
}

pub fn asl_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    asl(cpu, AddressingMode::ZeroPageX)
}

pub fn asl_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    asl(cpu, AddressingMode::Absolute)
}

pub fn asl_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    asl(cpu, AddressingMode::AbsoluteX)
}

fn lsr(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(cpu, addr_mode);
    Box::new(ModifyMemoryTasks::new_shift_right(addr_tasks))
}

pub fn lsr_acc(_cpu: &mut CPU) -> Box<dyn Tasks> {
    Box::new(ModifyRegisterTasks::new_shift_right(Registers::Accumulator))
}

pub fn lsr_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    lsr(cpu, AddressingMode::ZeroPage)
}

pub fn lsr_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    lsr(cpu, AddressingMode::ZeroPageX)
}

pub fn lsr_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    lsr(cpu, AddressingMode::Absolute)
}

pub fn lsr_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    lsr(cpu, AddressingMode::AbsoluteX)
}

fn rol(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(cpu, addr_mode);
    Box::new(ModifyMemoryTasks::new_rotate_left(addr_tasks))
}

pub fn rol_acc(_cpu: &mut CPU) -> Box<dyn Tasks> {
    Box::new(ModifyRegisterTasks::new_rotate_left(Registers::Accumulator))
}

pub fn rol_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    rol(cpu, AddressingMode::ZeroPage)
}

pub fn rol_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    rol(cpu, AddressingMode::ZeroPageX)
}

pub fn rol_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    rol(cpu, AddressingMode::Absolute)
}

pub fn rol_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    rol(cpu, AddressingMode::AbsoluteX)
}

fn ror(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(cpu, addr_mode);
    Box::new(ModifyMemoryTasks::new_rotate_right(addr_tasks))
}

pub fn ror_acc(_cpu: &mut CPU) -> Box<dyn Tasks> {
    Box::new(ModifyRegisterTasks::new_rotate_right(
        Registers::Accumulator,
    ))
}

pub fn ror_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    ror(cpu, AddressingMode::ZeroPage)
}

pub fn ror_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    ror(cpu, AddressingMode::ZeroPageX)
}

pub fn ror_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    ror(cpu, AddressingMode::Absolute)
}

pub fn ror_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    ror(cpu, AddressingMode::AbsoluteX)
}

#[cfg(test)]
mod asl {
    #[cfg(test)]
    mod common {
        mod acc {
            use std::cell::RefCell;

            use crate::cpu::{
                instructions::shifts::asl_acc,
                tests::{run_tasks, MemoryMock},
                CPU,
            };

            #[test]
            fn should_set_carry_when_bit_7_is_set() {
                let mut memory = MemoryMock::default();
                let mut cpu = CPU::new_nmos();
                cpu.accumulator = 0b10000000;

                assert_eq!(cpu.processor_status.get_carry_flag(), false);

                let mut tasks = asl_acc(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_carry_flag(), true);
            }

            #[test]
            fn should_not_change_carry_when_bit_7_is_not_set() {
                let mut memory = MemoryMock::default();
                let mut cpu = CPU::new_nmos();
                cpu.accumulator = 0b01111111;

                assert_eq!(cpu.processor_status.get_carry_flag(), false);

                let mut tasks = asl_acc(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_carry_flag(), false);
            }

            #[test]
            fn should_set_zero_when_value_after_shift_is_zero() {
                let mut memory = MemoryMock::default();
                let mut cpu = CPU::new_nmos();
                cpu.accumulator = 0b10000000;

                assert_eq!(cpu.processor_status.get_zero_flag(), false);

                let mut tasks = asl_acc(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_zero_flag(), true);
            }

            #[test]
            fn should_set_negative_when_value_after_shift_is_negative() {
                let mut memory = MemoryMock::default();
                let mut cpu = CPU::new_nmos();
                cpu.accumulator = 0b01000000;

                assert_eq!(cpu.processor_status.get_negative_flag(), false);

                let mut tasks = asl_acc(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_negative_flag(), true);
            }
        }

        mod mem {
            use std::cell::RefCell;

            use crate::{
                consts::Byte,
                cpu::{
                    instructions::shifts::asl_zp,
                    tests::{run_tasks, MemoryMock},
                    CPU,
                },
            };

            const ZERO_PAGE_ADDR: Byte = 0x01;

            #[test]
            fn should_set_carry_when_bit_7_is_set_before_shift() {
                const VALUE: Byte = 0b10000000;
                let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
                let mut cpu = CPU::new_nmos();
                cpu.program_counter = 0x00;

                assert_eq!(cpu.processor_status.get_carry_flag(), false);

                let mut tasks = asl_zp(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_carry_flag(), true);
            }

            #[test]
            fn should_not_change_carry_when_bit_7_is_not_set_before_shift() {
                const VALUE: Byte = 0b01111111;
                let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
                let mut cpu = CPU::new_nmos();
                cpu.program_counter = 0x00;

                assert_eq!(cpu.processor_status.get_carry_flag(), false);

                let mut tasks = asl_zp(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_carry_flag(), false);
            }

            #[test]
            fn should_set_zero_flag_when_value_after_shift_is_zero() {
                const VALUE: Byte = 0b10000000;
                let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
                let mut cpu = CPU::new_nmos();
                cpu.program_counter = 0x00;

                assert_eq!(cpu.processor_status.get_zero_flag(), false);

                let mut tasks = asl_zp(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_zero_flag(), true);
            }

            #[test]
            fn should_set_negative_when_value_after_shift_is_negative() {
                const VALUE: Byte = 0b01000000;
                let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
                let mut cpu = CPU::new_nmos();
                cpu.program_counter = 0x00;

                assert_eq!(cpu.processor_status.get_negative_flag(), false);

                let mut tasks = asl_zp(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_negative_flag(), true);
            }
        }
    }

    #[cfg(test)]
    mod asl_acc {
        use std::cell::RefCell;

        use crate::{
            consts::Byte,
            cpu::{
                instructions::shifts::asl_acc,
                tests::{run_tasks, MemoryMock},
                CPU,
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
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{
                instructions::shifts::asl_zp,
                tests::{run_tasks, MemoryMock},
                CPU,
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
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{
                instructions::shifts::asl_zpx,
                tests::{run_tasks, MemoryMock},
                CPU,
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
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{
                instructions::shifts::asl_a,
                tests::{run_tasks, MemoryMock},
                CPU,
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
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{
                instructions::shifts::asl_ax,
                tests::{run_tasks, MemoryMock},
                CPU,
            },
        };

        const ABSOLUTE_ADDR_HI: Byte = 0x00;
        const ABSOLUTE_ADDR_LO: Byte = 0x03;
        const OFFSET: Byte = 0x01;
        const VALUE: Byte = 0x02;

        #[test]
        fn should_shift_left_value_in_memory_at_absolute_address() {
            let mut memory =
                MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_x = OFFSET;
            cpu.program_counter = 0x00;

            let mut tasks = asl_ax(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(memory[(ABSOLUTE_ADDR_LO + OFFSET) as Word], 0x04);
        }

        #[test]
        fn should_take_six_cycles() {
            let mut memory =
                MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
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
            use std::cell::RefCell;

            use crate::cpu::{
                instructions::shifts::lsr_acc,
                tests::{run_tasks, MemoryMock},
                CPU,
            };

            #[test]
            fn should_set_carry_when_bit_0_is_set_before_shift() {
                let mut memory = MemoryMock::default();
                let mut cpu = CPU::new_nmos();
                cpu.accumulator = 0b00000001;

                assert_eq!(cpu.processor_status.get_carry_flag(), false);

                let mut tasks = lsr_acc(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_carry_flag(), true);
            }

            #[test]
            fn should_not_change_carry_when_bit_0_is_not_set_before_shift() {
                let mut memory = MemoryMock::default();
                let mut cpu = CPU::new_nmos();
                cpu.accumulator = 0b11111110;

                assert_eq!(cpu.processor_status.get_carry_flag(), false);

                let mut tasks = lsr_acc(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_carry_flag(), false);
            }

            #[test]
            fn should_set_zero_flag_when_value_after_shift_is_zero() {
                let mut memory = MemoryMock::default();
                let mut cpu = CPU::new_nmos();
                cpu.accumulator = 0b00000001;

                assert_eq!(cpu.processor_status.get_zero_flag(), false);

                let mut tasks = lsr_acc(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_zero_flag(), true);
            }
        }

        mod mem {
            use std::cell::RefCell;

            use crate::{
                consts::Byte,
                cpu::{
                    instructions::shifts::lsr_zp,
                    tests::{run_tasks, MemoryMock},
                    CPU,
                },
            };

            const ZERO_PAGE_ADDR: Byte = 0x01;

            #[test]
            fn should_set_carry_when_bit_0_is_set_before_shift() {
                const VALUE: Byte = 0b00000001;
                let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
                let mut cpu = CPU::new_nmos();
                cpu.program_counter = 0x00;

                assert_eq!(cpu.processor_status.get_carry_flag(), false);

                let mut tasks = lsr_zp(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_carry_flag(), true);
            }

            #[test]
            fn should_not_change_carry_when_bit_0_is_not_set_before_shift() {
                const VALUE: Byte = 0b11111110;
                let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
                let mut cpu = CPU::new_nmos();
                cpu.program_counter = 0x00;

                assert_eq!(cpu.processor_status.get_carry_flag(), false);

                let mut tasks = lsr_zp(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_carry_flag(), false);
            }

            #[test]
            fn should_set_zero_flag_when_value_after_shift_is_zero() {
                const VALUE: Byte = 0b00000001;
                let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
                let mut cpu = CPU::new_nmos();
                cpu.program_counter = 0x00;

                assert_eq!(cpu.processor_status.get_zero_flag(), false);

                let mut tasks = lsr_zp(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_zero_flag(), true);
            }
        }
    }

    #[cfg(test)]
    mod lsr_acc {
        use std::cell::RefCell;

        use crate::{
            consts::Byte,
            cpu::{
                instructions::shifts::lsr_acc,
                tests::{run_tasks, MemoryMock},
                CPU,
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
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{
                instructions::shifts::lsr_zp,
                tests::{run_tasks, MemoryMock},
                CPU,
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
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{
                instructions::shifts::lsr_zpx,
                tests::{run_tasks, MemoryMock},
                CPU,
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
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{
                instructions::shifts::lsr_a,
                tests::{run_tasks, MemoryMock},
                CPU,
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
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{
                instructions::shifts::lsr_ax,
                tests::{run_tasks, MemoryMock},
                CPU,
            },
        };

        const ABSOLUTE_ADDR_HI: Byte = 0x00;
        const ABSOLUTE_ADDR_LO: Byte = 0x03;
        const OFFSET: Byte = 0x01;
        const VALUE: Byte = 0x02;

        #[test]
        fn should_shift_right_value_in_memory_at_absolute_address() {
            let mut memory =
                MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_x = OFFSET;
            cpu.program_counter = 0x00;

            let mut tasks = lsr_ax(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(memory[(ABSOLUTE_ADDR_LO + OFFSET) as Word], 0x01);
        }

        #[test]
        fn should_take_six_cycles() {
            let mut memory =
                MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
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
            use std::cell::RefCell;

            use crate::cpu::{
                instructions::shifts::rol_acc,
                tests::{run_tasks, MemoryMock},
                CPU,
            };

            #[test]
            fn should_set_carry_when_bit_7_is_set_before_rotation() {
                let mut memory = MemoryMock::default();
                let mut cpu = CPU::new_nmos();
                cpu.accumulator = 0b10000000;

                assert_eq!(cpu.processor_status.get_carry_flag(), false);

                let mut tasks = rol_acc(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_carry_flag(), true);
            }

            #[test]
            fn should_not_change_carry_when_bit_7_is_not_set_before_rotation() {
                let mut memory = MemoryMock::default();
                let mut cpu = CPU::new_nmos();
                cpu.accumulator = 0b01111111;

                assert_eq!(cpu.processor_status.get_carry_flag(), false);

                let mut tasks = rol_acc(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_carry_flag(), false);
            }

            #[test]
            fn should_set_zero_when_value_after_shift_is_zero() {
                let mut memory = MemoryMock::default();
                let mut cpu = CPU::new_nmos();
                cpu.accumulator = 0b10000000;

                assert_eq!(cpu.processor_status.get_zero_flag(), false);

                let mut tasks = rol_acc(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_zero_flag(), true);
            }
        }

        mod mem {
            use std::cell::RefCell;

            use crate::{
                consts::Byte,
                cpu::{
                    instructions::shifts::rol_zp,
                    tests::{run_tasks, MemoryMock},
                    CPU,
                },
            };

            const ZERO_PAGE_ADDR: Byte = 0x01;

            #[test]
            fn should_set_carry_when_bit_7_is_set_before_rotation() {
                const VALUE: Byte = 0b10000000;
                let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
                let mut cpu = CPU::new_nmos();
                cpu.program_counter = 0x00;

                assert_eq!(cpu.processor_status.get_carry_flag(), false);

                let mut tasks = rol_zp(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_carry_flag(), true);
            }

            #[test]
            fn should_not_change_carry_when_bit_7_is_not_set_before_rotation() {
                const VALUE: Byte = 0b01111111;
                let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
                let mut cpu = CPU::new_nmos();
                cpu.program_counter = 0x00;

                assert_eq!(cpu.processor_status.get_carry_flag(), false);

                let mut tasks = rol_zp(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_carry_flag(), false);
            }

            #[test]
            fn should_set_zero_when_value_after_shift_is_zero() {
                const VALUE: Byte = 0b10000000;
                let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
                let mut cpu = CPU::new_nmos();
                cpu.program_counter = 0x00;

                assert_eq!(cpu.processor_status.get_zero_flag(), false);

                let mut tasks = rol_zp(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_zero_flag(), true);
            }
        }
    }

    #[cfg(test)]
    mod rol_acc {
        use std::cell::RefCell;

        use crate::{
            consts::Byte,
            cpu::{
                instructions::shifts::rol_acc,
                tests::{run_tasks, MemoryMock},
                CPU,
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
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{
                instructions::shifts::rol_zp,
                tests::{run_tasks, MemoryMock},
                CPU,
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
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{
                instructions::shifts::rol_zpx,
                tests::{run_tasks, MemoryMock},
                CPU,
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
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{
                instructions::shifts::rol_a,
                tests::{run_tasks, MemoryMock},
                CPU,
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
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{
                instructions::shifts::rol_ax,
                tests::{run_tasks, MemoryMock},
                CPU,
            },
        };

        const ABSOLUTE_ADDR_HI: Byte = 0x00;
        const ABSOLUTE_ADDR_LO: Byte = 0x03;
        const OFFSET: Byte = 0x01;
        const VALUE: Byte = 0x02;

        #[test]
        fn should_rotate_value_left_in_memory_at_absolute_address() {
            let mut memory =
                MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_x = OFFSET;
            cpu.program_counter = 0x00;

            let mut tasks = rol_ax(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(memory[(ABSOLUTE_ADDR_LO + OFFSET) as Word], 0b00000100);
        }

        #[test]
        fn should_set_bit_0_when_carry_is_set() {
            let mut memory =
                MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
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
            let mut memory =
                MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
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
            let mut memory =
                MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
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
            use std::cell::RefCell;

            use crate::cpu::{
                instructions::shifts::ror_acc,
                tests::{run_tasks, MemoryMock},
                CPU,
            };

            #[test]
            fn should_set_carry_when_bit_0_is_set() {
                let mut memory = MemoryMock::default();
                let mut cpu = CPU::new_nmos();
                cpu.accumulator = 0b00000001;

                assert_eq!(cpu.processor_status.get_carry_flag(), false);

                let mut tasks = ror_acc(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_carry_flag(), true);
            }

            #[test]
            fn should_not_change_carry_when_bit_0_is_not_set() {
                let mut memory = MemoryMock::default();
                let mut cpu = CPU::new_nmos();
                cpu.accumulator = 0b11111110;

                assert_eq!(cpu.processor_status.get_carry_flag(), false);

                let mut tasks = ror_acc(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_carry_flag(), false);
            }

            #[test]
            fn should_set_zero_when_value_after_shift_is_zero() {
                let mut memory = MemoryMock::default();
                let mut cpu = CPU::new_nmos();
                cpu.accumulator = 0b00000001;

                assert_eq!(cpu.processor_status.get_zero_flag(), false);

                let mut tasks = ror_acc(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_zero_flag(), true);
            }
        }

        mod mem {
            use std::cell::RefCell;

            use crate::{
                consts::Byte,
                cpu::{
                    instructions::shifts::ror_zp,
                    tests::{run_tasks, MemoryMock},
                    CPU,
                },
            };

            const ZERO_PAGE_ADDR: Byte = 0x01;

            #[test]
            fn should_set_carry_when_bit_0_is_set_before_rotation() {
                const VALUE: Byte = 0b00000001;
                let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
                let mut cpu = CPU::new_nmos();
                cpu.program_counter = 0x00;

                assert_eq!(cpu.processor_status.get_carry_flag(), false);

                let mut tasks = ror_zp(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_carry_flag(), true);
            }

            #[test]
            fn should_not_change_carry_when_bit_0_is_not_set_before_rotation() {
                const VALUE: Byte = 0b11111110;
                let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
                let mut cpu = CPU::new_nmos();
                cpu.program_counter = 0x00;

                assert_eq!(cpu.processor_status.get_carry_flag(), false);

                let mut tasks = ror_zp(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_carry_flag(), false);
            }

            #[test]
            fn should_set_zero_when_value_after_shift_is_zero() {
                const VALUE: Byte = 0b00000001;
                let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]);
                let mut cpu = CPU::new_nmos();
                cpu.program_counter = 0x00;

                assert_eq!(cpu.processor_status.get_zero_flag(), false);

                let mut tasks = ror_zp(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks, &mut memory);

                assert_eq!(cpu.processor_status.get_zero_flag(), true);
            }
        }
    }

    #[cfg(test)]
    mod ror_acc {
        use std::cell::RefCell;

        use crate::{
            consts::Byte,
            cpu::{
                instructions::shifts::ror_acc,
                tests::{run_tasks, MemoryMock},
                CPU,
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
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{
                instructions::shifts::ror_zp,
                tests::{run_tasks, MemoryMock},
                CPU,
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
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{
                instructions::shifts::ror_zpx,
                tests::{run_tasks, MemoryMock},
                CPU,
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
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{
                instructions::shifts::ror_a,
                tests::{run_tasks, MemoryMock},
                CPU,
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
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{
                instructions::shifts::ror_ax,
                tests::{run_tasks, MemoryMock},
                CPU,
            },
        };

        const ABSOLUTE_ADDR_HI: Byte = 0x00;
        const ABSOLUTE_ADDR_LO: Byte = 0x03;
        const OFFSET: Byte = 0x01;
        const VALUE: Byte = 0x02;

        #[test]
        fn should_rotate_value_right_in_memory_at_absolute_address() {
            let mut memory =
                MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_x = OFFSET;
            cpu.program_counter = 0x00;

            let mut tasks = ror_ax(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(memory[(ABSOLUTE_ADDR_LO + OFFSET) as Word], 0b00000001);
        }

        #[test]
        fn should_set_bit_7_when_carry_is_set() {
            let mut memory =
                MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
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
            let mut memory =
                MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
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
            let mut memory =
                MemoryMock::new(&[ABSOLUTE_ADDR_LO, ABSOLUTE_ADDR_HI, 0x00, 0x00, VALUE]);
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
