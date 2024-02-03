#[cfg(test)]
mod sta_zp {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::sta_zp, tests::MemoryMock, Byte, Word, CPU};

    const ZERO_PAGE_ADDR: Byte = 0x03;

    #[test]
    fn should_store_accumulator_in_memory_at_a_zero_page_address() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            0x00,
        ]))));
        cpu.accumulator = 0x02;
        cpu.program_counter = 0x00;

        sta_zp(&mut cpu);

        assert_eq!(cpu.memory.borrow()[ZERO_PAGE_ADDR as Word], 0x02);
    }

    #[test]
    fn should_take_two_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            0x00,
        ]))));
        cpu.accumulator = 0x02;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        sta_zp(&mut cpu);

        assert_eq!(cpu.cycle, 2);
    }
}

#[cfg(test)]
mod sta_zpx {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::sta_zpx, tests::MemoryMock, Byte, Word, CPU};

    const ZERO_PAGE_ADDR: Byte = 0x01;
    const ZERO_PAGE_ADDR_SUM_X: Word = 0x03;

    #[test]
    fn should_store_accumulator_in_memory_at_a_zero_page_address_summed_with_index_register_x() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            0x00,
        ]))));
        cpu.accumulator = 0x05;
        cpu.index_register_x = 0x02;
        cpu.program_counter = 0x00;

        sta_zpx(&mut cpu);

        assert_eq!(cpu.memory.borrow()[ZERO_PAGE_ADDR_SUM_X], 0x05);
    }

    #[test]
    fn should_take_three_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            0x00,
        ]))));
        cpu.accumulator = 0x05;
        cpu.index_register_x = 0x02;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        sta_zpx(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod sta_a {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::sta_a, tests::MemoryMock, Byte, Word, CPU};

    const ADDR_LO: Byte = 0x04;
    const ADDR_HI: Byte = 0x00;
    const ADDR: Word = 0x0004;

    #[test]
    fn should_store_accumulator_in_memory_at_an_absolute_address() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
        ]))));
        cpu.accumulator = 0x0A;
        cpu.program_counter = 0x00;

        sta_a(&mut cpu);

        assert_eq!(cpu.memory.borrow()[ADDR as Word], 0x0A);
    }

    #[test]
    fn should_take_three_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
        ]))));
        cpu.accumulator = 0x0A;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        sta_a(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod sta_ax {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::sta_ax, tests::MemoryMock, Byte, Word, CPU};

    const ADDR_LO: Byte = 0x02;
    const ADDR_HI: Byte = 0x00;
    const OFFSET: Byte = 0x02;
    const ADDR_OFFSET_BY_X: Word = 0x0004;

    #[test]
    fn should_store_accumulator_in_memory_at_an_absolute_address_offset_by_index_register_x() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
        ]))));
        cpu.accumulator = 0x08;
        cpu.program_counter = 0x00;
        cpu.index_register_x = OFFSET;

        sta_ax(&mut cpu);

        assert_eq!(cpu.memory.borrow()[ADDR_OFFSET_BY_X], 0x08);
    }

    #[test]
    fn should_take_four_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
        ]))));
        cpu.accumulator = 0x08;
        cpu.program_counter = 0x00;
        cpu.index_register_x = OFFSET;
        cpu.cycle = 0;

        sta_ax(&mut cpu);

        assert_eq!(cpu.cycle, 4);
    }
}

#[cfg(test)]
mod sta_ay {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::sta_ay, tests::MemoryMock, Byte, Word, CPU};

    const ADDR_LO: Byte = 0x02;
    const ADDR_HI: Byte = 0x00;
    const OFFSET: Byte = 0x02;
    const ADDR_OFFSET_BY_Y: Word = 0x0004;

    #[test]
    fn should_store_accumulator_in_memory_at_an_absolute_address_offset_by_index_register_y() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
        ]))));
        cpu.accumulator = 0x08;
        cpu.program_counter = 0x00;
        cpu.index_register_y = OFFSET;

        sta_ay(&mut cpu);

        assert_eq!(cpu.memory.borrow()[ADDR_OFFSET_BY_Y], 0x08);
    }

    #[test]
    fn should_take_four_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
        ]))));
        cpu.accumulator = 0x08;
        cpu.program_counter = 0x00;
        cpu.index_register_y = OFFSET;
        cpu.cycle = 0;

        sta_ay(&mut cpu);

        assert_eq!(cpu.cycle, 4);
    }
}

#[cfg(test)]
mod sta_inx {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::sta_inx, tests::MemoryMock, Byte, Word, CPU};

    const ZP_ADDRESS: Byte = 0x02;
    const OFFSET: Byte = 0x01;
    const EFFECTIVE_ADDRESS_LO: Byte = 0x05;
    const EFFECTIVE_ADDRESS_HI: Byte = 0x00;
    const EFFECTIVE_ADDRESS: Word = 0x0005;

    #[test]
    fn should_store_accumulator_in_an_indirect_adress_stored_in_zero_page_offset_with_index_register_x(
    ) {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZP_ADDRESS,
            0x00,
            0x00,
            EFFECTIVE_ADDRESS_LO,
            EFFECTIVE_ADDRESS_HI,
            0x00,
            0x00,
        ]))));
        cpu.program_counter = 0x00;
        cpu.accumulator = 0xA9;
        cpu.index_register_x = OFFSET;

        sta_inx(&mut cpu);

        assert_eq!(cpu.memory.borrow()[EFFECTIVE_ADDRESS], 0xA9);
    }

    #[test]
    fn should_take_five_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZP_ADDRESS,
            0x00,
            0x00,
            EFFECTIVE_ADDRESS_LO,
            EFFECTIVE_ADDRESS_HI,
            0x00,
            0x00,
        ]))));
        cpu.program_counter = 0x00;
        cpu.accumulator = 0xA9;
        cpu.index_register_x = OFFSET;
        cpu.cycle = 0;

        sta_inx(&mut cpu);

        assert_eq!(cpu.cycle, 5);
    }
}

#[cfg(test)]
mod sta_iny {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::sta_iny, tests::MemoryMock, Byte, Word, CPU};

    const ZP_ADDRESS: Byte = 0x01;
    const ADDRESS_LO: Byte = 0x03;
    const ADDRESS_HI: Byte = 0x00;
    const OFFSET: Byte = 0x01;
    const EFFECTIVE_ADDRESS: Word = 0x0004;

    #[test]
    fn should_store_accumulator_in_offset_with_index_register_y_indirect_adress_stored_in_zero_page(
    ) {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZP_ADDRESS, ADDRESS_LO, ADDRESS_HI, 0x00, 0x00,
        ]))));
        cpu.accumulator = 0xDF;
        cpu.index_register_y = OFFSET;
        cpu.program_counter = 0x00;

        sta_iny(&mut cpu);

        assert_eq!(cpu.memory.borrow()[EFFECTIVE_ADDRESS], 0xDF);
    }

    #[test]
    fn should_take_five_cycles_when_summing_indirect_address_with_index_y_crosses_page_flip() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZP_ADDRESS, ADDRESS_LO, ADDRESS_HI, 0x00, 0x00,
        ]))));
        cpu.accumulator = 0xDF;
        cpu.index_register_y = OFFSET;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        sta_iny(&mut cpu);

        assert_eq!(cpu.cycle, 5);
    }
}

#[cfg(test)]
mod stx_zp {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::stx_zp, tests::MemoryMock, Byte, Word, CPU};

    const ZERO_PAGE_ADDR: Byte = 0x03;

    #[test]
    fn should_store_index_register_x_in_memory_at_a_zero_page_address() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            0x00,
        ]))));
        cpu.index_register_x = 0x02;
        cpu.program_counter = 0x00;

        stx_zp(&mut cpu);

        assert_eq!(cpu.memory.borrow()[ZERO_PAGE_ADDR as Word], 0x02);
    }

    #[test]
    fn should_take_two_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            0x00,
        ]))));
        cpu.index_register_x = 0x02;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        stx_zp(&mut cpu);

        assert_eq!(cpu.cycle, 2);
    }
}

#[cfg(test)]
mod stx_zpy {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::stx_zpy, tests::MemoryMock, Byte, Word, CPU};

    const ZERO_PAGE_ADDR: Byte = 0x01;
    const ZERO_PAGE_ADDR_SUM_Y: Word = 0x03;

    #[test]
    fn should_store_index_register_x_in_memory_at_a_zero_page_address_summed_with_index_register_y()
    {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            0x00,
        ]))));
        cpu.index_register_x = 0x05;
        cpu.index_register_y = 0x02;
        cpu.program_counter = 0x00;

        stx_zpy(&mut cpu);

        assert_eq!(cpu.memory.borrow()[ZERO_PAGE_ADDR_SUM_Y], 0x05);
    }

    #[test]
    fn should_take_three_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            0x00,
        ]))));
        cpu.index_register_x = 0x05;
        cpu.index_register_y = 0x02;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        stx_zpy(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod stx_a {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::stx_a, tests::MemoryMock, Byte, Word, CPU};

    const ADDR_LO: Byte = 0x04;
    const ADDR_HI: Byte = 0x00;
    const ADDR: Word = 0x0004;

    #[test]
    fn should_store_index_register_x_in_memory_at_an_absolute_address() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
        ]))));
        cpu.index_register_x = 0x0A;
        cpu.program_counter = 0x00;

        stx_a(&mut cpu);

        assert_eq!(cpu.memory.borrow()[ADDR as Word], 0x0A);
    }

    #[test]
    fn should_take_three_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
        ]))));
        cpu.index_register_x = 0x0A;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        stx_a(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod sty_zp {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::sty_zp, tests::MemoryMock, Byte, Word, CPU};

    const ZERO_PAGE_ADDR: Byte = 0x03;

    #[test]
    fn should_store_index_register_y_in_memory_at_a_zero_page_address() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            0x00,
        ]))));
        cpu.index_register_y = 0x02;
        cpu.program_counter = 0x00;

        sty_zp(&mut cpu);

        assert_eq!(cpu.memory.borrow()[ZERO_PAGE_ADDR as Word], 0x02);
    }

    #[test]
    fn should_take_two_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            0x00,
        ]))));
        cpu.index_register_y = 0x02;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        sty_zp(&mut cpu);

        assert_eq!(cpu.cycle, 2);
    }
}

#[cfg(test)]
mod sty_zpx {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::sty_zpx, tests::MemoryMock, Byte, Word, CPU};

    const ZERO_PAGE_ADDR: Byte = 0x01;
    const ZERO_PAGE_ADDR_SUM_X: Word = 0x03;

    #[test]
    fn should_store_index_register_y_in_memory_at_a_zero_page_address_summed_with_index_register_x()
    {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            0x00,
        ]))));
        cpu.index_register_y = 0x05;
        cpu.index_register_x = 0x02;
        cpu.program_counter = 0x00;

        sty_zpx(&mut cpu);

        assert_eq!(cpu.memory.borrow()[ZERO_PAGE_ADDR_SUM_X], 0x05);
    }

    #[test]
    fn should_take_three_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ZERO_PAGE_ADDR,
            0xFF,
            0x00,
            0x00,
        ]))));
        cpu.index_register_y = 0x05;
        cpu.index_register_x = 0x02;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        sty_zpx(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod sty_a {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::sty_a, tests::MemoryMock, Byte, Word, CPU};

    const ADDR_LO: Byte = 0x04;
    const ADDR_HI: Byte = 0x00;
    const ADDR: Word = 0x0004;

    #[test]
    fn should_store_index_register_y_in_memory_at_an_absolute_address() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
        ]))));
        cpu.index_register_y = 0x0A;
        cpu.program_counter = 0x00;

        sty_a(&mut cpu);

        assert_eq!(cpu.memory.borrow()[ADDR as Word], 0x0A);
    }

    #[test]
    fn should_take_three_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
        ]))));
        cpu.index_register_y = 0x0A;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        sty_a(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }
}
