#[cfg(test)]
mod common_branch {
    use std::cell::RefCell;

    use crate::{
        consts::Byte,
        cpu::{instructions::branches::branch, tests::MemoryMock, CPU},
    };

    #[test]
    fn should_not_take_branch_and_advance_past_operand_when_condition_is_false() {
        let memory = &RefCell::new(MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x0000;

        let condition: fn(&CPU) -> bool = |_| false;
        branch(&mut cpu, condition);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_branch_and_offset_program_counter_by_operand_when_condition_is_true() {
        const OFFSET: Byte = 0x03;
        let memory = &RefCell::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;

        let condition: fn(&CPU) -> bool = |_| true;
        branch(&mut cpu, condition);

        assert_eq!(cpu.program_counter, 0x0004);
    }

    #[test]
    fn should_take_branch_and_offset_program_counter_backwards_by_negative_operand_in_twos_complement_when_condition_is_true(
    ) {
        const OFFSET: Byte = 0x03;
        const NEGATIVE_OFFSET_TWOS_COMPLEMENT: Byte = (OFFSET ^ 0xFF) + 1;
        let memory = &RefCell::new(MemoryMock::new(&[
            0x22,
            0x00,
            NEGATIVE_OFFSET_TWOS_COMPLEMENT,
            0x00,
        ]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x02;

        let condition: fn(&CPU) -> bool = |_| true;
        branch(&mut cpu, condition);

        assert_eq!(cpu.program_counter, 0x00);
    }

    #[test]
    fn should_take_and_offset_program_counter_over_page_flip_by_operand_when_condition_is_true() {
        const OFFSET: Byte = 0x04;
        let mut payload: [Byte; 512] = [0x00; 512];
        payload[0x00FE] = OFFSET;
        let memory = &RefCell::new(MemoryMock::new(&payload));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00FE;

        let condition: fn(&CPU) -> bool = |_| true;
        branch(&mut cpu, condition);

        assert_eq!(cpu.program_counter, 0x0103);
    }

    #[test]
    fn should_take_branch_and_offset_program_counter_backwards_over_page_flip_by_negative_operand_in_twos_complement_when_condition_is_true(
    ) {
        const OFFSET: Byte = 0x03;
        const NEGATIVE_OFFSET_TWOS_COMPLEMENT: Byte = (OFFSET ^ 0xFF) + 1;
        let memory = &RefCell::new(MemoryMock::new(&[
            NEGATIVE_OFFSET_TWOS_COMPLEMENT,
            0x00,
            0x00,
            0x00,
        ]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;

        let condition: fn(&CPU) -> bool = |_| true;
        branch(&mut cpu, condition);

        assert_eq!(cpu.program_counter, 0xFFFE);
    }

    #[test]
    fn should_take_one_cycle_when_not_branching() {
        let memory = &RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let condition: fn(&CPU) -> bool = |_| false;
        branch(&mut cpu, condition);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_take_two_cycles_when_branching_without_crossing_a_page_flip() {
        const OFFSET: Byte = 0x03;
        let memory = &RefCell::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let condition: fn(&CPU) -> bool = |_| true;
        branch(&mut cpu, condition);

        assert_eq!(cpu.cycle, 2);
    }

    #[test]
    fn should_take_three_cycles_when_branching_with_a_page_flips_crossing() {
        const OFFSET: Byte = 0x04;
        let mut payload: [Byte; 512] = [0x00; 512];
        payload[0x00FE] = OFFSET;
        let memory = &RefCell::new(MemoryMock::new(&payload));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00FE;
        cpu.cycle = 0;

        let condition: fn(&CPU) -> bool = |_| true;
        branch(&mut cpu, condition);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod bcc {
    use std::cell::RefCell;

    use crate::{
        consts::Byte,
        cpu::{instructions::bcc, tests::MemoryMock, CPU},
    };

    #[test]
    fn should_not_take_branch_when_carry_flag_is_set() {
        let memory = &RefCell::new(MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_carry_flag(true);
        cpu.program_counter = 0x00;

        bcc(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_clear() {
        const OFFSET: Byte = 0x03;
        let memory = &RefCell::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_carry_flag(false);
        cpu.program_counter = 0x00;

        bcc(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0004);
    }
}

#[cfg(test)]
mod bcs {
    use std::cell::RefCell;

    use crate::{
        consts::Byte,
        cpu::{instructions::bcs, tests::MemoryMock, CPU},
    };

    #[test]
    fn should_not_take_branch_when_carry_flag_is_clear() {
        let memory = &RefCell::new(MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_carry_flag(false);
        cpu.program_counter = 0x00;

        bcs(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_set() {
        const OFFSET: Byte = 0x03;
        let memory = &RefCell::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_carry_flag(true);
        cpu.program_counter = 0x00;

        bcs(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0004);
    }
}

#[cfg(test)]
mod beq {
    use std::cell::RefCell;

    use crate::{
        consts::Byte,
        cpu::{instructions::beq, tests::MemoryMock, CPU},
    };

    #[test]
    fn should_not_take_branch_when_zero_flag_is_clear() {
        let memory = &RefCell::new(MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_zero_flag(false);
        cpu.program_counter = 0x00;

        beq(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_set() {
        const OFFSET: Byte = 0x03;
        let memory = &RefCell::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_zero_flag(true);
        cpu.program_counter = 0x00;

        beq(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0004);
    }
}

#[cfg(test)]
mod bmi {
    use std::cell::RefCell;

    use crate::{
        consts::Byte,
        cpu::{instructions::bmi, tests::MemoryMock, CPU},
    };

    #[test]
    fn should_not_take_branch_when_negative_flag_is_clear() {
        let memory = &RefCell::new(MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_negative_flag(false);
        cpu.program_counter = 0x00;

        bmi(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_branch_when_negative_flag_is_set() {
        const OFFSET: Byte = 0x03;
        let memory = &RefCell::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_negative_flag(true);
        cpu.program_counter = 0x00;

        bmi(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0004);
    }
}

#[cfg(test)]
mod bne {
    use std::cell::RefCell;

    use crate::{
        consts::Byte,
        cpu::{instructions::bne, tests::MemoryMock, CPU},
    };

    #[test]
    fn should_not_take_branch_when_zero_flag_is_set() {
        let memory = &RefCell::new(MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_zero_flag(true);
        cpu.program_counter = 0x00;

        bne(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_clear() {
        const OFFSET: Byte = 0x03;
        let memory = &RefCell::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_zero_flag(false);
        cpu.program_counter = 0x00;

        bne(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0004);
    }
}

#[cfg(test)]
mod bpl {
    use std::cell::RefCell;

    use crate::{
        consts::Byte,
        cpu::{instructions::bpl, tests::MemoryMock, CPU},
    };

    #[test]
    fn should_not_take_branch_when_negative_flag_is_set() {
        let memory = &RefCell::new(MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_negative_flag(true);
        cpu.program_counter = 0x00;

        bpl(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_branch_when_negative_flag_is_clear() {
        const OFFSET: Byte = 0x03;
        let memory = &RefCell::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_negative_flag(false);
        cpu.program_counter = 0x00;

        bpl(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0004);
    }
}

#[cfg(test)]
mod bvc {
    use std::cell::RefCell;

    use crate::{
        consts::Byte,
        cpu::{instructions::bvc, tests::MemoryMock, CPU},
    };

    #[test]
    fn should_not_take_branch_when_negative_flag_is_set() {
        let memory = &RefCell::new(MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_overflow_flag(true);
        cpu.program_counter = 0x00;

        bvc(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_branch_when_negative_flag_is_clear() {
        const OFFSET: Byte = 0x03;
        let memory = &RefCell::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_overflow_flag(false);
        cpu.program_counter = 0x00;

        bvc(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0004);
    }
}

#[cfg(test)]
mod bvs {
    use std::cell::RefCell;

    use crate::{
        consts::Byte,
        cpu::{instructions::bvs, tests::MemoryMock, CPU},
    };

    #[test]
    fn should_not_take_branch_when_negative_flag_is_clear() {
        let memory = &RefCell::new(MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_overflow_flag(false);
        cpu.program_counter = 0x00;

        bvs(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_branch_when_negative_flag_is_set() {
        const OFFSET: Byte = 0x03;
        let memory = &RefCell::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_overflow_flag(true);
        cpu.program_counter = 0x00;

        bvs(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0004);
    }
}
