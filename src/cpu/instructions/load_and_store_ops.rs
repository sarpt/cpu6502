use crate::{
    cpu::{
        addressing::{get_addressing_tasks, AddressingTasks},
        tasks::read_memory::ReadMemoryTasks,
        AddressingMode, Registers, Tasks, CPU,
    },
    memory::Memory,
};

struct LoadTasks {
    done: bool,
    read_memory_tasks: Box<dyn ReadMemoryTasks>,
    register: Registers,
}

impl LoadTasks {
    pub fn new(read_memory_tasks: Box<dyn ReadMemoryTasks>, register: Registers) -> Self {
        LoadTasks {
            done: false,
            read_memory_tasks,
            register,
        }
    }
}

impl Tasks for LoadTasks {
    fn done(&self) -> bool {
        self.done
    }

    fn tick(&mut self, cpu: &mut CPU, memory: &mut dyn Memory) -> bool {
        if self.done {
            panic!("tick mustn't be called when done")
        }

        if !self.read_memory_tasks.done() && !self.read_memory_tasks.tick(cpu, memory) {
            return false;
        }

        let value = match self.read_memory_tasks.value() {
            Some(ctx) => ctx.to_le_bytes()[0],
            None => panic!("unexpected lack of value after memory read"),
        };
        cpu.set_register(self.register, value);
        self.done = true;

        self.done
    }
}

fn ld(cpu: &mut CPU, addr_mode: Option<AddressingMode>, register: Registers) -> Box<dyn Tasks> {
    let read_memory_tasks = cpu.read_memory(addr_mode);
    Box::new(LoadTasks::new(read_memory_tasks, register))
}

pub fn lda_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(cpu, None, Registers::Accumulator)
}

pub fn lda_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(cpu, Some(AddressingMode::ZeroPage), Registers::Accumulator)
}

pub fn lda_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(cpu, Some(AddressingMode::ZeroPageX), Registers::Accumulator)
}

pub fn lda_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(cpu, Some(AddressingMode::Absolute), Registers::Accumulator)
}

pub fn lda_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(cpu, Some(AddressingMode::AbsoluteX), Registers::Accumulator)
}

pub fn lda_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(cpu, Some(AddressingMode::AbsoluteY), Registers::Accumulator)
}

pub fn lda_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(
        cpu,
        Some(AddressingMode::IndexIndirectX),
        Registers::Accumulator,
    )
}

pub fn lda_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(
        cpu,
        Some(AddressingMode::IndirectIndexY),
        Registers::Accumulator,
    )
}

pub fn ldy_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(cpu, None, Registers::IndexY)
}

pub fn ldy_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(cpu, Some(AddressingMode::ZeroPage), Registers::IndexY)
}

pub fn ldy_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(cpu, Some(AddressingMode::ZeroPageX), Registers::IndexY)
}

pub fn ldy_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(cpu, Some(AddressingMode::Absolute), Registers::IndexY)
}

pub fn ldy_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(cpu, Some(AddressingMode::AbsoluteX), Registers::IndexY)
}

pub fn ldx_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(cpu, None, Registers::IndexX)
}

pub fn ldx_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(cpu, Some(AddressingMode::ZeroPage), Registers::IndexX)
}

pub fn ldx_zpy(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(cpu, Some(AddressingMode::ZeroPageY), Registers::IndexX)
}

pub fn ldx_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(cpu, Some(AddressingMode::Absolute), Registers::IndexX)
}

pub fn ldx_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    ld(cpu, Some(AddressingMode::AbsoluteY), Registers::IndexX)
}

struct StoreTasks {
    done: bool,
    addressing_tasks: Box<dyn AddressingTasks>,
    src_register: Registers,
}

impl StoreTasks {
    pub fn new(addressing_tasks: Box<dyn AddressingTasks>, src_register: Registers) -> Self {
        StoreTasks {
            done: false,
            addressing_tasks,
            src_register,
        }
    }
}

impl Tasks for StoreTasks {
    fn done(&self) -> bool {
        self.done
    }

    fn tick(&mut self, cpu: &mut CPU, memory: &mut dyn Memory) -> bool {
        if self.done {
            panic!("tick mustn't be called when done")
        }

        if !self.addressing_tasks.done() {
            self.addressing_tasks.tick(cpu, memory);
            return false;
        }

        let value = cpu.get_register(self.src_register);
        memory[self
            .addressing_tasks
            .address()
            .expect("unexpected lack of address in StoreTasks")] = value;
        self.done = true;

        self.done
    }
}

pub fn store(cpu: &mut CPU, addr_mode: AddressingMode, register: Registers) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(cpu, addr_mode);

    Box::new(StoreTasks::new(addr_tasks, register))
}

pub fn sta_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    store(cpu, AddressingMode::ZeroPage, Registers::Accumulator)
}

pub fn sta_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    store(cpu, AddressingMode::ZeroPageX, Registers::Accumulator)
}

pub fn sta_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    store(cpu, AddressingMode::Absolute, Registers::Accumulator)
}

pub fn sta_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    store(cpu, AddressingMode::AbsoluteX, Registers::Accumulator)
}

pub fn sta_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    store(cpu, AddressingMode::AbsoluteY, Registers::Accumulator)
}

pub fn sta_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
    store(cpu, AddressingMode::IndexIndirectX, Registers::Accumulator)
}

pub fn sta_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
    store(cpu, AddressingMode::IndirectIndexY, Registers::Accumulator)
}

pub fn stx_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    store(cpu, AddressingMode::ZeroPage, Registers::IndexX)
}

pub fn stx_zpy(cpu: &mut CPU) -> Box<dyn Tasks> {
    store(cpu, AddressingMode::ZeroPageY, Registers::IndexX)
}

pub fn stx_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    store(cpu, AddressingMode::Absolute, Registers::IndexX)
}

pub fn sty_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    store(cpu, AddressingMode::ZeroPage, Registers::IndexY)
}

pub fn sty_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    store(cpu, AddressingMode::ZeroPageX, Registers::IndexY)
}

pub fn sty_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    store(cpu, AddressingMode::Absolute, Registers::IndexY)
}

#[cfg(test)]
mod lda {
    #[cfg(test)]
    mod lda_im {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::lda_im,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_fetch_byte_pointed_by_program_counter_into_accumulator() {
            let mut memory = MemoryMock::default();
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            assert_eq!(cpu.accumulator, 0x0);

            let mut tasks = lda_im(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.accumulator, 0x44);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut memory = MemoryMock::default();
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x04;

            let mut tasks = lda_im(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut memory = MemoryMock::default();
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let mut tasks = lda_im(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod lda_zp {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::lda_zp,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_fetch_byte_from_a_zero_page_address_stored_in_a_place_pointed_by_program_counter_into_accumulator(
        ) {
            let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, 0x45]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            assert_eq!(cpu.accumulator, 0x0);

            let mut tasks = lda_zp(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.accumulator, 0x45);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, 0xFF]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;

            let mut tasks = lda_zp(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, 0x05]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let mut tasks = lda_zp(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod lda_zpx {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::lda_zpx,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_fetch_byte_from_an_address_stored_in_program_counter_pointed_place_summed_with_index_register_x_into_accumulator(
        ) {
            let mut memory = MemoryMock::new(&[0x01, 0x00, 0x00, 0x55]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.accumulator, 0x0);

            let mut tasks = lda_zpx(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.accumulator, 0x55);
        }

        #[test]
        fn should_overflow_over_byte_when_summing_address_from_memory_with_register_x() {
            let mut memory = MemoryMock::new(&[0xFF, 0x88, 0x00]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            let mut tasks = lda_zpx(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.accumulator, 0x88);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut memory = MemoryMock::new(&[0x01, 0x00, 0x00, 0xFF]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            let mut tasks = lda_zpx(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut memory = MemoryMock::new(&[0x01, 0x00, 0x00, 0x55]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let mut tasks = lda_zpx(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod lda_a {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::lda_a,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_fetch_byte_from_an_absolute_address_stored_in_a_place_pointed_by_program_counter_into_accumulator(
        ) {
            let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, 0x45]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            assert_eq!(cpu.accumulator, 0x0);

            let mut tasks = lda_a(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.accumulator, 0x45);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, 0xFF]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;

            let mut tasks = lda_a(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, 0x05]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let mut tasks = lda_a(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod lda_ax {
        use std::cell::RefCell;

        use crate::{
            consts::Byte,
            cpu::{
                instructions::lda_ax,
                tests::{run_tasks, MemoryMock},
                CPU,
            },
        };

        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0xDB;

        #[test]
        fn should_fetch_byte_from_an_absolute_address_offset_by_index_register_x_into_accumulator()
        {
            let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            assert_eq!(cpu.accumulator, 0x0);

            let mut tasks = lda_ax(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.accumulator, VALUE);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;

            let mut tasks = lda_ax(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            let mut tasks = lda_ax(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut memory = MemoryMock::new(&[
                ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDRESS_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            let mut tasks = lda_ax(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod lda_ay {
        use std::cell::RefCell;

        use crate::{
            consts::Byte,
            cpu::{
                instructions::lda_ay,
                tests::{run_tasks, MemoryMock},
                CPU,
            },
        };

        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0xDB;

        #[test]
        fn should_fetch_byte_from_an_absolute_address_offset_by_index_register_y_into_accumulator()
        {
            let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            assert_eq!(cpu.accumulator, 0x0);

            let mut tasks = lda_ay(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.accumulator, VALUE);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;

            let mut tasks = lda_ay(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            let mut tasks = lda_ay(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut memory = MemoryMock::new(&[
                ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDRESS_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            let mut tasks = lda_ay(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod lda_iny {
        use crate::{
            consts::Byte,
            cpu::{
                instructions::lda_iny,
                tests::{run_tasks, MemoryMock},
                CPU,
            },
        };
        use std::cell::RefCell;

        const INDIRECT_ZERO_PAGE_ADDRESS_PLACE: Byte = 0x01;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0xDB;

        #[test]
        fn should_fetch_byte_from_an_indirect_adress_stored_in_memory_at_zero_page_and_offset_with_value_from_index_register_y(
        ) {
            let mut memory = MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.accumulator, 0x0);

            let mut tasks = lda_iny(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.accumulator, VALUE);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut memory = MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            let mut tasks = lda_iny(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_four_cycles_when_summing_indirect_address_with_index_y_does_not_cross_page_flip(
        ) {
            let mut memory = MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let mut tasks = lda_iny(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 4);
        }

        #[test]
        fn should_take_five_cycles_when_summing_indirect_address_with_index_y_crosses_page_flip() {
            let mut payload: [Byte; 512] = [0x00; 512];
            payload[0x0000] = INDIRECT_ZERO_PAGE_ADDRESS_PLACE;
            payload[0x0001] = ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY;
            payload[0x0002] = ADDRESS_HI;
            payload[0x0101] = VALUE;

            let mut memory = MemoryMock::new(&payload);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let mut tasks = lda_iny(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 5);
        }
    }
}

#[cfg(test)]
mod ldx {
    #[cfg(test)]
    mod ldx_im {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::ldx_im,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_fetch_byte_pointed_by_program_counter_into_index_register_x() {
            let mut memory = MemoryMock::default();
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_x, 0x0);

            let mut tasks = ldx_im(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.index_register_x, 0x44);
        }

        #[test]
        fn should_set_load_index_register_x_processor_status() {
            let mut memory = MemoryMock::default();
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x04;

            let mut tasks = ldx_im(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut memory = MemoryMock::default();
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let mut tasks = ldx_im(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod ldx_zp {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::ldx_zp,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_fetch_byte_from_a_zero_page_address_stored_in_a_place_pointed_by_program_counter_into_index_register_x(
        ) {
            let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, 0x45]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_x, 0x0);

            let mut tasks = ldx_zp(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.index_register_x, 0x45);
        }

        #[test]
        fn should_set_load_index_register_x_processor_status() {
            let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, 0xFF]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;

            let mut tasks = ldx_zp(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, 0x05]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let mut tasks = ldx_zp(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod ldx_zpy {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::ldx_zpy,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_fetch_byte_from_an_address_stored_in_program_counter_pointed_place_summed_with_index_register_y_into_index_register_x(
        ) {
            let mut memory = MemoryMock::new(&[0x01, 0x00, 0x00, 0x55]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_x, 0x0);

            let mut tasks = ldx_zpy(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.index_register_x, 0x55);
        }

        #[test]
        fn should_overflow_over_byte_when_summing_address_from_memory_with_register_y() {
            let mut memory = MemoryMock::new(&[0xFF, 0x88, 0x00]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            let mut tasks = ldx_zpy(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.index_register_x, 0x88);
        }

        #[test]
        fn should_set_load_index_register_x_processor_status() {
            let mut memory = MemoryMock::new(&[0x01, 0x00, 0x00, 0xFF]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            let mut tasks = ldx_zpy(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut memory = MemoryMock::new(&[0x01, 0x00, 0x00, 0x55]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let mut tasks = ldx_zpy(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod ldx_a {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::ldx_a,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_fetch_byte_from_an_absolute_address_stored_in_a_place_pointed_by_program_counter_into_index_register_x(
        ) {
            let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, 0x45]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_x, 0x0);

            let mut tasks = ldx_a(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.index_register_x, 0x45);
        }

        #[test]
        fn should_set_load_index_register_x_processor_status() {
            let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, 0xFF]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;

            let mut tasks = ldx_a(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, 0x05]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let mut tasks = ldx_a(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod ldx_ay {
        use std::cell::RefCell;

        use crate::{
            consts::Byte,
            cpu::{
                instructions::ldx_ay,
                tests::{run_tasks, MemoryMock},
                CPU,
            },
        };

        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0xDB;

        #[test]
        fn should_fetch_byte_from_an_absolute_address_offset_by_index_register_y_into_index_register_x(
        ) {
            let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            assert_eq!(cpu.index_register_x, 0x0);

            let mut tasks = ldx_ay(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.index_register_x, VALUE);
        }

        #[test]
        fn should_set_load_index_register_x_processor_status() {
            let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;

            let mut tasks = ldx_ay(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            let mut tasks = ldx_ay(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut memory = MemoryMock::new(&[
                ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDRESS_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            let mut tasks = ldx_ay(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 4);
        }
    }
}

#[cfg(test)]
mod ldy {
    #[cfg(test)]
    mod ldy_im {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::ldy_im,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_fetch_byte_pointed_by_program_counter_into_index_register_y() {
            let mut memory = MemoryMock::default();
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_y, 0x0);

            let mut tasks = ldy_im(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.index_register_y, 0x44);
        }

        #[test]
        fn should_set_load_index_register_y_processor_status() {
            let mut memory = MemoryMock::default();
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x04;

            let mut tasks = ldy_im(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut memory = MemoryMock::default();
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let mut tasks = ldy_im(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod ldy_zp {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::ldy_zp,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_fetch_byte_from_a_zero_page_address_stored_in_a_place_pointed_by_program_counter_into_index_register_y(
        ) {
            let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, 0x45]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_y, 0x0);

            let mut tasks = ldy_zp(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.index_register_y, 0x45);
        }

        #[test]
        fn should_set_load_index_register_y_processor_status() {
            let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, 0xFF]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;

            let mut tasks = ldy_zp(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut memory = MemoryMock::new(&[0x03, 0xFF, 0x00, 0x05]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let mut tasks = ldy_zp(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod ldy_zpx {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::ldy_zpx,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_fetch_byte_from_an_address_stored_in_program_counter_pointed_place_summed_with_index_register_x_into_index_register_y(
        ) {
            let mut memory = MemoryMock::new(&[0x01, 0x00, 0x00, 0x55]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_y, 0x0);

            let mut tasks = ldy_zpx(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.index_register_y, 0x55);
        }

        #[test]
        fn should_overflow_over_byte_when_summing_address_from_memory_with_register_x() {
            let mut memory = MemoryMock::new(&[0xFF, 0x88, 0x00]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            let mut tasks = ldy_zpx(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.index_register_y, 0x88);
        }

        #[test]
        fn should_set_load_index_register_y_processor_status() {
            let mut memory = MemoryMock::new(&[0x01, 0x00, 0x00, 0xFF]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            let mut tasks = ldy_zpx(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut memory = MemoryMock::new(&[0x01, 0x00, 0x00, 0x55]);
            let mut cpu = CPU::new_nmos();
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let mut tasks = ldy_zpx(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod ldy_a {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::ldy_a,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_fetch_byte_from_an_absolute_address_stored_in_a_place_pointed_by_program_counter_into_index_register_y(
        ) {
            let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, 0x45]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_y, 0x0);

            let mut tasks = ldy_a(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.index_register_y, 0x45);
        }

        #[test]
        fn should_set_load_index_register_y_processor_status() {
            let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, 0xFF]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;

            let mut tasks = ldy_a(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut memory = MemoryMock::new(&[0x03, 0x00, 0x00, 0x05]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let mut tasks = ldy_a(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod ldy_ax {
        use std::cell::RefCell;

        use crate::{
            consts::Byte,
            cpu::{
                instructions::ldy_ax,
                tests::{run_tasks, MemoryMock},
                CPU,
            },
        };

        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0xDB;

        #[test]
        fn should_fetch_byte_from_an_absolute_address_offset_by_index_register_x_into_index_register_y(
        ) {
            let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            assert_eq!(cpu.index_register_y, 0x0);

            let mut tasks = ldy_ax(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.index_register_y, VALUE);
        }

        #[test]
        fn should_set_load_index_register_y_processor_status() {
            let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;

            let mut tasks = ldy_ax(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut memory = MemoryMock::new(&[ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            let mut tasks = ldy_ax(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut memory = MemoryMock::new(&[
                ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDRESS_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            let mut tasks = ldy_ax(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks, &mut memory);

            assert_eq!(cpu.cycle, 4);
        }
    }
}

#[cfg(test)]
mod sta_zp {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::sta_zp,
        tests::{run_tasks, MemoryMock},
        Byte, CPU,
    };

    const ZERO_PAGE_ADDR: Byte = 0x03;

    #[test]
    fn should_store_accumulator_in_memory_at_a_zero_page_address() {
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0x02;
        cpu.program_counter = 0x00;

        let mut tasks = sta_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(memory[ZERO_PAGE_ADDR.into()], 0x02);
    }

    #[test]
    fn should_take_two_cycles() {
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0x02;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let mut tasks = sta_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(cpu.cycle, 2);
    }
}

#[cfg(test)]
mod sta_zpx {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::sta_zpx,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const ZERO_PAGE_ADDR: Byte = 0x01;
    const ZERO_PAGE_ADDR_SUM_X: Word = 0x03;

    #[test]
    fn should_store_accumulator_in_memory_at_a_zero_page_address_summed_with_index_register_x() {
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0x05;
        cpu.index_register_x = 0x02;
        cpu.program_counter = 0x00;

        let mut tasks = sta_zpx(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(memory[ZERO_PAGE_ADDR_SUM_X], 0x05);
    }

    #[test]
    fn should_take_three_cycles() {
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0x05;
        cpu.index_register_x = 0x02;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let mut tasks = sta_zpx(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod sta_a {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::sta_a,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const ADDR_LO: Byte = 0x04;
    const ADDR_HI: Byte = 0x00;
    const ADDR: Word = 0x0004;

    #[test]
    fn should_store_accumulator_in_memory_at_an_absolute_address() {
        let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0x0A;
        cpu.program_counter = 0x00;

        let mut tasks = sta_a(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(memory[ADDR as Word], 0x0A);
    }

    #[test]
    fn should_take_three_cycles() {
        let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0x0A;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let mut tasks = sta_a(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod sta_ax {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::sta_ax,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const ADDR_LO: Byte = 0x02;
    const ADDR_HI: Byte = 0x00;
    const OFFSET: Byte = 0x02;
    const ADDR_OFFSET_BY_X: Word = 0x0004;

    #[test]
    fn should_store_accumulator_in_memory_at_an_absolute_address_offset_by_index_register_x() {
        let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0x08;
        cpu.program_counter = 0x00;
        cpu.index_register_x = OFFSET;

        let mut tasks = sta_ax(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(memory[ADDR_OFFSET_BY_X], 0x08);
    }

    #[test]
    fn should_take_four_cycles() {
        let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0x08;
        cpu.program_counter = 0x00;
        cpu.index_register_x = OFFSET;
        cpu.cycle = 0;

        let mut tasks = sta_ax(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(cpu.cycle, 4);
    }
}

#[cfg(test)]
mod sta_ay {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::sta_ay,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const ADDR_LO: Byte = 0x02;
    const ADDR_HI: Byte = 0x00;
    const OFFSET: Byte = 0x02;
    const ADDR_OFFSET_BY_Y: Word = 0x0004;

    #[test]
    fn should_store_accumulator_in_memory_at_an_absolute_address_offset_by_index_register_y() {
        let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0x08;
        cpu.program_counter = 0x00;
        cpu.index_register_y = OFFSET;

        let mut tasks = sta_ay(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(memory[ADDR_OFFSET_BY_Y], 0x08);
    }

    #[test]
    fn should_take_four_cycles() {
        let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0x08;
        cpu.program_counter = 0x00;
        cpu.index_register_y = OFFSET;
        cpu.cycle = 0;

        let mut tasks = sta_ay(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(cpu.cycle, 4);
    }
}

#[cfg(test)]
mod sta_inx {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::sta_inx,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const ZP_ADDRESS: Byte = 0x02;
    const OFFSET: Byte = 0x01;
    const EFFECTIVE_ADDRESS_LO: Byte = 0x05;
    const EFFECTIVE_ADDRESS_HI: Byte = 0x00;
    const EFFECTIVE_ADDRESS: Word = 0x0005;

    #[test]
    fn should_store_accumulator_in_an_indirect_adress_stored_in_zero_page_offset_with_index_register_x(
    ) {
        let mut memory = MemoryMock::new(&[
            ZP_ADDRESS,
            0x00,
            0x00,
            EFFECTIVE_ADDRESS_LO,
            EFFECTIVE_ADDRESS_HI,
            0x00,
            0x00,
        ]);
        let mut cpu = CPU::new_nmos();
        cpu.program_counter = 0x00;
        cpu.accumulator = 0xA9;
        cpu.index_register_x = OFFSET;

        let mut tasks = sta_inx(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(memory[EFFECTIVE_ADDRESS], 0xA9);
    }

    #[test]
    fn should_take_five_cycles() {
        let mut memory = MemoryMock::new(&[
            ZP_ADDRESS,
            0x00,
            0x00,
            EFFECTIVE_ADDRESS_LO,
            EFFECTIVE_ADDRESS_HI,
            0x00,
            0x00,
        ]);
        let mut cpu = CPU::new_nmos();
        cpu.program_counter = 0x00;
        cpu.accumulator = 0xA9;
        cpu.index_register_x = OFFSET;
        cpu.cycle = 0;

        let mut tasks = sta_inx(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(cpu.cycle, 5);
    }
}

#[cfg(test)]
mod sta_iny {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::sta_iny,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const ZP_ADDRESS: Byte = 0x01;
    const ADDRESS_LO: Byte = 0x03;
    const ADDRESS_HI: Byte = 0x00;
    const OFFSET: Byte = 0x01;
    const EFFECTIVE_ADDRESS: Word = 0x0004;

    #[test]
    fn should_store_accumulator_in_offset_with_index_register_y_indirect_adress_stored_in_zero_page(
    ) {
        let mut memory = MemoryMock::new(&[ZP_ADDRESS, ADDRESS_LO, ADDRESS_HI, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0xDF;
        cpu.index_register_y = OFFSET;
        cpu.program_counter = 0x00;

        let mut tasks = sta_iny(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(memory[EFFECTIVE_ADDRESS], 0xDF);
    }

    #[test]
    fn should_take_five_cycles_when_summing_indirect_address_with_index_y_crosses_page_flip() {
        let mut memory = MemoryMock::new(&[ZP_ADDRESS, ADDRESS_LO, ADDRESS_HI, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.accumulator = 0xDF;
        cpu.index_register_y = OFFSET;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let mut tasks = sta_iny(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(cpu.cycle, 5);
    }
}

#[cfg(test)]
mod stx_zp {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::stx_zp,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const ZERO_PAGE_ADDR: Byte = 0x03;

    #[test]
    fn should_store_index_register_x_in_memory_at_a_zero_page_address() {
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.index_register_x = 0x02;
        cpu.program_counter = 0x00;

        let mut tasks = stx_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(memory[ZERO_PAGE_ADDR as Word], 0x02);
    }

    #[test]
    fn should_take_two_cycles() {
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.index_register_x = 0x02;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let mut tasks = stx_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(cpu.cycle, 2);
    }
}

#[cfg(test)]
mod stx_zpy {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::stx_zpy,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const ZERO_PAGE_ADDR: Byte = 0x01;
    const ZERO_PAGE_ADDR_SUM_Y: Word = 0x03;

    #[test]
    fn should_store_index_register_x_in_memory_at_a_zero_page_address_summed_with_index_register_y()
    {
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.index_register_x = 0x05;
        cpu.index_register_y = 0x02;
        cpu.program_counter = 0x00;

        let mut tasks = stx_zpy(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(memory[ZERO_PAGE_ADDR_SUM_Y], 0x05);
    }

    #[test]
    fn should_take_three_cycles() {
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.index_register_x = 0x05;
        cpu.index_register_y = 0x02;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let mut tasks = stx_zpy(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod stx_a {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::stx_a,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const ADDR_LO: Byte = 0x04;
    const ADDR_HI: Byte = 0x00;
    const ADDR: Word = 0x0004;

    #[test]
    fn should_store_index_register_x_in_memory_at_an_absolute_address() {
        let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.index_register_x = 0x0A;
        cpu.program_counter = 0x00;

        let mut tasks = stx_a(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(memory[ADDR], 0x0A);
    }

    #[test]
    fn should_take_three_cycles() {
        let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.index_register_x = 0x0A;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let mut tasks = stx_a(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod sty_zp {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::sty_zp,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const ZERO_PAGE_ADDR: Byte = 0x03;

    #[test]
    fn should_store_index_register_y_in_memory_at_a_zero_page_address() {
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.index_register_y = 0x02;
        cpu.program_counter = 0x00;

        let mut tasks = sty_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(memory[ZERO_PAGE_ADDR as Word], 0x02);
    }

    #[test]
    fn should_take_two_cycles() {
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.index_register_y = 0x02;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let mut tasks = sty_zp(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(cpu.cycle, 2);
    }
}

#[cfg(test)]
mod sty_zpx {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::sty_zpx,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const ZERO_PAGE_ADDR: Byte = 0x01;
    const ZERO_PAGE_ADDR_SUM_X: Word = 0x03;

    #[test]
    fn should_store_index_register_y_in_memory_at_a_zero_page_address_summed_with_index_register_x()
    {
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.index_register_y = 0x05;
        cpu.index_register_x = 0x02;
        cpu.program_counter = 0x00;

        let mut tasks = sty_zpx(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(memory[ZERO_PAGE_ADDR_SUM_X], 0x05);
    }

    #[test]
    fn should_take_three_cycles() {
        let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.index_register_y = 0x05;
        cpu.index_register_x = 0x02;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let mut tasks = sty_zpx(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod sty_a {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::sty_a,
        tests::{run_tasks, MemoryMock},
        Byte, Word, CPU,
    };

    const ADDR_LO: Byte = 0x04;
    const ADDR_HI: Byte = 0x00;
    const ADDR: Word = 0x0004;

    #[test]
    fn should_store_index_register_y_in_memory_at_an_absolute_address() {
        let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.index_register_y = 0x0A;
        cpu.program_counter = 0x00;

        let mut tasks = sty_a(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(memory[ADDR], 0x0A);
    }

    #[test]
    fn should_take_three_cycles() {
        let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00]);
        let mut cpu = CPU::new_nmos();
        cpu.index_register_y = 0x0A;
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let mut tasks = sty_a(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert_eq!(cpu.cycle, 3);
    }
}
