use std::cell::RefCell;
use std::collections::HashMap;

use ringbuffer::{AllocRingBuffer, RingBuffer};

use addressing::{get_addressing_tasks, AddressingMode};
use tasks::read_memory::{AddressingReadMemoryTasks, ImmediateReadMemoryTasks, ReadMemoryTasks};
use tasks::Tasks;

use super::consts::{Byte, Word};
use crate::consts::RESET_VECTOR;
use crate::{consts::STACK_PAGE_HI, memory::Memory};

mod addressing;
mod instructions;
mod opcodes;
mod processor_status;
mod tasks;

type Instruction = Byte;

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

struct InstructionExecution {
    opcode: Byte,
    starting_cycle: usize,
    tasks: Box<dyn Tasks>,
}

pub struct CPU<'a> {
    chip_variant: ChipVariant,
    current_instruction: Option<InstructionExecution>,
    instruction_history: AllocRingBuffer<InstructionExecution>,
    cycle: usize,
    program_counter: Word,
    stack_pointer: Byte,
    accumulator: Byte,
    index_register_x: Byte,
    index_register_y: Byte,
    processor_status: processor_status::ProcessorStatus,
    memory: &'a RefCell<dyn Memory>,
    opcode_handlers: HashMap<Byte, OpcodeHandler>,
    sync: bool,
}

const INSTRUCTION_HISTORY_CAPACITY: usize = 32;

impl<'a> CPU<'a> {
    fn new(memory: &'a RefCell<dyn Memory>, chip_variant: ChipVariant) -> Self {
        return CPU {
            chip_variant: chip_variant,
            current_instruction: None,
            instruction_history: AllocRingBuffer::new(INSTRUCTION_HISTORY_CAPACITY),
            cycle: 0,
            program_counter: RESET_VECTOR,
            stack_pointer: 0x00,
            accumulator: 0,
            index_register_x: 0,
            index_register_y: 0,
            processor_status: processor_status::ProcessorStatus::default(),
            memory: memory,
            opcode_handlers: instructions::get_instructions(),
            sync: false,
        };
    }

    pub fn new_nmos(memory: &'a RefCell<dyn Memory>) -> Self {
        return CPU::new(memory, ChipVariant::NMOS);
    }

    pub fn new_rockwell_cmos(memory: &'a RefCell<dyn Memory>) -> Self {
        return CPU::new(memory, ChipVariant::RockwellCMOS);
    }

    pub fn new_wdc_cmos(memory: &'a RefCell<dyn Memory>) -> Self {
        return CPU::new(memory, ChipVariant::WDCCMOS);
    }

    pub fn reset(&mut self) -> () {
        self.program_counter = self.fetch_address_from(RESET_VECTOR);
        self.cycle = 0;
        self.stack_pointer = 0x00;
        self.processor_status.change_decimal_mode_flag(false);
        self.accumulator = 0;
        self.index_register_x = 0;
        self.index_register_y = 0;
    }

    pub fn get_processor_status(&self) -> Byte {
        return self.processor_status.into();
    }

    pub fn execute_next_instruction(&mut self) {
        loop {
            self.tick();
            if self.current_instruction.is_none() {
                break;
            }
        }
    }

    pub fn execute_until_break(&mut self) -> usize {
        while !self.processor_status.get_break_flag() {
            self.execute_next_instruction();
        }

        return self.cycle;
    }

    pub fn tick(&mut self) {
        let current_instruction = std::mem::replace(&mut self.current_instruction, None);
        match current_instruction {
            Some(mut current_instruction) => {
                self.sync = false;
                let tasks_done = current_instruction.tasks.tick(self);
                self.cycle += 1;
                if tasks_done {
                    self.instruction_history.push(current_instruction);
                    return;
                }

                self.current_instruction = Some(current_instruction);
            }
            None => {
                self.sync = true;
                let opcode = self.fetch_opcode();
                self.current_instruction = Some(self.schedule_instruction(opcode));
            }
        }
    }

    pub fn sync(&mut self) -> bool {
        return self.sync;
    }

    fn access_memory(&self, addr: Word) -> Byte {
        return self.memory.borrow()[addr];
    }

    fn put_into_memory(&mut self, addr: Word, value: Byte) {
        self.memory.borrow_mut()[addr] = value;
    }

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
        return match register {
            Registers::Accumulator => self.accumulator,
            Registers::IndexX => self.index_register_x,
            Registers::IndexY => self.index_register_y,
            Registers::ProcessorStatus => self.processor_status.into(),
            Registers::StackPointer => self.stack_pointer,
        };
    }

    fn fetch_opcode(&mut self) -> Instruction {
        let opcode = self.access_memory(self.program_counter);
        self.increment_program_counter();
        self.cycle += 1;

        return opcode;
    }

    fn fetch_address_from(&mut self, addr: Word) -> Word {
        let lo = self.access_memory(addr);
        self.cycle += 1;
        let hi = self.access_memory(addr + 1);
        self.cycle += 1;

        return Word::from_le_bytes([lo, hi]);
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

    fn push_byte_to_stack(&mut self, val: Byte) {
        let stack_addr: Word = STACK_PAGE_HI | (self.stack_pointer as u16);
        self.put_into_memory(stack_addr, val);
        self.decrement_register(Registers::StackPointer);
    }

    fn pop_byte_from_stack(&mut self) -> Byte {
        self.increment_register(Registers::StackPointer);
        let stack_addr: Word = STACK_PAGE_HI | (self.stack_pointer as u16);
        let val = self.access_memory(stack_addr);

        return val;
    }

    fn read_memory(&self, addr_mode: Option<AddressingMode>) -> Box<dyn ReadMemoryTasks> {
        match addr_mode {
            Some(mode) => {
                let addressing_tasks = get_addressing_tasks(self, mode);
                if access_cycle_has_been_done_during_addressing(mode) {
                    return Box::new(
                        AddressingReadMemoryTasks::new_with_access_during_addressing(
                            addressing_tasks,
                        ),
                    );
                } else {
                    return Box::new(
                        AddressingReadMemoryTasks::new_with_access_in_separate_cycle(
                            addressing_tasks,
                        ),
                    );
                }
            }
            None => {
                return Box::new(ImmediateReadMemoryTasks::new());
            }
        }
    }

    fn get_program_counter_lo(&self) -> Byte {
        return self.program_counter.to_le_bytes()[0];
    }

    fn get_program_counter_hi(&self) -> Byte {
        return self.program_counter.to_le_bytes()[1];
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

    fn dummy_fetch(&mut self) {
        self.access_memory(self.program_counter); // fetch and discard
    }

    fn schedule_instruction(&mut self, opcode: Byte) -> InstructionExecution {
        let handler = self.opcode_handlers.get(&opcode);
        let tasks = match handler {
            Some(cb) => cb(self),
            None => panic!("illegal opcode found: {:#04x}", opcode),
        };

        return InstructionExecution {
            tasks: tasks,
            opcode: opcode,
            starting_cycle: self.cycle,
        };
    }
}

fn access_cycle_has_been_done_during_addressing(addr_mode: AddressingMode) -> bool {
    return addr_mode == AddressingMode::AbsoluteX
        || addr_mode == AddressingMode::AbsoluteY
        || addr_mode == AddressingMode::IndirectIndexY;
}

#[cfg(test)]
mod tests;
