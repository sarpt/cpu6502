use crate::cpu::{tasks::transfer_register::TransferRegistersTasks, Registers, Tasks, CPU};

#[derive(PartialEq, PartialOrd)]
enum PushRegisterSteps {
    DummyFetch,
    PushToStack,
    Done,
}

struct PushRegisterTasks {
    register: Registers,
    step: PushRegisterSteps,
}

impl PushRegisterTasks {
    fn new(register: Registers) -> Self {
        return PushRegisterTasks {
            register,
            step: PushRegisterSteps::DummyFetch,
        };
    }
}

impl Tasks for PushRegisterTasks {
    fn done(&self) -> bool {
        return self.step == PushRegisterSteps::Done;
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        match self.step {
            PushRegisterSteps::DummyFetch => {
                cpu.dummy_fetch();

                self.step = PushRegisterSteps::PushToStack;
                return false;
            }
            PushRegisterSteps::PushToStack => {
                let val = cpu.get_register(self.register);
                cpu.push_byte_to_stack(val);

                self.step = PushRegisterSteps::Done;
                return true;
            }
            PushRegisterSteps::Done => {
                panic!("tick mustn't be called when done")
            }
        }
    }
}

fn push_register(_cpu: &mut CPU, register: Registers) -> Box<dyn Tasks> {
    return Box::new(PushRegisterTasks::new(register));
}

pub fn pha(cpu: &mut CPU) -> Box<dyn Tasks> {
    return push_register(cpu, Registers::Accumulator);
}

pub fn php(cpu: &mut CPU) -> Box<dyn Tasks> {
    return push_register(cpu, Registers::ProcessorStatus);
}

#[derive(PartialEq, PartialOrd)]
enum PullRegisterSteps {
    DummyFetch,
    PreDecrementStackPointer,
    PullFromStack,
    Done,
}

struct PullRegisterTasks {
    register: Registers,
    step: PullRegisterSteps,
}

impl PullRegisterTasks {
    fn new(register: Registers) -> Self {
        return PullRegisterTasks {
            register,
            step: PullRegisterSteps::DummyFetch,
        };
    }
}

impl Tasks for PullRegisterTasks {
    fn done(&self) -> bool {
        return self.step == PullRegisterSteps::Done;
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        match self.step {
            PullRegisterSteps::DummyFetch => {
                cpu.dummy_fetch();

                self.step = PullRegisterSteps::PreDecrementStackPointer;
                return false;
            }
            PullRegisterSteps::PreDecrementStackPointer => {
                // dummy tick, simulate separate stack pointer decrement
                // second cycle involves decrement of the stack pointer but poping byte from stack in third cycle does it in a single fn call
                // TODO: dont create dummy cycles, instead of decrementing and poping values in one call separate them into respective cycles
                self.step = PullRegisterSteps::PullFromStack;
                return false;
            }
            PullRegisterSteps::PullFromStack => {
                let value = cpu.pop_byte_from_stack();
                cpu.set_register(self.register, value);

                self.step = PullRegisterSteps::Done;
                return true;
            }
            PullRegisterSteps::Done => {
                panic!("tick mustn't be called when done")
            }
        }
    }
}

fn pull_register(_cpu: &mut CPU, register: Registers) -> Box<dyn Tasks> {
    return Box::new(PullRegisterTasks::new(register));
}

pub fn pla(cpu: &mut CPU) -> Box<dyn Tasks> {
    return pull_register(cpu, Registers::Accumulator);
}

pub fn plp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return pull_register(cpu, Registers::ProcessorStatus);
}

pub fn tsx(_cpu: &mut CPU) -> Box<dyn Tasks> {
    return Box::new(TransferRegistersTasks::new(
        Registers::StackPointer,
        Registers::IndexX,
    ));
}

pub fn txs(_cpu: &mut CPU) -> Box<dyn Tasks> {
    return Box::new(TransferRegistersTasks::new(
        Registers::IndexX,
        Registers::StackPointer,
    ));
}

#[cfg(test)]
mod pha {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::pha,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_push_accumulator_into_stack() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xFF;
        cpu.accumulator = 0xDE;

        let mut tasks = pha(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(memory.borrow()[0x01FF], 0xDE);
    }

    #[test]
    fn should_take_two_cycles() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.accumulator = 0xDE;
        cpu.stack_pointer = 0xFF;
        cpu.cycle = 0;

        let mut tasks = pha(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.cycle, 2);
    }
}

#[cfg(test)]
mod pla {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::pla,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_pull_stack_into_accumulator() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xFE;
        memory.borrow_mut()[0x01FF] = 0xDE;
        cpu.accumulator = 0x00;

        let mut tasks = pla(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.accumulator, 0xDE);
    }

    #[test]
    fn should_take_three_cycles() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xFE;
        memory.borrow_mut()[0x01FF] = 0xDE;
        cpu.cycle = 0;

        let mut tasks = pla(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.cycle, 3);
    }

    #[test]
    fn should_set_processor_status_based_on_accumulator_value() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xFE;
        memory.borrow_mut()[0x01FF] = 0xDE;
        cpu.processor_status = (0x00 as u8).into();

        let mut tasks = pla(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.processor_status, 0b10000000);
    }
}

#[cfg(test)]
mod php {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::php,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_push_processor_status_into_stack() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status = (0b10101010 as u8).into();
        cpu.stack_pointer = 0xFF;

        let mut tasks = php(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(memory.borrow()[0x01FF], 0b10101010);
    }

    #[test]
    fn should_take_two_cycles() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status = (0b10101010 as u8).into();
        cpu.stack_pointer = 0xFF;
        cpu.cycle = 0;

        let mut tasks = php(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.cycle, 2);
    }
}

#[cfg(test)]
mod plp {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::plp,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_pull_stack_into_accumulator() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xFE;
        memory.borrow_mut()[0x01FF] = 0xDE;
        cpu.processor_status = (0x00 as u8).into();

        let mut tasks = plp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.processor_status, 0xDE);
    }

    #[test]
    fn should_take_three_cycles() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xFE;
        memory.borrow_mut()[0x01FF] = 0xDE;
        cpu.processor_status = (0x00 as u8).into();
        cpu.cycle = 0;

        let mut tasks = plp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod txs {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::txs,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_push_index_x_register_into_stack_pointer_register() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_x = 0xDE;

        let mut tasks = txs(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.stack_pointer, 0xDE);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_x = 0xDE;
        cpu.cycle = 0;

        let mut tasks = txs(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.cycle, 1);
    }
}

#[cfg(test)]
mod tsx {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::tsx,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_push_stack_pointer_into_index_x_register_register() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xDE;

        let mut tasks = tsx(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.index_register_x, 0xDE);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xDE;
        cpu.cycle = 0;

        let mut tasks = tsx(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_set_processor_status_based_on_index_x_register_value() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xDE;
        cpu.processor_status = (0x00 as u8).into();

        let mut tasks = tsx(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.processor_status, 0b10000000);
    }
}
