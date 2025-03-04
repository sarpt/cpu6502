#[cfg(test)]
mod inx_im {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::inx_im,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_increment_x_register() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_x = 0x02;

        let tasks = inx_im(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.index_register_x, 0x03);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_x = 0x02;
        cpu.cycle = 0;

        let tasks = inx_im(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_set_processor_status_of_x_register_after_increment() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_x = 0xFF;

        let tasks = inx_im(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}

#[cfg(test)]
mod iny_im {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::iny_im,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_increment_y_register() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_y = 0x02;

        let tasks = iny_im(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.index_register_y, 0x03);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_y = 0x02;
        cpu.cycle = 0;

        let tasks = iny_im(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_set_processor_status_of_x_register_after_increment() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_y = 0xFF;

        let tasks = iny_im(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}

#[cfg(test)]
mod inc_zp {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::inc_zp,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const VALUE: Byte = 0x02;
    const ZERO_PAGE_ADDR: Byte = 0x03;

    #[test]
    fn should_increment_value_stored_in_memory_at_zero_page_address() {
        let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;

        let tasks = inc_zp(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(memory.borrow()[ZERO_PAGE_ADDR as Word], 0x03);
    }

    #[test]
    fn should_take_four_cycles() {
        let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let tasks = inc_zp(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 4);
    }

    #[test]
    fn should_set_processor_status_of_value_in_memory() {
        const VALUE: Byte = 0xFF;
        let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;

        let tasks = inc_zp(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}

#[cfg(test)]
mod inc_zpx {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::inc_zpx,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const VALUE: Byte = 0x09;
    const ZERO_PAGE_ADDR: Byte = 0x01;
    const ZERO_PAGE_ADDR_SUM_X: Word = 0x03;

    #[test]
    fn should_increment_value_stored_in_memory_at_zero_page_address_summed_with_index_register_x() {
        let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.index_register_x = 0x02;

        let tasks = inc_zpx(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(memory.borrow()[ZERO_PAGE_ADDR_SUM_X as Word], 0x0A);
    }

    #[test]
    fn should_take_five_cycles() {
        let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.index_register_x = 0x02;
        cpu.cycle = 0;

        let tasks = inc_zpx(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 5);
    }

    #[test]
    fn should_set_processor_status_of_value_in_memory() {
        const VALUE: Byte = 0xFF;
        let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.index_register_x = 0x02;

        let tasks = inc_zpx(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}

#[cfg(test)]
mod inc_a {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::inc_a,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const VALUE: Byte = 0x09;
    const ADDR_LO: Byte = 0x04;
    const ADDR_HI: Byte = 0x00;
    const ADDR: Word = 0x0004;

    #[test]
    fn should_increment_value_stored_in_memory_at_absolute_address() {
        let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;

        let tasks = inc_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(memory.borrow()[ADDR as Word], 0x0A);
    }

    #[test]
    fn should_take_five_cycles() {
        let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let tasks = inc_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 5);
    }

    #[test]
    fn should_set_processor_status_of_value_in_memory() {
        const VALUE: Byte = 0xFF;
        let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;

        let tasks = inc_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}

#[cfg(test)]
mod inc_ax {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::inc_ax,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const VALUE: Byte = 0x09;
    const ADDR_LO: Byte = 0x02;
    const ADDR_HI: Byte = 0x00;
    const OFFSET: Byte = 0x02;
    const ADDR_OFFSET_BY_X: Word = 0x0004;

    #[test]
    fn should_increment_value_stored_in_memory_at_absolute_address_offset_by_index_register_x() {
        let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.index_register_x = OFFSET;

        let tasks = inc_ax(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(memory.borrow()[ADDR_OFFSET_BY_X], 0x0A);
    }

    #[test]
    fn should_take_six_cycles() {
        let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.index_register_x = OFFSET;
        cpu.cycle = 0;

        let tasks = inc_ax(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 6);
    }

    #[test]
    fn should_set_processor_status_of_value_in_memory() {
        const VALUE: Byte = 0xFF;
        let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.index_register_x = OFFSET;

        let tasks = inc_ax(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}

#[cfg(test)]
mod dex_im {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::dex_im,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_decrement_x_register() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_x = 0x02;

        let tasks = dex_im(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.index_register_x, 0x01);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_x = 0x02;
        cpu.cycle = 0;

        let tasks = dex_im(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_set_processor_status_of_x_register_after_decrement() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_x = 0x01;

        let tasks = dex_im(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}

#[cfg(test)]
mod dey_im {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::dey_im,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_decrement_y_register() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_y = 0x02;

        let tasks = dey_im(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.index_register_y, 0x01);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_y = 0x02;
        cpu.cycle = 0;

        let tasks = dey_im(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_set_processor_status_of_y_register_after_decrement() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_y = 0x01;

        let tasks = dey_im(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}

#[cfg(test)]
mod dec_zp {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::dec_zp,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const VALUE: Byte = 0x02;
    const ZERO_PAGE_ADDR: Byte = 0x03;

    #[test]
    fn should_decrement_value_stored_in_memory_at_zero_page_address() {
        let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;

        let tasks = dec_zp(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(memory.borrow()[ZERO_PAGE_ADDR as Word], 0x01);
    }

    #[test]
    fn should_take_four_cycles() {
        let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let tasks = dec_zp(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 4);
    }

    #[test]
    fn should_set_processor_status_of_value_in_memory() {
        const VALUE: Byte = 0x01;
        let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;

        let tasks = dec_zp(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}

#[cfg(test)]
mod dec_zpx {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::dec_zpx,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const VALUE: Byte = 0x09;
    const ZERO_PAGE_ADDR: Byte = 0x01;
    const ZERO_PAGE_ADDR_SUM_X: Word = 0x03;

    #[test]
    fn should_decrement_value_stored_in_memory_at_zero_page_address_summed_with_index_register_x() {
        let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.index_register_x = 0x02;

        let tasks = dec_zpx(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(memory.borrow()[ZERO_PAGE_ADDR_SUM_X as Word], 0x08);
    }

    #[test]
    fn should_take_five_cycles() {
        let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.index_register_x = 0x02;
        cpu.cycle = 0;

        let tasks = dec_zpx(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 5);
    }

    #[test]
    fn should_set_processor_status_of_value_in_memory() {
        const VALUE: Byte = 0x01;
        let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.index_register_x = 0x02;

        let tasks = dec_zpx(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}

#[cfg(test)]
mod dec_a {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::dec_a,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const VALUE: Byte = 0x09;
    const ADDR_LO: Byte = 0x04;
    const ADDR_HI: Byte = 0x00;
    const ADDR: Word = 0x0004;

    #[test]
    fn should_decrement_value_stored_in_memory_at_absolute_address() {
        let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;

        let tasks = dec_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(memory.borrow()[ADDR as Word], 0x08);
    }

    #[test]
    fn should_take_five_cycles() {
        let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let tasks = dec_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 5);
    }

    #[test]
    fn should_set_processor_status_of_value_in_memory() {
        const VALUE: Byte = 0x01;
        let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;

        let tasks = dec_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}

#[cfg(test)]
mod dec_ax {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::dec_ax,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const VALUE: Byte = 0x09;
    const ADDR_LO: Byte = 0x02;
    const ADDR_HI: Byte = 0x00;
    const OFFSET: Byte = 0x02;
    const ADDR_OFFSET_BY_X: Word = 0x0004;

    #[test]
    fn should_decrement_value_stored_in_memory_at_absolute_address_offset_by_index_register_x() {
        let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.index_register_x = OFFSET;

        let tasks = dec_ax(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(memory.borrow()[ADDR_OFFSET_BY_X], 0x08);
    }

    #[test]
    fn should_take_six_cycles() {
        let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.index_register_x = OFFSET;
        cpu.cycle = 0;

        let tasks = dec_ax(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 6);
    }

    #[test]
    fn should_set_processor_status_of_value_in_memory() {
        const VALUE: Byte = 0x01;
        let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.index_register_x = OFFSET;

        let tasks = dec_ax(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.processor_status, 0b00000010);
    }
}
