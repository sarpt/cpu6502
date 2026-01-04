use addressing::{get_addressing_tasks, AddressingMode};
use tasks::read_memory::{AddressingReadMemoryTasks, ImmediateReadMemoryTasks, ReadMemoryTasks};
use tasks::Tasks;

use super::consts::{Byte, Word};
use crate::consts::RESET_VECTOR;
use crate::cpu::instructions::INSTRUCTIONS;
use crate::{consts::STACK_PAGE_HI, memory::Memory};

mod addressing;
mod debugger;
mod instructions;
mod processor_status;
mod tasks;

#[derive(Copy, Clone, PartialEq)]
enum ChipVariant {
    NMOS,
    RockwellCMOS,
    WDCCMOS,
}

#[derive(Copy, Clone, PartialEq)]
enum Registers {
    StackPointer,
    ProcessorStatus,
    Accumulator,
    IndexX,
    IndexY,
}

type OpcodeHandler = fn(&mut CPU) -> Box<dyn Tasks>;

pub struct CPU {
    chip_variant: ChipVariant,
    current_instruction: Option<InstructionExecution>,
    cycle: usize,
    program_counter: Word,
    stack_pointer: Byte,
    accumulator: Byte,
    index_register_x: Byte,
    index_register_y: Byte,
    processor_status: processor_status::ProcessorStatus,
    sync: bool,
}

impl CPU {
    fn new(chip_variant: ChipVariant) -> Self {
        CPU {
            chip_variant,
            current_instruction: None,
            cycle: 0,
            program_counter: RESET_VECTOR,
            stack_pointer: 0x00,
            accumulator: 0,
            index_register_x: 0,
            index_register_y: 0,
            processor_status: processor_status::ProcessorStatus::default(),
            sync: false,
        }
    }

    pub fn new_nmos() -> Self {
        CPU::new(ChipVariant::NMOS)
    }

    pub fn new_rockwell_cmos() -> Self {
        CPU::new(ChipVariant::RockwellCMOS)
    }

    pub fn new_wdc_cmos() -> Self {
        CPU::new(ChipVariant::WDCCMOS)
    }

    pub fn reset(&mut self, memory: &dyn Memory) {
        self.program_counter = self.fetch_address_from(RESET_VECTOR, memory);
        self.cycle = 0;
        self.stack_pointer = 0x00;
        self.processor_status.change_decimal_mode_flag(false);
        self.accumulator = 0;
        self.index_register_x = 0;
        self.index_register_y = 0;
    }

    pub fn get_processor_status(&self) -> Byte {
        self.processor_status.into()
    }

    pub fn execute_next_instruction(&mut self, memory: &mut dyn Memory) {
        loop {
            self.tick(memory);
            if self.current_instruction.is_none() {
                break;
            }
        }
    }

    pub fn execute_until_break(&mut self, memory: &mut dyn Memory) -> usize {
        while !self.processor_status.get_break_flag() {
            self.execute_next_instruction(memory);
        }

        self.cycle
    }

    pub fn tick(&mut self, memory: &mut dyn Memory) {
        let current_instruction = self.current_instruction.take();
        match current_instruction {
            Some(mut current_instruction) => {
                self.sync = false;
                let tasks_done = current_instruction.tasks.tick(self, memory);
                self.cycle += 1;
                if tasks_done {
                    return;
                }

                self.current_instruction = Some(current_instruction);
            }
            None => {
                self.sync = true;
                self.current_instruction = Some(self.schedule_instruction(memory));
            }
        }
    }

    pub fn sync(&self) -> bool {
        self.sync
    }

    #[inline]
    fn increment_program_counter(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(1);
    }

    fn increment_register(&mut self, register: Registers) {
        self.set_register(register, self.get_register(register).wrapping_add(1));
    }

    fn decrement_register(&mut self, register: Registers) {
        self.set_register(register, self.get_register(register).wrapping_sub(1));
    }

    fn set_register(&mut self, register: Registers, value: Byte) {
        match register {
            Registers::Accumulator => self.accumulator = value,
            Registers::IndexX => self.index_register_x = value,
            Registers::IndexY => self.index_register_y = value,
            Registers::ProcessorStatus => self.processor_status.set(value),
            Registers::StackPointer => self.stack_pointer = value,
        };
        if register == Registers::ProcessorStatus || register == Registers::StackPointer {
            return;
        };

        self.set_status_of_register(register);
    }

    fn get_register(&self, register: Registers) -> Byte {
        match register {
            Registers::Accumulator => self.accumulator,
            Registers::IndexX => self.index_register_x,
            Registers::IndexY => self.index_register_y,
            Registers::ProcessorStatus => self.processor_status.into(),
            Registers::StackPointer => self.stack_pointer,
        }
    }

    fn fetch_opcode(&mut self, memory: &dyn Memory) -> (Byte, Word) {
        let addr = self.program_counter;
        let opcode = memory[addr];
        self.increment_program_counter();
        self.cycle += 1;

        (opcode, addr)
    }

    fn fetch_address_from(&mut self, addr: Word, memory: &dyn Memory) -> Word {
        let lo = memory[addr];
        self.cycle += 1;
        let hi = memory[addr + 1];
        self.cycle += 1;

        Word::from_le_bytes([lo, hi])
    }

    fn set_status_of_register(&mut self, register: Registers) {
        let target_register = self.get_register(register);

        self.processor_status.change_zero_flag(target_register == 0);
        self.processor_status
            .change_negative_flag((target_register & 0b10000000) > 1);
    }

    fn set_status_of_value(&mut self, value: Byte) {
        self.processor_status.change_zero_flag(value == 0);
        self.processor_status
            .change_negative_flag((value & 0b10000000) > 1);
    }

    fn set_cmp_status(&mut self, register: Registers, value: Byte) {
        let target_register = self.get_register(register);

        self.processor_status
            .change_carry_flag(target_register >= value);
        self.processor_status
            .change_zero_flag(target_register == value);
        self.processor_status
            .change_negative_flag(((target_register.wrapping_sub(value)) & 0b10000000) > 1);
    }

    fn set_bit_status(&mut self, value: Byte) {
        self.processor_status.change_zero_flag(value == 0);
        self.processor_status
            .change_overflow_flag((value & 0b01000000) > 0);
        self.processor_status
            .change_negative_flag((value & 0b10000000) > 0);
    }

    fn push_byte_to_stack(&mut self, val: Byte, memory: &mut dyn Memory) {
        let stack_addr: Word = STACK_PAGE_HI | (self.stack_pointer as u16);
        memory[stack_addr] = val;
        self.decrement_register(Registers::StackPointer);
    }

    fn pop_byte_from_stack(&mut self, memory: &dyn Memory) -> Byte {
        self.increment_register(Registers::StackPointer);
        let stack_addr: Word = STACK_PAGE_HI | (self.stack_pointer as u16);
        memory[stack_addr]
    }

    fn read_memory(&self, addr_mode: Option<AddressingMode>) -> Box<dyn ReadMemoryTasks> {
        match addr_mode {
            Some(mode) => {
                let addressing_tasks = get_addressing_tasks(self, mode);
                if access_cycle_has_been_done_during_addressing(mode) {
                    Box::new(
                        AddressingReadMemoryTasks::new_with_access_during_addressing(
                            addressing_tasks,
                        ),
                    )
                } else {
                    Box::new(
                        AddressingReadMemoryTasks::new_with_access_in_separate_cycle(
                            addressing_tasks,
                        ),
                    )
                }
            }
            None => Box::new(ImmediateReadMemoryTasks::new()),
        }
    }

    fn get_program_counter_lo(&self) -> Byte {
        self.program_counter.to_le_bytes()[0]
    }

    fn get_program_counter_hi(&self) -> Byte {
        self.program_counter.to_le_bytes()[1]
    }

    fn set_program_counter_lo(&mut self, lo: Byte) {
        self.program_counter = Word::from_le_bytes([lo, self.get_program_counter_hi()]);
    }

    fn set_program_counter_hi(&mut self, hi: Byte) {
        self.program_counter = Word::from_le_bytes([self.get_program_counter_lo(), hi]);
    }

    fn transfer_registers(&mut self, src: Registers, tgt: Registers) {
        let value = self.get_register(src);
        self.set_register(tgt, value);
    }

    fn dummy_fetch(&mut self, memory: &dyn Memory) {
        let _ = memory[self.program_counter]; // fetch and discard
    }

    fn schedule_instruction(&mut self, memory: &dyn Memory) -> InstructionExecution {
        let (opcode, addr) = self.fetch_opcode(memory);
        let instruction = INSTRUCTIONS.get(&opcode);
        let tasks = match instruction {
            Some(inst) => (inst.handler)(self),
            None => panic!("illegal opcode found: {:#04X}", opcode),
        };

        InstructionExecution {
            addr,
            tasks,
            opcode,
            starting_cycle: self.cycle,
        }
    }
}

fn access_cycle_has_been_done_during_addressing(addr_mode: AddressingMode) -> bool {
    addr_mode == AddressingMode::AbsoluteX
        || addr_mode == AddressingMode::AbsoluteY
        || addr_mode == AddressingMode::IndirectIndexY
}

pub struct InstructionExecution {
    pub addr: Word,
    pub opcode: Byte,
    pub starting_cycle: usize,
    tasks: Box<dyn Tasks>,
}

#[cfg(test)]
mod tests;
