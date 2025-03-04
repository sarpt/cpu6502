#[cfg(test)]
mod tax {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::tax,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_push_accumulator_into_index_x_register_register() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.accumulator = 0xDE;

        let tasks = tax(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.index_register_x, 0xDE);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.accumulator = 0xDE;
        cpu.cycle = 0;

        let tasks = tax(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_set_accumulator_based_on_index_x_register_value() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.accumulator = 0xDE;
        cpu.processor_status = (0x00 as u8).into();

        let tasks = tax(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.processor_status, 0b10000000);
    }
}

#[cfg(test)]
mod txa {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::txa,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_push_index_x_register_into_stack_pointer_register() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_x = 0xDE;

        let tasks = txa(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.accumulator, 0xDE);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_x = 0xDE;
        cpu.cycle = 0;

        let tasks = txa(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_set_processor_status_based_on_index_x_register_value() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_x = 0xDE;
        cpu.processor_status = (0x00 as u8).into();

        let tasks = txa(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.processor_status, 0b10000000);
    }
}

#[cfg(test)]
mod tay {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::tay,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_push_accumulator_into_index_y_register_register() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.accumulator = 0xDE;

        let tasks = tay(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.index_register_y, 0xDE);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.accumulator = 0xDE;
        cpu.cycle = 0;

        let tasks = tay(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_set_accumulator_based_on_index_y_register_value() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.accumulator = 0xDE;
        cpu.processor_status = (0x00 as u8).into();

        let tasks = tay(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.processor_status, 0b10000000);
    }
}

#[cfg(test)]
mod tya {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::tya,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_push_index_y_register_into_stack_pointer_register() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_y = 0xDE;

        let tasks = tya(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.accumulator, 0xDE);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_y = 0xDE;
        cpu.cycle = 0;

        let tasks = tya(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_set_processor_status_based_on_index_y_register_value() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_y = 0xDE;
        cpu.processor_status = (0x00 as u8).into();

        let tasks = tya(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.processor_status, 0b10000000);
    }
}
