use crate::cpu::{
    addressing::{get_addressing_tasks, AddressingTasks},
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
        JsrTasks {
            step: JsrSteps::Addressing,
            addressing_tasks,
        }
    }
}

impl Tasks for JsrTasks {
    fn done(&self) -> bool {
        self.step == JsrSteps::Done
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        match self.step {
            JsrSteps::Addressing => {
                let addr_done = self.addressing_tasks.tick(cpu);
                if addr_done {
                    self.step = JsrSteps::DecrementProgramCounterHi;
                }

                false
            }
            JsrSteps::DecrementProgramCounterHi => {
                let [_, ret_program_counter_hi] =
                    cpu.program_counter.wrapping_sub(1).to_le_bytes();
                cpu.push_byte_to_stack(ret_program_counter_hi);

                self.step = JsrSteps::DecrementProgramCounterLo;
                false
            }
            JsrSteps::DecrementProgramCounterLo => {
                let [ret_program_counter_lo, _] =
                    cpu.program_counter.wrapping_sub(1).to_le_bytes();
                cpu.push_byte_to_stack(ret_program_counter_lo);

                self.step = JsrSteps::SetProgramCounter;
                false
            }
            JsrSteps::SetProgramCounter => {
                cpu.program_counter = self
                    .addressing_tasks
                    .address()
                    .expect("unexpected lack of output address in SetProgramCounter step");

                self.step = JsrSteps::Done;
                true
            }
            JsrSteps::Done => {
                panic!("tick mustn't be called when done")
            }
        }
    }
}

pub fn jsr_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(cpu, AddressingMode::Absolute);
    Box::new(JsrTasks::new(addr_tasks))
}

#[derive(PartialEq, PartialOrd)]
enum RtsSteps {
    DummyFetch,
    PreDecrementStackPointer,
    PopProgramCounterLo,
    PopProgramCounterHi,
    IncrementProgramCounter,
    Done,
}

struct RtsTasks {
    step: RtsSteps,
}

impl RtsTasks {
    fn new() -> Self {
        RtsTasks {
            step: RtsSteps::DummyFetch,
        }
    }
}

impl Tasks for RtsTasks {
    fn done(&self) -> bool {
        self.step == RtsSteps::Done
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        match self.step {
            RtsSteps::DummyFetch => {
                cpu.dummy_fetch();
                self.step = RtsSteps::PreDecrementStackPointer;
                false
            }
            RtsSteps::PreDecrementStackPointer => {
                // dummy tick, simulate separate stack pointer decrement
                // second cycle involves decrement of the stack pointer but poping byte from stack in third cycle does it in a single fn call
                // TODO: dont create dummy cycles, instead of decrementing and poping values in one call separate them into respective cycles
                self.step = RtsSteps::PopProgramCounterLo;
                false
            }
            RtsSteps::PopProgramCounterLo => {
                let lo = cpu.pop_byte_from_stack();
                cpu.set_program_counter_lo(lo);
                self.step = RtsSteps::PopProgramCounterHi;
                false
            }
            RtsSteps::PopProgramCounterHi => {
                let hi = cpu.pop_byte_from_stack();
                cpu.set_program_counter_hi(hi);
                self.step = RtsSteps::IncrementProgramCounter;
                false
            }
            RtsSteps::IncrementProgramCounter => {
                cpu.increment_program_counter();
                self.step = RtsSteps::Done;
                true
            }
            RtsSteps::Done => {
                panic!("tick mustn't be called when done")
            }
        }
    }
}

pub fn rts(_cpu: &mut CPU) -> Box<dyn Tasks> {
    Box::new(RtsTasks::new())
}

pub struct JmpTasks {
    addressing_tasks: Box<dyn AddressingTasks>,
}

impl JmpTasks {
    fn new(addressing_tasks: Box<dyn AddressingTasks>) -> Self {
        JmpTasks { addressing_tasks }
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

        done
    }
}

fn jmp(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(cpu, addr_mode);

    Box::new(JmpTasks::new(addr_tasks))
}

pub fn jmp_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    jmp(cpu, AddressingMode::Absolute)
}

pub fn jmp_in(cpu: &mut CPU) -> Box<dyn Tasks> {
    jmp(cpu, AddressingMode::Indirect)
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

        let mut tasks = jsr_a(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.program_counter, 0x5144);
    }

    #[test]
    fn should_save_program_counter_shifted_once_into_stack_pointer() {
        let memory = &RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.stack_pointer = 0xFF;

        let mut tasks = jsr_a(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(memory.borrow()[0x01FF], 0x00);
        assert_eq!(memory.borrow()[0x01FE], 0x01);
    }

    #[test]
    fn should_decrement_stack_pointer_twice() {
        let memory = &RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.stack_pointer = 0xFF;

        let mut tasks = jsr_a(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.stack_pointer, 0xFD);
    }

    #[test]
    fn should_take_five_cycles() {
        let memory = &RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let mut tasks = jsr_a(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

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

        let mut tasks = rts(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

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

        let mut tasks = rts(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

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

        let mut tasks = rts(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

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

        let mut tasks = jmp_a(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.program_counter, 0x5144);
    }

    #[test]
    fn should_take_two_cycles() {
        let memory = &RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let mut tasks = jmp_a(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

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

            let mut tasks = jmp_in(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks);

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

            let mut tasks = jmp_in(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks);

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

            let mut tasks = jmp_in(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks);

            assert_eq!(cpu.cycle, 5);
        }
    }
}
