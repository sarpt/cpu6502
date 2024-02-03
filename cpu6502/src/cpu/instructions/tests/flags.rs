#[cfg(test)]
mod clc {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::clc, tests::MemoryMock, CPU};

    #[test]
    fn should_clear_carry_flag() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.processor_status.change_carry_flag(true);

        clc(&mut cpu);

        assert_eq!(cpu.processor_status.get_carry_flag(), false);
    }

    #[test]
    fn should_take_one_cycle() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.processor_status.change_carry_flag(true);
        cpu.cycle = 0;

        clc(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }
}

#[cfg(test)]
mod cld {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::cld, tests::MemoryMock, CPU};

    #[test]
    fn should_clear_decimal_flag() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.processor_status.change_decimal_mode_flag(true);

        cld(&mut cpu);

        assert_eq!(cpu.processor_status.get_decimal_mode_flag(), false);
    }

    #[test]
    fn should_take_one_cycle() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.processor_status.change_decimal_mode_flag(true);
        cpu.cycle = 0;

        cld(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }
}

#[cfg(test)]
mod cli {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::cli, tests::MemoryMock, CPU};

    #[test]
    fn should_clear_interrupt_disable_flag() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.processor_status.change_interrupt_disable_flag(true);

        cli(&mut cpu);

        assert_eq!(cpu.processor_status.get_interrupt_disable_flag(), false);
    }

    #[test]
    fn should_take_one_cycle() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.processor_status.change_interrupt_disable_flag(true);
        cpu.cycle = 0;

        cli(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }
}

#[cfg(test)]
mod clv {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::clv, tests::MemoryMock, CPU};

    #[test]
    fn should_clear_overflow_flag() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.processor_status.change_overflow_flag(true);

        clv(&mut cpu);

        assert_eq!(cpu.processor_status.get_overflow_flag(), false);
    }

    #[test]
    fn should_take_one_cycle() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.processor_status.change_overflow_flag(true);
        cpu.cycle = 0;

        clv(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }
}

#[cfg(test)]
mod sec {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::sec, tests::MemoryMock, CPU};

    #[test]
    fn should_set_carry_flag() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.processor_status.change_carry_flag(false);

        sec(&mut cpu);

        assert_eq!(cpu.processor_status.get_carry_flag(), true);
    }

    #[test]
    fn should_take_one_cycle() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.processor_status.change_carry_flag(false);
        cpu.cycle = 0;

        sec(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }
}

#[cfg(test)]
mod sed {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::sed, tests::MemoryMock, CPU};

    #[test]
    fn should_set_decimal_mode_flag() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.processor_status.change_decimal_mode_flag(false);

        sed(&mut cpu);

        assert_eq!(cpu.processor_status.get_decimal_mode_flag(), true);
    }

    #[test]
    fn should_take_one_cycle() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.processor_status.change_decimal_mode_flag(false);
        cpu.cycle = 0;

        sed(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }
}

#[cfg(test)]
mod sei {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::sei, tests::MemoryMock, CPU};

    #[test]
    fn should_set_interrupt_disable_flag() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.processor_status.change_interrupt_disable_flag(false);

        sei(&mut cpu);

        assert_eq!(cpu.processor_status.get_interrupt_disable_flag(), true);
    }

    #[test]
    fn should_take_one_cycle() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.processor_status.change_interrupt_disable_flag(false);
        cpu.cycle = 0;

        sei(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }
}

#[cfg(test)]
mod brk {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        consts::Byte,
        cpu::{instructions::brk, tests::MemoryMock, CPU},
    };

    #[test]
    fn should_put_program_counter_incremented_by_one_and_processor_status_on_stack() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.processor_status.set(0b11111111);
        cpu.stack_pointer = 0xFF;
        cpu.program_counter = 0xABCD;

        brk(&mut cpu);

        assert_eq!(cpu.memory.borrow()[0x01FF], 0xCE);
        assert_eq!(cpu.memory.borrow()[0x01FE], 0xAB);
        assert_eq!(cpu.memory.borrow()[0x01FD], 0b11111111);
    }

    #[test]
    fn should_jump_to_address_stored_in_brk_vector() {
        const ADDR_LO: Byte = 0xAD;
        const ADDR_HI: Byte = 0x9B;
        let mut memory = MemoryMock::default();
        memory[0xFFFE] = ADDR_LO;
        memory[0xFFFF] = ADDR_HI;

        let mut cpu = CPU::new(Rc::new(RefCell::new(memory)));
        cpu.program_counter = 0x00;

        brk(&mut cpu);

        assert_eq!(cpu.program_counter, 0x9BAD);
    }

    #[test]
    fn should_set_break_processor_status_flag() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.program_counter = 0x00;
        cpu.processor_status.change_break_flag(false);

        brk(&mut cpu);

        assert_eq!(cpu.processor_status.get_break_flag(), true);
    }

    #[test]
    fn should_take_six_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        brk(&mut cpu);

        assert_eq!(cpu.cycle, 6);
    }
}
