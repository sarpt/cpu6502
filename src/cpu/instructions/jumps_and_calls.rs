use std::rc::Rc;

use crate::cpu::{
    addressing::{get_addressing_tasks, AddressingTasks},
    tasks::GenericTasks,
    AddressingMode, Tasks, CPU,
};

#[derive(PartialEq, PartialOrd)]
enum JsrSteps {
    Addressing,
    DecrementProgramCounterLo,
    DecrementProgramCounterHi,
    SetProgramCounter,
    Done,
}

struct JsrTasks {
    step: JsrSteps,
    addressing_tasks: Box<dyn AddressingTasks>,
}

impl JsrTasks {
    pub fn new(addressing_tasks: Box<dyn AddressingTasks>) -> Self {
        return JsrTasks {
            step: JsrSteps::Addressing,
            addressing_tasks,
        };
    }
}

impl Tasks for JsrTasks {
    fn done(&self) -> bool {
        return self.step == JsrSteps::Done;
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        match self.step {
            JsrSteps::Addressing => {
                let addr_done = self.addressing_tasks.tick(cpu);
                if addr_done {
                    self.step = JsrSteps::DecrementProgramCounterHi;
                }

                return false;
            }
            JsrSteps::DecrementProgramCounterHi => {
                let [_, ret_program_counter_hi] =
                    cpu.program_counter.clone().wrapping_sub(1).to_le_bytes();
                cpu.push_byte_to_stack(ret_program_counter_hi);

                self.step = JsrSteps::DecrementProgramCounterLo;
                return false;
            }
            JsrSteps::DecrementProgramCounterLo => {
                let [ret_program_counter_lo, _] =
                    cpu.program_counter.clone().wrapping_sub(1).to_le_bytes();
                cpu.push_byte_to_stack(ret_program_counter_lo);

                self.step = JsrSteps::SetProgramCounter;
                return false;
            }
            JsrSteps::SetProgramCounter => {
                cpu.program_counter = self
                    .addressing_tasks
                    .address()
                    .expect("unexpected lack of output address in SetProgramCounter step");

                self.step = JsrSteps::Done;
                return true;
            }
            JsrSteps::Done => {
                panic!("tick mustn't be called when done")
            }
        }
    }
}

pub fn jsr_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(&cpu, AddressingMode::Absolute);
    return Box::new(JsrTasks::new(addr_tasks));
}

pub fn rts(_cpu: &mut CPU) -> Box<dyn Tasks> {
    let mut tasks = GenericTasks::new();
    tasks.push(Rc::new(|cpu| {
        cpu.dummy_fetch();
    }));

    // dummy tick, simulate separate stack pointer decrement
    // second cycle involves decrement of the stack pointer but poping byte from stack in third cycle does it in a single fn call
    // TODO: dont create dummy cycles, instead of decrementing and poping values in one call separate them into respective cycles
    tasks.push(Rc::new(|_| {}));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        let lo = cpu.pop_byte_from_stack();
        cpu.set_program_counter_lo(lo);
    }));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        let hi = cpu.pop_byte_from_stack();
        cpu.set_program_counter_hi(hi);
    }));

    tasks.push(Rc::new(|cpu| {
        cpu.increment_program_counter();
    }));

    return Box::new(tasks);
}

pub struct JmpTasks {
    addressing_tasks: Box<dyn AddressingTasks>,
}

impl JmpTasks {
    fn new(addressing_tasks: Box<dyn AddressingTasks>) -> Self {
        return JmpTasks { addressing_tasks };
    }
}

impl Tasks for JmpTasks {
    fn done(&self) -> bool {
        self.addressing_tasks.done()
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        if self.addressing_tasks.done() {
            return true;
        }

        let done = self.addressing_tasks.tick(cpu);

        if done {
            cpu.program_counter = self
                .addressing_tasks
                .address()
                .expect("unexpected lack of address in JmpTasks");
        }

        return done;
    }
}

fn jmp(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(&cpu, addr_mode);

    return Box::new(JmpTasks::new(addr_tasks));
}

pub fn jmp_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return jmp(cpu, AddressingMode::Absolute);
}

pub fn jmp_in(cpu: &mut CPU) -> Box<dyn Tasks> {
    return jmp(cpu, AddressingMode::Indirect);
}

#[cfg(test)]
mod jsr_a {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::jsr_a,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_fetch_address_pointed_by_program_counter_and_put_in_program_counter() {
        let memory = &RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.stack_pointer = 0xFF;

        let tasks = jsr_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.program_counter, 0x5144);
    }

    #[test]
    fn should_save_program_counter_shifted_once_into_stack_pointer() {
        let memory = &RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.stack_pointer = 0xFF;

        let tasks = jsr_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(memory.borrow()[0x01FF], 0x00);
        assert_eq!(memory.borrow()[0x01FE], 0x01);
    }

    #[test]
    fn should_decrement_stack_pointer_twice() {
        let memory = &RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.stack_pointer = 0xFF;

        let tasks = jsr_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.stack_pointer, 0xFD);
    }

    #[test]
    fn should_take_five_cycles() {
        let memory = &RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let tasks = jsr_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 5);
    }
}

#[cfg(test)]
mod rts {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::rts,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_fetch_address_from_stack_and_put_it_in_program_counter_incremented_by_one() {
        let memory = &RefCell::new(MemoryMock::new(&[0x01, 0x02, 0x03]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        memory.borrow_mut()[0x01FF] = 0x44;
        memory.borrow_mut()[0x01FE] = 0x51;
        cpu.stack_pointer = 0xFD;

        let tasks = rts(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.program_counter, 0x4452);
    }

    #[test]
    fn should_increment_stack_pointer_twice() {
        let memory = &RefCell::new(MemoryMock::new(&[0x01, 0x02, 0x03]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        memory.borrow_mut()[0x01FF] = 0x44;
        memory.borrow_mut()[0x01FE] = 0x51;
        cpu.stack_pointer = 0xFD;

        let tasks = rts(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.stack_pointer, 0xFF);
    }

    #[test]
    fn should_take_five_cycles() {
        let memory = &RefCell::new(MemoryMock::new(&[0x01, 0x02, 0x03]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        memory.borrow_mut()[0x01FF] = 0x44;
        memory.borrow_mut()[0x01FE] = 0x51;
        cpu.stack_pointer = 0xFD;
        cpu.cycle = 0;

        let tasks = rts(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 5);
    }
}

#[cfg(test)]
mod jmp_a {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::jmp_a,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_put_address_stored_in_memory_at_program_counter_as_a_new_program_counter() {
        let memory = &RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;

        let tasks = jmp_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.program_counter, 0x5144);
    }

    #[test]
    fn should_take_two_cycles() {
        let memory = &RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let tasks = jmp_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 2);
    }
}

#[cfg(test)]
mod jmp_in {
    #[cfg(test)]
    mod common {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::jmp_in,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_fetch_indirect_address_from_memory_and_put_in_program_counter() {
            let memory = &RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;

            let tasks = jmp_in(&mut cpu);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x0001);
        }
    }

    #[cfg(test)]
    mod nmos {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::jmp_in,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_take_four_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let tasks = jmp_in(&mut cpu);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod cmos {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::jmp_in,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_take_five_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_rockwell_cmos(memory);
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let tasks = jmp_in(&mut cpu);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 5);
        }
    }
}
