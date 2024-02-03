#[cfg(test)]
mod dex_im {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::dex_im, tests::MemoryMock, CPU};

    #[test]
    fn should_decrement_x_register() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.index_register_x = 0x02;

        dex_im(&mut cpu);

        assert_eq!(cpu.index_register_x, 0x01);
    }

    #[test]
    fn should_take_one_cycle() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.index_register_x = 0x02;
        cpu.cycle = 0;

        dex_im(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_set_processor_status_of_x_register_after_decrement() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.index_register_x = 0x01;

        dex_im(&mut cpu);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}

#[cfg(test)]
mod dey_im {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::dey_im, tests::MemoryMock, CPU};

    #[test]
    fn should_decrement_y_register() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.index_register_y = 0x02;

        dey_im(&mut cpu);

        assert_eq!(cpu.index_register_y, 0x01);
    }

    #[test]
    fn should_take_one_cycle() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.index_register_y = 0x02;
        cpu.cycle = 0;

        dey_im(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_set_processor_status_of_y_register_after_decrement() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.index_register_y = 0x01;

        dey_im(&mut cpu);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}

#[cfg(test)]
mod dec_zp {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::dec_zp, tests::MemoryMock, Byte, Word, CPU};

    const VALUE: Byte = 0x02;
    const ZERO_PAGE_ADDR: Byte = 0x03;

    #[test]
    fn should_decrement_value_stored_in_memory_at_zero_page_address() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            VALUE,
        ]))));
        cpu.program_counter = 0x00;

        dec_zp(&mut cpu);

        assert_eq!(cpu.memory.borrow()[ZERO_PAGE_ADDR as Word], 0x01);
    }

    #[test]
    fn should_take_four_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            VALUE,
        ]))));
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        dec_zp(&mut cpu);

        assert_eq!(cpu.cycle, 4);
    }

    #[test]
    fn should_set_processor_status_of_value_in_memory() {
        const VALUE: Byte = 0x01;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            VALUE,
        ]))));
        cpu.program_counter = 0x00;

        dec_zp(&mut cpu);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}

#[cfg(test)]
mod dec_zpx {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::dec_zpx, tests::MemoryMock, Byte, Word, CPU};

    const VALUE: Byte = 0x09;
    const ZERO_PAGE_ADDR: Byte = 0x01;
    const ZERO_PAGE_ADDR_SUM_X: Word = 0x03;

    #[test]
    fn should_decrement_value_stored_in_memory_at_zero_page_address_summed_with_index_register_x() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            VALUE,
        ]))));
        cpu.program_counter = 0x00;
        cpu.index_register_x = 0x02;

        dec_zpx(&mut cpu);

        assert_eq!(cpu.memory.borrow()[ZERO_PAGE_ADDR_SUM_X as Word], 0x08);
    }

    #[test]
    fn should_take_five_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            VALUE,
        ]))));
        cpu.program_counter = 0x00;
        cpu.index_register_x = 0x02;
        cpu.cycle = 0;

        dec_zpx(&mut cpu);

        assert_eq!(cpu.cycle, 5);
    }

    #[test]
    fn should_set_processor_status_of_value_in_memory() {
        const VALUE: Byte = 0x01;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            VALUE,
        ]))));
        cpu.program_counter = 0x00;
        cpu.index_register_x = 0x02;

        dec_zpx(&mut cpu);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}

#[cfg(test)]
mod dec_a {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::dec_a, tests::MemoryMock, Byte, Word, CPU};

    const VALUE: Byte = 0x09;
    const ADDR_LO: Byte = 0x04;
    const ADDR_HI: Byte = 0x00;
    const ADDR: Word = 0x0004;

    #[test]
    fn should_decrement_value_stored_in_memory_at_absolute_address() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
        ]))));
        cpu.program_counter = 0x00;

        dec_a(&mut cpu);

        assert_eq!(cpu.memory.borrow()[ADDR as Word], 0x08);
    }

    #[test]
    fn should_take_five_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
        ]))));
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        dec_a(&mut cpu);

        assert_eq!(cpu.cycle, 5);
    }

    #[test]
    fn should_set_processor_status_of_value_in_memory() {
        const VALUE: Byte = 0x01;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
        ]))));
        cpu.program_counter = 0x00;

        dec_a(&mut cpu);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}

#[cfg(test)]
mod dec_ax {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::dec_ax, tests::MemoryMock, Byte, Word, CPU};

    const VALUE: Byte = 0x09;
    const ADDR_LO: Byte = 0x02;
    const ADDR_HI: Byte = 0x00;
    const OFFSET: Byte = 0x02;
    const ADDR_OFFSET_BY_X: Word = 0x0004;

    #[test]
    fn should_decrement_value_stored_in_memory_at_absolute_address_offset_by_index_register_x() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
        ]))));
        cpu.program_counter = 0x00;
        cpu.index_register_x = OFFSET;

        dec_ax(&mut cpu);

        assert_eq!(cpu.memory.borrow()[ADDR_OFFSET_BY_X], 0x08);
    }

    #[test]
    fn should_take_six_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
        ]))));
        cpu.program_counter = 0x00;
        cpu.index_register_x = OFFSET;
        cpu.cycle = 0;

        dec_ax(&mut cpu);

        assert_eq!(cpu.cycle, 6);
    }

    #[test]
    fn should_set_processor_status_of_value_in_memory() {
        const VALUE: Byte = 0x01;
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
        ]))));
        cpu.program_counter = 0x00;
        cpu.index_register_x = OFFSET;

        dec_ax(&mut cpu);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}
