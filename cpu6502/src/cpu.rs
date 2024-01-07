use std::collections::HashMap;

use self::instructions::*;
use super::consts::{Byte, Word};
use crate::{consts::STACK_PAGE_HI, memory::Memory};

mod instructions;

type Instruction = Byte;

const INSTRUCTION_LDA_IM: Byte = 0xA9;
const INSTRUCTION_LDA_ZP: Byte = 0xA5;
const INSTRUCTION_LDA_ZPX: Byte = 0xB5;
const INSTRUCTION_LDA_A: Byte = 0xAD;
const INSTRUCTION_LDA_A_X: Byte = 0xBD;
const INSTRUCTION_LDA_A_Y: Byte = 0xB9;
const INSTRUCTION_LDA_IN_X: Byte = 0xA1;
const INSTRUCTION_LDA_IN_Y: Byte = 0xB1;
const INSTRUCTION_LDY_IM: Byte = 0xA0;
const INSTRUCTION_LDY_ZP: Byte = 0xA4;
const INSTRUCTION_LDY_ZPX: Byte = 0xB4;
const INSTRUCTION_LDY_A: Byte = 0xAC;
const INSTRUCTION_LDY_A_X: Byte = 0xBC;
const INSTRUCTION_LDX_IM: Byte = 0xA2;
const INSTRUCTION_LDX_ZP: Byte = 0xA6;
const INSTRUCTION_LDX_ZPY: Byte = 0xB6;
const INSTRUCTION_LDX_A: Byte = 0xAE;
const INSTRUCTION_LDX_A_Y: Byte = 0xBE;
const INSTRUCTION_JMP_A: Byte = 0x4C;
const INSTRUCTION_JMP_IN: Byte = 0x6C;
const INSTRUCTION_JSR_A: Byte = 0x20;
const INSTRUCTION_RTS: Byte = 0x60;
const INSTRUCTION_BEQ: Byte = 0xF0;
const INSTRUCTION_BCC: Byte = 0x90;
const INSTRUCTION_BCS: Byte = 0xB0;
const INSTRUCTION_BNE: Byte = 0xD0;
const INSTRUCTION_CMP_IM: Byte = 0xC9;
const INSTRUCTION_CMP_ZP: Byte = 0xC5;
const INSTRUCTION_CMP_ZPX: Byte = 0xD5;
const INSTRUCTION_CMP_A: Byte = 0xCD;
const INSTRUCTION_CMP_A_X: Byte = 0xDD;
const INSTRUCTION_CMP_A_Y: Byte = 0xD9;
const INSTRUCTION_CMP_IN_X: Byte = 0xC1;
const INSTRUCTION_CMP_IN_Y: Byte = 0xD1;
const INSTRUCTION_CPX_IM: Byte = 0xE0;
const INSTRUCTION_CPX_ZP: Byte = 0xE4;
const INSTRUCTION_CPX_A: Byte = 0xEC;
const INSTRUCTION_CPY_IM: Byte = 0xC0;
const INSTRUCTION_CPY_ZP: Byte = 0xC4;
const INSTRUCTION_CPY_A: Byte = 0xCC;
const INSTRUCTION_INC_ZP: Byte = 0xE6;
const INSTRUCTION_INC_ZPX: Byte = 0xF6;
const INSTRUCTION_INC_A: Byte = 0xEE;
const INSTRUCTION_INC_A_X: Byte = 0xFE;
const INSTRUCTION_INX_IM: Byte = 0xE8;
const INSTRUCTION_INY_IM: Byte = 0xC8;
const INSTRUCTION_DEC_ZP: Byte = 0xC6;
const INSTRUCTION_DEC_ZPX: Byte = 0xD6;
const INSTRUCTION_DEC_A: Byte = 0xCE;
const INSTRUCTION_DEC_A_X: Byte = 0xDE;
const INSTRUCTION_DEX_IM: Byte = 0xCA;
const INSTRUCTION_DEY_IM: Byte = 0x88;
const INSTRUCTION_STA_ZP: Byte = 0x85;
const INSTRUCTION_STA_ZPX: Byte = 0x95;
const INSTRUCTION_STA_A: Byte = 0x8D;
const INSTRUCTION_STA_A_X: Byte = 0x9D;
const INSTRUCTION_STA_A_Y: Byte = 0x99;
const INSTRUCTION_STA_IN_X: Byte = 0x81;
const INSTRUCTION_STA_IN_Y: Byte = 0x91;
const INSTRUCTION_STX_ZP: Byte = 0x86;
const INSTRUCTION_STX_ZPY: Byte = 0x96;
const INSTRUCTION_STX_A: Byte = 0x8E;
const INSTRUCTION_STY_ZP: Byte = 0x84;
const INSTRUCTION_STY_ZPX: Byte = 0x94;
const INSTRUCTION_STY_A: Byte = 0x8C;

enum Flags {
    Carry = 0,
    Zero = 1,
    DecimalMode = 3,
    Negative = 7,
}

struct ProcessorStatus {
    flags: Byte,
}

#[derive(Copy, Clone, PartialEq)]
enum AddressingMode {
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

#[derive(Copy, Clone)]
enum Registers {
    StackPointer,
    ProcessorStatus,
    Accumulator,
    IndexX,
    IndexY,
}

#[derive(Copy, Clone)]
enum MemoryModifications {
    Increment,
    Decrement,
    RotateLeft,
    RotateRight,
}

#[derive(Copy, Clone, PartialEq)]
enum MemoryOperation {
    Read,
    Modify,
    Write,
}

impl ProcessorStatus {
    pub fn set_decimal_mode_flag(&mut self, value_set: bool) {
        self.set_flag(Flags::DecimalMode, value_set);
    }

    pub fn set_zero_flag(&mut self, value_set: bool) {
        self.set_flag(Flags::Zero, value_set);
    }

    pub fn set_carry_flag(&mut self, value_set: bool) {
        self.set_flag(Flags::Carry, value_set);
    }

    pub fn get_carry_flag(&self) -> bool {
        return self.get_flag(Flags::Carry);
    }

    pub fn get_zero_flag(&self) -> bool {
        return self.get_flag(Flags::Zero);
    }

    pub fn set_negative_flag(&mut self, value_set: bool) {
        self.set_flag(Flags::Negative, value_set);
    }

    pub fn get_negative_flag(&self) -> bool {
        return self.get_flag(Flags::Negative);
    }

    fn set_flag(&mut self, flag: Flags, value_set: bool) {
        let shift: u8 = flag as u8;
        if value_set {
            self.flags |= 1 << shift;
        } else {
            self.flags &= !(1 << shift);
        }
    }

    fn get_flag(&self, flag: Flags) -> bool {
        let shift: u8 = flag as u8;
        return (self.flags & (1 << shift)) > 0;
    }
}

type OpcodeHandler = fn(&mut CPU) -> ();

pub struct CPU {
    cycle: u64,
    program_counter: Word,
    stack_pointer: Byte,
    accumulator: Byte,
    index_register_x: Byte,
    index_register_y: Byte,
    processor_status: ProcessorStatus,
    memory: Box<dyn Memory>,
    opcode_handlers: HashMap<Byte, OpcodeHandler>,
}

impl CPU {
    pub fn new(memory: Box<dyn Memory>) -> Self {
        let opcode_handlers: HashMap<Byte, OpcodeHandler> = HashMap::from([
            (INSTRUCTION_LDA_IM, lda_im as OpcodeHandler),
            (INSTRUCTION_LDA_ZP, lda_zp as OpcodeHandler),
            (INSTRUCTION_LDA_ZPX, lda_zpx as OpcodeHandler),
            (INSTRUCTION_LDA_A, lda_a as OpcodeHandler),
            (INSTRUCTION_LDA_A_X, lda_a_x as OpcodeHandler),
            (INSTRUCTION_LDA_A_Y, lda_a_y as OpcodeHandler),
            (INSTRUCTION_LDA_IN_X, lda_in_x as OpcodeHandler),
            (INSTRUCTION_LDA_IN_Y, lda_in_y as OpcodeHandler),
            (INSTRUCTION_LDY_IM, ldy_im as OpcodeHandler),
            (INSTRUCTION_LDY_ZP, ldy_zp as OpcodeHandler),
            (INSTRUCTION_LDY_ZPX, ldy_zpx as OpcodeHandler),
            (INSTRUCTION_LDY_A, ldy_a as OpcodeHandler),
            (INSTRUCTION_LDY_A_X, ldy_a_x as OpcodeHandler),
            (INSTRUCTION_LDX_IM, ldx_im as OpcodeHandler),
            (INSTRUCTION_LDX_ZP, ldx_zp as OpcodeHandler),
            (INSTRUCTION_LDX_ZPY, ldx_zpy as OpcodeHandler),
            (INSTRUCTION_LDX_A, ldx_a as OpcodeHandler),
            (INSTRUCTION_LDX_A_Y, ldx_a_y as OpcodeHandler),
            (INSTRUCTION_JMP_A, jmp_a as OpcodeHandler),
            (INSTRUCTION_JMP_IN, jmp_in as OpcodeHandler),
            (INSTRUCTION_JSR_A, jsr_a as OpcodeHandler),
            (INSTRUCTION_RTS, rts as OpcodeHandler),
            (INSTRUCTION_BCC, bcc as OpcodeHandler),
            (INSTRUCTION_BCS, bcs as OpcodeHandler),
            (INSTRUCTION_BEQ, beq as OpcodeHandler),
            (INSTRUCTION_BNE, bne as OpcodeHandler),
            (INSTRUCTION_CMP_IM, cmp_im as OpcodeHandler),
            (INSTRUCTION_CMP_ZP, cmp_zp as OpcodeHandler),
            (INSTRUCTION_CMP_ZPX, cmp_zpx as OpcodeHandler),
            (INSTRUCTION_CMP_A, cmp_a as OpcodeHandler),
            (INSTRUCTION_CMP_A_X, cmp_a_x as OpcodeHandler),
            (INSTRUCTION_CMP_A_Y, cmp_a_y as OpcodeHandler),
            (INSTRUCTION_CMP_IN_X, cmp_in_x as OpcodeHandler),
            (INSTRUCTION_CMP_IN_Y, cmp_in_y as OpcodeHandler),
            (INSTRUCTION_CPX_IM, cpx_im as OpcodeHandler),
            (INSTRUCTION_CPX_ZP, cpx_zp as OpcodeHandler),
            (INSTRUCTION_CPX_A, cpx_a as OpcodeHandler),
            (INSTRUCTION_CPY_IM, cpy_im as OpcodeHandler),
            (INSTRUCTION_CPY_ZP, cpy_zp as OpcodeHandler),
            (INSTRUCTION_CPY_A, cpy_a as OpcodeHandler),
            (INSTRUCTION_INC_ZP, inc_zp as OpcodeHandler),
            (INSTRUCTION_INC_ZPX, inc_zpx as OpcodeHandler),
            (INSTRUCTION_INC_A, inc_a as OpcodeHandler),
            (INSTRUCTION_INC_A_X, inc_a_x as OpcodeHandler),
            (INSTRUCTION_INX_IM, inx_im as OpcodeHandler),
            (INSTRUCTION_INY_IM, iny_im as OpcodeHandler),
            (INSTRUCTION_DEC_ZP, dec_zp as OpcodeHandler),
            (INSTRUCTION_DEC_ZPX, dec_zpx as OpcodeHandler),
            (INSTRUCTION_DEC_A, dec_a as OpcodeHandler),
            (INSTRUCTION_DEC_A_X, dec_a_x as OpcodeHandler),
            (INSTRUCTION_DEX_IM, dex_im as OpcodeHandler),
            (INSTRUCTION_DEY_IM, dey_im as OpcodeHandler),
            (INSTRUCTION_STA_ZP, sta_zp as OpcodeHandler),
            (INSTRUCTION_STA_ZPX, sta_zpx as OpcodeHandler),
            (INSTRUCTION_STA_A, sta_a as OpcodeHandler),
            (INSTRUCTION_STA_A_X, sta_a_x as OpcodeHandler),
            (INSTRUCTION_STA_A_Y, sta_a_y as OpcodeHandler),
            (INSTRUCTION_STA_IN_X, sta_in_x as OpcodeHandler),
            (INSTRUCTION_STA_IN_Y, sta_in_y as OpcodeHandler),
            (INSTRUCTION_STX_ZP, stx_zp as OpcodeHandler),
            (INSTRUCTION_STX_ZPY, stx_zpy as OpcodeHandler),
            (INSTRUCTION_STX_A, stx_a as OpcodeHandler),
            (INSTRUCTION_STY_ZP, sty_zp as OpcodeHandler),
            (INSTRUCTION_STY_ZPX, sty_zpx as OpcodeHandler),
            (INSTRUCTION_STY_A, sty_a as OpcodeHandler),
        ]);

        return CPU {
            cycle: 0,
            program_counter: 0xFFFC,
            stack_pointer: 0,
            accumulator: 0,
            index_register_x: 0,
            index_register_y: 0,
            processor_status: ProcessorStatus { flags: 0 },
            memory: memory,
            opcode_handlers,
        };
    }

    pub fn reset(&mut self) -> () {
        self.cycle = 0;
        self.program_counter = 0xFFFC;
        self.stack_pointer = 0x00;
        self.processor_status.set_decimal_mode_flag(false);
        self.accumulator = 0;
        self.index_register_x = 0;
        self.index_register_y = 0;
    }

    fn access_memory(&mut self, addr: Word) -> Byte {
        return self.memory[addr];
    }

    fn put_into_memory(&mut self, addr: Word, value: Byte) {
        self.memory[addr] = value;
    }

    fn increment_program_counter(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(1);
        self.cycle += 1;
    }

    fn increment_register(&mut self, register: Registers) {
        self.set_register(register, self.get_register(register).wrapping_add(1));
        self.cycle += 1;
    }

    fn decrement_register(&mut self, register: Registers) {
        self.set_register(register, self.get_register(register).wrapping_sub(1));
        self.cycle += 1;
    }

    fn set_register(&mut self, register: Registers, value: Byte) {
        match register {
            Registers::Accumulator => self.accumulator = value,
            Registers::IndexX => self.index_register_x = value,
            Registers::IndexY => self.index_register_y = value,
            Registers::ProcessorStatus => self.processor_status.flags = value,
            Registers::StackPointer => self.stack_pointer = value,
        };
    }

    fn get_register(&self, register: Registers) -> u8 {
        return match register {
            Registers::Accumulator => self.accumulator,
            Registers::IndexX => self.index_register_x,
            Registers::IndexY => self.index_register_y,
            Registers::ProcessorStatus => self.processor_status.flags,
            Registers::StackPointer => self.stack_pointer,
        };
    }

    fn offset_addr(&mut self, addr: Word, offset: Byte, operation: MemoryOperation) -> Word {
        let [lo, mut hi] = addr.to_le_bytes();
        let (new_lo, carry) = lo.overflowing_add(offset);
        let mut address = Word::from_le_bytes([new_lo, hi]);
        self.cycle += 1;

        if !carry {
            if operation != MemoryOperation::Read {
                self.cycle += 1
            };
            return address;
        };

        hi = hi.wrapping_add(1);
        address = ((hi as u16) << 8) | new_lo as u16;
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

    fn fetch_zero_page_address_with_y_offset(&mut self) -> Word {
        let zero_page_addr = self.fetch_zero_page_address_lsb();
        return self.sum_with_y(zero_page_addr).into();
    }

    fn fetch_zero_page_address_with_x_offset(&mut self) -> Word {
        let zero_page_addr = self.fetch_zero_page_address_lsb();
        return self.sum_with_x(zero_page_addr).into();
    }

    fn set_load_status(&mut self, register: Registers) {
        let target_register = self.get_register(register);

        self.processor_status.set_zero_flag(target_register == 0);
        self.processor_status
            .set_negative_flag((target_register & 0b10000000) > 1);
    }

    fn set_cmp_status(&mut self, register: Registers, value: Byte) {
        let target_register = self.get_register(register);

        self.processor_status
            .set_carry_flag(target_register >= value);
        self.processor_status
            .set_zero_flag(target_register == value);
        self.processor_status
            .set_negative_flag(((target_register.wrapping_sub(value)) & 0b10000000) > 1);
    }

    fn sum_with_x(&mut self, val: Byte) -> Byte {
        let reg_x = self.index_register_x;
        let res = val.wrapping_add(reg_x);
        self.cycle += 1;

        return res;
    }

    fn sum_with_y(&mut self, val: Byte) -> Byte {
        let reg_y = self.index_register_y;
        let res = val.wrapping_add(reg_y);
        self.cycle += 1;

        return res;
    }

    fn push_byte_to_stack(&mut self, val: Byte) {
        let stack_addr: Word = STACK_PAGE_HI | (self.stack_pointer as u16);
        self.memory[stack_addr] = val;
        self.decrement_register(Registers::StackPointer);
    }

    fn push_word_to_stack(&mut self, val: Word) {
        let [lo, hi] = val.to_le_bytes();
        self.push_byte_to_stack(lo);
        self.push_byte_to_stack(hi);
    }

    fn pop_byte_from_stack(&mut self) -> Byte {
        self.increment_register(Registers::StackPointer);
        let stack_addr: Word = STACK_PAGE_HI | (self.stack_pointer as u16);
        let val = self.memory[stack_addr];

        return val;
    }

    fn pop_word_from_stack(&mut self) -> Word {
        let lo = self.pop_byte_from_stack();
        let hi = self.pop_byte_from_stack();

        return Word::from_le_bytes([lo, hi]);
    }

    pub fn set_memory(&mut self, memory: Box<dyn Memory>) {
        self.memory = memory;
    }

    pub fn offset_program_counter(&mut self, offset: u8) {
        let [program_counter_lo, program_counter_hi] = self.program_counter.to_le_bytes();
        let negative_offset_direction = 0b10000000 & offset > 0;
        let offset = 0b01111111 & offset;
        let offset_program_counter_lo: Byte;
        let carry: bool;

        if negative_offset_direction {
            (offset_program_counter_lo, carry) = program_counter_lo.overflowing_sub(offset);
        } else {
            (offset_program_counter_lo, carry) = program_counter_lo.overflowing_add(offset);
        }

        self.program_counter = Word::from_le_bytes([offset_program_counter_lo, program_counter_hi]);
        self.cycle += 1;
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
        self.cycle += 1;
    }

    fn read_memory(&mut self, addr_mode: AddressingMode) -> Option<Byte> {
        let address = match self.get_address(addr_mode, MemoryOperation::Read) {
            Some(address) => address,
            None => return None,
        };

        let value = self.access_memory(address);
        if !addressing_takes_extra_cycle_to_fix(addr_mode) {
            self.cycle += 1;
        }

        return Some(value);
    }

    fn modify_memory(
        &mut self,
        addr_mode: AddressingMode,
        modification: MemoryModifications,
    ) -> Option<()> {
        let address = match self.get_address(addr_mode, MemoryOperation::Modify) {
            Some(address) => address,
            None => return None,
        };

        let value = self.access_memory(address);
        if !addressing_takes_extra_cycle_to_fix(addr_mode) {
            self.cycle += 1;
        }

        let modified_value = match modification {
            MemoryModifications::Increment => value.wrapping_add(1),
            MemoryModifications::Decrement => value.wrapping_sub(1),
            MemoryModifications::RotateLeft => panic!("rotate left not implemented yet"),
            MemoryModifications::RotateRight => panic!("rotate right not implemented yet"),
        };
        self.cycle += 1;

        self.put_into_memory(address, modified_value);
        self.cycle += 1;

        return Some(());
    }

    fn write_memory(&mut self, addr_mode: AddressingMode, value: Byte) -> Option<()> {
        let address = match self.get_address(addr_mode, MemoryOperation::Write) {
            Some(address) => address,
            None => return None,
        };

        self.put_into_memory(address, value);
        if !addressing_takes_extra_cycle_to_fix(addr_mode) {
            self.cycle += 1;
        }

        return Some(());
    }

    fn get_address(
        &mut self,
        addr_mode: AddressingMode,
        operation: MemoryOperation,
    ) -> Option<Word> {
        match addr_mode {
            AddressingMode::ZeroPage => {
                return Some(self.fetch_zero_page_address());
            }
            AddressingMode::IndexIndirectX => {
                let address = self.fetch_zero_page_address_with_x_offset();
                let effective_address = self.fetch_address_from(address);

                return Some(effective_address);
            }
            AddressingMode::IndirectIndexY => {
                let address = self.fetch_zero_page_address();
                let partial = self.fetch_address_from(address);
                let effective_address = self.offset_addr(partial, self.index_register_y, operation);

                return Some(effective_address);
            }
            AddressingMode::ZeroPageY => {
                return Some(self.fetch_zero_page_address_with_y_offset());
            }
            AddressingMode::ZeroPageX => {
                return Some(self.fetch_zero_page_address_with_x_offset());
            }
            AddressingMode::Absolute => {
                return Some(self.fetch_address());
            }
            AddressingMode::AbsoluteX => {
                let partial = self.fetch_address();
                let effective_addr = self.offset_addr(partial, self.index_register_x, operation);
                return Some(effective_addr);
            }
            AddressingMode::AbsoluteY => {
                let partial = self.fetch_address();
                let effective_addr = self.offset_addr(partial, self.index_register_y, operation);
                return Some(effective_addr);
            }
            AddressingMode::Indirect => {
                let address = self.fetch_address();
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
                return Some(self.program_counter);
            }
            _ => None,
        }
    }

    pub fn execute(&mut self, cycles: u64) -> u64 {
        let cycles_before_execution = self.cycle;
        let stop_cycle = cycles_before_execution + cycles;

        while self.cycle < stop_cycle {
            let opcode = self.fetch_instruction();
            let handler = self.opcode_handlers.get(&opcode);
            match handler {
                Some(cb) => cb(self),
                None => panic!("illegal opcode found: {opcode}"),
            }
        }

        return stop_cycle;
    }
}

fn addressing_takes_extra_cycle_to_fix(addr_mode: AddressingMode) -> bool {
    return addr_mode == AddressingMode::AbsoluteX
        || addr_mode == AddressingMode::AbsoluteY
        || addr_mode == AddressingMode::IndirectIndexY;
}

#[cfg(test)]
mod tests;
