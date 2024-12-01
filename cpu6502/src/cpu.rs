use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

use super::consts::{Byte, Word};
use crate::consts::RESET_VECTOR;
use crate::{consts::STACK_PAGE_HI, memory::Memory};

mod instructions;
mod opcodes;
mod processor_status;

type Instruction = Byte;

#[derive(Copy, Clone, PartialEq)]
enum AddressingMode {
    Accumulator,
    Immediate,
    Indirect,
    Implicit,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndexIndirectX,
    IndirectIndexY,
}

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

type OpcodeHandler = fn(&mut CPU) -> ();

pub struct CPU<'a> {
    cycle: u64,
    cycle_queue: VecDeque<Box<dyn Fn(&mut CPU) -> ()>>,
    chip_variant: ChipVariant,
    program_counter: Word,
    stack_pointer: Byte,
    accumulator: Byte,
    index_register_x: Byte,
    index_register_y: Byte,
    processor_status: processor_status::ProcessorStatus,
    memory: &'a RefCell<dyn Memory>,
    opcode_handlers: HashMap<Byte, OpcodeHandler>,
}

impl<'a> CPU<'a> {
    fn new(memory: &'a RefCell<dyn Memory>, chip_variant: ChipVariant) -> Self {
        return CPU {
            cycle: 0,
            cycle_queue: VecDeque::new(),
            chip_variant: chip_variant,
            program_counter: RESET_VECTOR,
            stack_pointer: 0x00,
            accumulator: 0,
            index_register_x: 0,
            index_register_y: 0,
            processor_status: processor_status::ProcessorStatus::default(),
            memory: memory,
            opcode_handlers: instructions::get_instructions(),
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

    fn access_memory(&mut self, addr: Word) -> Byte {
        return self.memory.borrow()[addr];
    }

    fn put_into_memory(&mut self, addr: Word, value: Byte) {
        self.memory.borrow_mut()[addr] = value;
    }

    fn increment_program_counter(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(1);
        self.cycle += 1;
    }

    fn queued_increment_program_counter(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(1);
    }

    fn queued_increment_register(&mut self, register: Registers) {
        self.set_register(register, self.get_register(register).wrapping_add(1));
    }

    fn increment_register(&mut self, register: Registers) {
        self.queued_increment_register(register);
        self.cycle += 1;
    }

    fn queued_decrement_register(&mut self, register: Registers) {
        self.set_register(register, self.get_register(register).wrapping_sub(1));
    }

    fn decrement_register(&mut self, register: Registers) {
        self.queued_decrement_register(register);
        self.cycle += 1;
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

    fn offset_addr(&mut self, addr: Word, offset: Byte) -> Word {
        let [lo, mut hi] = addr.to_le_bytes();
        let (new_lo, carry) = lo.overflowing_add(offset);
        let mut address = Word::from_le_bytes([new_lo, hi]);
        self.cycle += 1;

        if !carry {
            return address;
        };

        hi = hi.wrapping_add(1);
        address = Word::from_le_bytes([new_lo, hi]);
        self.cycle += 1;

        return address;
    }

    fn fetch_instruction(&mut self) -> Instruction {
        let opcode = self.access_memory(self.program_counter);
        self.increment_program_counter();

        return opcode;
    }

    fn fetch_address(&mut self) -> Word {
        let lo = self.access_memory(self.program_counter);
        self.increment_program_counter();
        let hi = self.access_memory(self.program_counter);
        self.increment_program_counter();

        return Word::from_le_bytes([lo, hi]);
    }

    fn fetch_address_from(&mut self, addr: Word) -> Word {
        let lo = self.access_memory(addr);
        self.cycle += 1;
        let hi = self.access_memory(addr + 1);
        self.cycle += 1;

        return Word::from_le_bytes([lo, hi]);
    }

    fn fetch_zero_page_address(&mut self) -> Word {
        let address: Word = self.access_memory(self.program_counter).into();
        self.increment_program_counter();

        return address;
    }

    fn fetch_zero_page_address_lsb(&mut self) -> Byte {
        let address: Byte = self.access_memory(self.program_counter);
        self.increment_program_counter();

        return address;
    }

    fn fetch_zero_page_address_with_idx_register_offset(&mut self, register: Registers) -> Word {
        let zero_page_addr = self.fetch_zero_page_address_lsb();
        return self.sum_with_idx_register(zero_page_addr, register).into();
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

    fn sum_with_idx_register(&mut self, val: Byte, register: Registers) -> Byte {
        let register_value = match register {
            Registers::IndexX | Registers::IndexY => self.get_register(register),
            _ => panic!("cannot sum with non-idx register"),
        };

        let res = val.wrapping_add(register_value);
        self.cycle += 1;

        return res;
    }

    fn push_byte_to_stack(&mut self, val: Byte) {
        let stack_addr: Word = STACK_PAGE_HI | (self.stack_pointer as u16);
        self.put_into_memory(stack_addr, val);
        self.decrement_register(Registers::StackPointer);
    }

    fn queued_push_byte_to_stack(&mut self, val: Byte) {
        let stack_addr: Word = STACK_PAGE_HI | (self.stack_pointer as u16);
        self.put_into_memory(stack_addr, val);
        self.queued_decrement_register(Registers::StackPointer);
    }

    fn push_word_to_stack(&mut self, val: Word) {
        let [lo, hi] = val.to_le_bytes();
        self.push_byte_to_stack(hi);
        self.push_byte_to_stack(lo);
    }

    fn pop_byte_from_stack(&mut self) -> Byte {
        self.increment_register(Registers::StackPointer);
        let stack_addr: Word = STACK_PAGE_HI | (self.stack_pointer as u16);
        let val = self.access_memory(stack_addr);

        return val;
    }

    fn queued_pop_byte_from_stack(&mut self) -> Byte {
        self.queued_increment_register(Registers::StackPointer);
        let stack_addr: Word = STACK_PAGE_HI | (self.stack_pointer as u16);
        let val = self.access_memory(stack_addr);

        return val;
    }

    fn pop_word_from_stack(&mut self) -> Word {
        let lo = self.pop_byte_from_stack();
        let hi = self.pop_byte_from_stack();

        return Word::from_le_bytes([lo, hi]);
    }

    pub fn offset_program_counter(&mut self, offset: Byte) {
        let [program_counter_lo, program_counter_hi] = self.program_counter.to_le_bytes();
        let negative_offset_direction = 0b10000000 & offset > 0;
        let directionless_offset = if negative_offset_direction {
            (offset ^ 0b11111111) + 1
        } else {
            offset
        };
        let offset_program_counter_lo: Byte;
        let carry: bool;

        if negative_offset_direction {
            (offset_program_counter_lo, carry) =
                program_counter_lo.overflowing_sub(directionless_offset);
        } else {
            (offset_program_counter_lo, carry) =
                program_counter_lo.overflowing_add(directionless_offset);
        }

        self.program_counter = Word::from_le_bytes([offset_program_counter_lo, program_counter_hi]);
        self.tick();
        if !carry {
            return;
        }

        let offset_program_counter_hi: Byte;
        if negative_offset_direction {
            offset_program_counter_hi = program_counter_hi.wrapping_sub(1);
        } else {
            offset_program_counter_hi = program_counter_hi.wrapping_add(1);
        }
        self.program_counter =
            Word::from_le_bytes([offset_program_counter_lo, offset_program_counter_hi]);
        self.tick();
    }

    fn read_memory(&mut self, addr_mode: AddressingMode) -> Option<Byte> {
        let address = match self.get_address(addr_mode) {
            Some(address) => address,
            None => return None,
        };

        let value = self.access_memory(address);
        if !access_cycle_has_been_done_during_address_fixing(addr_mode) {
            self.tick();
        }

        return Some(value);
    }

    fn modify_memory(
        &mut self,
        addr_mode: AddressingMode,
        cb: &dyn Fn(&u8) -> u8,
    ) -> Option<(Byte, Byte)> {
        let address = match self.get_address(addr_mode) {
            Some(address) => address,
            None => return None,
        };

        let value = self.access_memory(address);
        // extra cycle to fix address
        self.tick();

        let modified_value = cb(&value);
        self.tick();

        self.put_into_memory(address, modified_value);
        self.tick();

        return Some((value, modified_value));
    }

    fn write_memory(&mut self, addr_mode: AddressingMode, value: Byte) -> Option<()> {
        let address = match self.get_address(addr_mode) {
            Some(address) => address,
            None => return None,
        };
        // extra cycle to fix address
        self.tick();

        self.put_into_memory(address, value);

        return Some(());
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

    fn tick(&mut self) {
        self.cycle += 1;
    }

    fn run_next_cycle(&mut self) {
        if let Some(next_cycle_runner) = self.cycle_queue.pop_front() {
            next_cycle_runner(self);
        };
        self.tick();
    }

    fn run_next_cycles(&mut self, count: usize) {
        for _ in 1..=count {
            self.run_next_cycle();
        }
    }

    fn schedule_cycle(&mut self, cb: Box<dyn Fn(&mut CPU) -> ()>) {
        self.cycle_queue.push_back(cb);
    }

    fn get_address(&mut self, addr_mode: AddressingMode) -> Option<Word> {
        match addr_mode {
            AddressingMode::ZeroPage => {
                return Some(self.fetch_zero_page_address());
            }
            AddressingMode::IndexIndirectX => {
                let address =
                    self.fetch_zero_page_address_with_idx_register_offset(Registers::IndexX);
                let effective_address = self.fetch_address_from(address);

                return Some(effective_address);
            }
            AddressingMode::IndirectIndexY => {
                let address = self.fetch_zero_page_address();
                let partial = self.fetch_address_from(address);
                let effective_address = self.offset_addr(partial, self.index_register_y);

                return Some(effective_address);
            }
            AddressingMode::ZeroPageY => {
                return Some(
                    self.fetch_zero_page_address_with_idx_register_offset(Registers::IndexY),
                );
            }
            AddressingMode::ZeroPageX => {
                return Some(
                    self.fetch_zero_page_address_with_idx_register_offset(Registers::IndexX),
                );
            }
            AddressingMode::Absolute => {
                return Some(self.fetch_address());
            }
            AddressingMode::AbsoluteX => {
                let partial = self.fetch_address();
                let effective_addr = self.offset_addr(partial, self.index_register_x);
                return Some(effective_addr);
            }
            AddressingMode::AbsoluteY => {
                let partial = self.fetch_address();
                let effective_addr = self.offset_addr(partial, self.index_register_y);
                return Some(effective_addr);
            }
            AddressingMode::Indirect => {
                let address = self.fetch_address();
                if self.chip_variant != ChipVariant::NMOS {
                    self.tick();
                    return Some(self.fetch_address_from(address));
                }

                let should_incorrectly_jump = address & 0x00FF == 0x00FF;
                if !should_incorrectly_jump {
                    return Some(self.fetch_address_from(address));
                };

                let hi = self.access_memory(address);
                let lo = self.access_memory(address & 0xFF00);
                let incorrect_jmp_address = Word::from_le_bytes([hi, lo]);

                return Some(incorrect_jmp_address);
            }
            AddressingMode::Immediate => {
                let addr = self.program_counter;
                self.program_counter += 1;
                return Some(addr);
            }
            _ => None,
        }
    }

    pub fn execute_next_instruction(&mut self) {
        let opcode = self.fetch_instruction();
        let handler = self.opcode_handlers.get(&opcode);
        match handler {
            Some(cb) => cb(self),
            None => panic!("illegal opcode found: {opcode}"),
        }
    }

    pub fn execute_until_break(&mut self) -> u64 {
        while !self.processor_status.get_break_flag() {
            self.execute_next_instruction();
        }

        return self.cycle;
    }
}

fn access_cycle_has_been_done_during_address_fixing(addr_mode: AddressingMode) -> bool {
    return addr_mode == AddressingMode::AbsoluteX
        || addr_mode == AddressingMode::AbsoluteY
        || addr_mode == AddressingMode::IndirectIndexY;
}

#[cfg(test)]
mod tests;
