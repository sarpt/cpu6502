#[cfg(test)]
mod clc {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::clc,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_clear_carry_flag() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_carry_flag(true);

        let mut tasks = clc(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.processor_status.get_carry_flag(), false);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_carry_flag(true);
        cpu.cycle = 0;

        let mut tasks = clc(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.cycle, 1);
    }
}

#[cfg(test)]
mod cld {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::cld,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_clear_decimal_flag() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_decimal_mode_flag(true);

        let mut tasks = cld(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.processor_status.get_decimal_mode_flag(), false);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_decimal_mode_flag(true);
        cpu.cycle = 0;

        let mut tasks = cld(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.cycle, 1);
    }
}

#[cfg(test)]
mod cli {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::cli,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_clear_interrupt_disable_flag() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_interrupt_disable_flag(true);

        let mut tasks = cli(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.processor_status.get_interrupt_disable_flag(), false);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_interrupt_disable_flag(true);
        cpu.cycle = 0;

        let mut tasks = cli(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.cycle, 1);
    }
}

#[cfg(test)]
mod clv {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::clv,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_clear_overflow_flag() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_overflow_flag(true);

        let mut tasks = clv(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.processor_status.get_overflow_flag(), false);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_overflow_flag(true);
        cpu.cycle = 0;

        let mut tasks = clv(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.cycle, 1);
    }
}

#[cfg(test)]
mod sec {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::sec,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_set_carry_flag() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_carry_flag(false);

        let mut tasks = sec(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.processor_status.get_carry_flag(), true);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_carry_flag(false);
        cpu.cycle = 0;

        let mut tasks = sec(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.cycle, 1);
    }
}

#[cfg(test)]
mod sed {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::sed,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_set_decimal_mode_flag() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_decimal_mode_flag(false);

        let mut tasks = sed(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.processor_status.get_decimal_mode_flag(), true);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_decimal_mode_flag(false);
        cpu.cycle = 0;

        let mut tasks = sed(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.cycle, 1);
    }
}

#[cfg(test)]
mod sei {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::sei,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_set_interrupt_disable_flag() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_interrupt_disable_flag(false);

        let mut tasks = sei(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.processor_status.get_interrupt_disable_flag(), true);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.change_interrupt_disable_flag(false);
        cpu.cycle = 0;

        let mut tasks = sei(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.cycle, 1);
    }
}
