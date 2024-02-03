#[cfg(test)]
mod beq {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        consts::Byte,
        cpu::{beq, tests::MemoryMock, CPU},
    };

    #[test]
    fn should_not_take_branch_when_zero_flag_is_clear_and_advance_past_operand() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            0x22, 0x00, 0x01, 0x00,
        ]))));
        cpu.processor_status.change_zero_flag(false);
        cpu.program_counter = 0x00;

        beq(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_set_and_offset_program_counter_by_operand() {
        const OFFSET: Byte = 0x03;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            OFFSET, 0x00, 0x01, 0x00,
        ]))));
        cpu.processor_status.change_zero_flag(true);
        cpu.program_counter = 0x00;

        beq(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0004);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_set_and_offset_program_counter_backwards_by_negative_operand_in_twos_complement(
    ) {
        const OFFSET: Byte = 0x03;
        const NEGATIVE_OFFSET_TWOS_COMPLEMENT: Byte = (OFFSET ^ 0xFF) + 1;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            0x22,
            0x00,
            NEGATIVE_OFFSET_TWOS_COMPLEMENT,
            0x00,
        ]))));
        cpu.processor_status.change_zero_flag(true);
        cpu.program_counter = 0x02;

        beq(&mut cpu);

        assert_eq!(cpu.program_counter, 0x00);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_set_and_offset_program_counter_over_page_flip_by_operand(
    ) {
        const OFFSET: Byte = 0x04;
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x00FE] = OFFSET;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&memory))));
        cpu.processor_status.change_zero_flag(true);
        cpu.program_counter = 0x00FE;
        cpu.cycle = 0;

        beq(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0103);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_set_and_offset_program_counter_backwards_over_page_flip_by_negative_operand_in_twos_complement(
    ) {
        const OFFSET: Byte = 0x03;
        const NEGATIVE_OFFSET_TWOS_COMPLEMENT: Byte = (OFFSET ^ 0xFF) + 1;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            NEGATIVE_OFFSET_TWOS_COMPLEMENT,
            0x00,
            0x00,
            0x00,
        ]))));
        cpu.processor_status.change_zero_flag(true);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        beq(&mut cpu);

        assert_eq!(cpu.program_counter, 0xFFFE);
    }

    #[test]
    fn should_take_one_cycle_when_not_branching() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            0x02, 0x00, 0x01, 0x00,
        ]))));
        cpu.processor_status.change_zero_flag(false);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        beq(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_take_two_cycles_when_branching_without_crossing_a_page_flip() {
        const OFFSET: Byte = 0x03;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            OFFSET, 0x00, 0x01, 0x00,
        ]))));
        cpu.processor_status.change_zero_flag(true);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        beq(&mut cpu);

        assert_eq!(cpu.cycle, 2);
    }

    #[test]
    fn should_take_three_cycles_when_branching_with_a_page_flips_crossing() {
        const OFFSET: Byte = 0x04;
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x00FE] = OFFSET;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&memory))));
        cpu.processor_status.change_zero_flag(true);
        cpu.program_counter = 0x00FE;
        cpu.cycle = 0;

        beq(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod bne {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        consts::Byte,
        cpu::{bne, tests::MemoryMock, CPU},
    };

    #[test]
    fn should_not_take_branch_when_zero_flag_is_set_and_advance_past_operand() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            0x22, 0x00, 0x01, 0x00,
        ]))));
        cpu.processor_status.change_zero_flag(true);
        cpu.program_counter = 0x00;

        bne(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_clear_and_offset_program_counter_by_operand() {
        const OFFSET: Byte = 0x03;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            OFFSET, 0x00, 0x01, 0x00,
        ]))));
        cpu.processor_status.change_zero_flag(false);
        cpu.program_counter = 0x00;

        bne(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0004);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_clear_and_offset_program_counter_backwards_by_negative_operand_in_twos_complement(
    ) {
        const OFFSET: Byte = 0x03;
        const NEGATIVE_OFFSET_TWOS_COMPLEMENT: Byte = (OFFSET ^ 0xFF) + 1;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            0x22,
            0x00,
            NEGATIVE_OFFSET_TWOS_COMPLEMENT,
            0x00,
        ]))));
        cpu.processor_status.change_zero_flag(false);
        cpu.program_counter = 0x02;

        bne(&mut cpu);

        assert_eq!(cpu.program_counter, 0x00);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_clear_and_offset_program_counter_over_page_flip_by_operand(
    ) {
        const OFFSET: Byte = 0x04;
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x00FE] = OFFSET;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&memory))));
        cpu.processor_status.change_zero_flag(false);
        cpu.program_counter = 0x00FE;
        cpu.cycle = 0;

        bne(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0103);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_clear_and_offset_program_counter_backwards_over_page_flip_by_negative_operand_in_twos_complement(
    ) {
        const OFFSET: Byte = 0x03;
        const NEGATIVE_OFFSET_TWOS_COMPLEMENT: Byte = (OFFSET ^ 0xFF) + 1;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            NEGATIVE_OFFSET_TWOS_COMPLEMENT,
            0x00,
            0x00,
            0x00,
        ]))));
        cpu.processor_status.change_zero_flag(false);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bne(&mut cpu);

        assert_eq!(cpu.program_counter, 0xFFFE);
    }

    #[test]
    fn should_take_one_cycle_when_not_branching() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            0x02, 0x00, 0x01, 0x00,
        ]))));
        cpu.processor_status.change_zero_flag(true);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bne(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_take_two_cycles_when_branching_without_crossing_a_page_flip() {
        const OFFSET: Byte = 0x03;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            OFFSET, 0x00, 0x01, 0x00,
        ]))));
        cpu.processor_status.change_zero_flag(false);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bne(&mut cpu);

        assert_eq!(cpu.cycle, 2);
    }

    #[test]
    fn should_take_three_cycles_when_branching_with_a_page_flips_crossing() {
        const OFFSET: Byte = 0x04;
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x00FE] = OFFSET;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&memory))));
        cpu.processor_status.change_zero_flag(false);
        cpu.program_counter = 0x00FE;
        cpu.cycle = 0;

        bne(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod bcs {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        consts::Byte,
        cpu::{bcs, tests::MemoryMock, CPU},
    };

    #[test]
    fn should_not_take_branch_when_carry_flag_is_clear_and_advance_past_operand() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            0x22, 0x00, 0x01, 0x00,
        ]))));
        cpu.processor_status.change_carry_flag(false);
        cpu.program_counter = 0x00;

        bcs(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_set_and_offset_program_counter_by_operand() {
        const OFFSET: Byte = 0x03;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            OFFSET, 0x00, 0x01, 0x00,
        ]))));
        cpu.processor_status.change_carry_flag(true);
        cpu.program_counter = 0x00;

        bcs(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0004);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_set_and_offset_program_counter_backwards_by_negative_operand_in_twos_complement(
    ) {
        const OFFSET: Byte = 0x03;
        const NEGATIVE_OFFSET_TWOS_COMPLEMENT: Byte = (OFFSET ^ 0xFF) + 1;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            0x22,
            0x00,
            NEGATIVE_OFFSET_TWOS_COMPLEMENT,
            0x00,
        ]))));
        cpu.processor_status.change_carry_flag(true);
        cpu.program_counter = 0x02;

        bcs(&mut cpu);

        assert_eq!(cpu.program_counter, 0x00);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_set_and_offset_program_counter_over_page_flip_by_operand(
    ) {
        const OFFSET: Byte = 0x04;
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x00FE] = OFFSET;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&memory))));
        cpu.processor_status.change_carry_flag(true);
        cpu.program_counter = 0x00FE;
        cpu.cycle = 0;

        bcs(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0103);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_set_and_offset_program_counter_backwards_over_page_flip_by_negative_operand_in_twos_complement(
    ) {
        const OFFSET: Byte = 0x03;
        const NEGATIVE_OFFSET_TWOS_COMPLEMENT: Byte = (OFFSET ^ 0xFF) + 1;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            NEGATIVE_OFFSET_TWOS_COMPLEMENT,
            0x00,
            0x00,
            0x00,
        ]))));
        cpu.processor_status.change_carry_flag(true);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bcs(&mut cpu);

        assert_eq!(cpu.program_counter, 0xFFFE);
    }

    #[test]
    fn should_take_one_cycle_when_not_branching() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            0x02, 0x00, 0x01, 0x00,
        ]))));
        cpu.processor_status.change_carry_flag(false);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bcs(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_take_two_cycles_when_branching_without_crossing_a_page_flip() {
        const OFFSET: Byte = 0x03;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            OFFSET, 0x00, 0x01, 0x00,
        ]))));
        cpu.processor_status.change_carry_flag(true);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bcs(&mut cpu);

        assert_eq!(cpu.cycle, 2);
    }

    #[test]
    fn should_take_three_cycles_when_branching_with_a_page_flips_crossing() {
        const OFFSET: Byte = 0x04;
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x00FE] = OFFSET;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&memory))));
        cpu.processor_status.change_carry_flag(true);
        cpu.program_counter = 0x00FE;
        cpu.cycle = 0;

        bcs(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod bcc {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        consts::Byte,
        cpu::{bcc, tests::MemoryMock, CPU},
    };

    #[test]
    fn should_not_take_branch_when_carry_flag_is_set_and_advance_past_operand() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            0x22, 0x00, 0x01, 0x00,
        ]))));
        cpu.processor_status.change_carry_flag(true);
        cpu.program_counter = 0x00;

        bcc(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_clear_and_offset_program_counter_by_operand() {
        const OFFSET: Byte = 0x03;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            OFFSET, 0x00, 0x01, 0x00,
        ]))));
        cpu.processor_status.change_carry_flag(false);
        cpu.program_counter = 0x00;

        bcc(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0004);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_clear_and_offset_program_counter_backwards_by_negative_operand_in_twos_complement(
    ) {
        const OFFSET: Byte = 0x03;
        const NEGATIVE_OFFSET_TWOS_COMPLEMENT: Byte = (OFFSET ^ 0xFF) + 1;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            0x22,
            0x00,
            NEGATIVE_OFFSET_TWOS_COMPLEMENT,
            0x00,
        ]))));
        cpu.processor_status.change_carry_flag(false);
        cpu.program_counter = 0x02;

        bcc(&mut cpu);

        assert_eq!(cpu.program_counter, 0x00);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_clear_and_offset_program_counter_over_page_flip_by_operand(
    ) {
        const OFFSET: Byte = 0x04;
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x00FE] = OFFSET;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&memory))));
        cpu.processor_status.change_carry_flag(false);
        cpu.program_counter = 0x00FE;
        cpu.cycle = 0;

        bcc(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0103);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_clear_and_offset_program_counter_backwards_over_page_flip_by_negative_operand_in_twos_complement(
    ) {
        const OFFSET: Byte = 0x03;
        const NEGATIVE_OFFSET_TWOS_COMPLEMENT: Byte = (OFFSET ^ 0xFF) + 1;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            NEGATIVE_OFFSET_TWOS_COMPLEMENT,
            0x00,
            0x00,
            0x00,
        ]))));
        cpu.processor_status.change_carry_flag(false);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bcc(&mut cpu);

        assert_eq!(cpu.program_counter, 0xFFFE);
    }

    #[test]
    fn should_take_one_cycle_when_not_branching() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            0x02, 0x00, 0x01, 0x00,
        ]))));
        cpu.processor_status.change_carry_flag(true);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bcc(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_take_two_cycles_when_branching_without_crossing_a_page_flip() {
        const OFFSET: Byte = 0x03;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            OFFSET, 0x00, 0x01, 0x00,
        ]))));
        cpu.processor_status.change_carry_flag(false);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bcc(&mut cpu);

        assert_eq!(cpu.cycle, 2);
    }

    #[test]
    fn should_take_three_cycles_when_branching_with_a_page_flips_crossing() {
        const OFFSET: Byte = 0x04;
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x00FE] = OFFSET;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&memory))));
        cpu.processor_status.change_carry_flag(false);
        cpu.program_counter = 0x00FE;
        cpu.cycle = 0;

        bcc(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }
}
