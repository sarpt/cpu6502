use std::{cell::RefCell, collections::HashMap, rc::Rc};

use self::instructions::*;
use super::consts::{Byte, Word};
use crate::{consts::STACK_PAGE_HI, memory::Memory};

mod instructions;
mod processor_status;

type Instruction = Byte;

const INSTRUCTION_LDA_IM: Byte = 0xA9;
const INSTRUCTION_LDA_ZP: Byte = 0xA5;
const INSTRUCTION_LDA_ZPX: Byte = 0xB5;
const INSTRUCTION_LDA_A: Byte = 0xAD;
const INSTRUCTION_LDA_AX: Byte = 0xBD;
const INSTRUCTION_LDA_AY: Byte = 0xB9;
const INSTRUCTION_LDA_INX: Byte = 0xA1;
const INSTRUCTION_LDA_INY: Byte = 0xB1;
const INSTRUCTION_LDY_IM: Byte = 0xA0;
const INSTRUCTION_LDY_ZP: Byte = 0xA4;
const INSTRUCTION_LDY_ZPX: Byte = 0xB4;
const INSTRUCTION_LDY_A: Byte = 0xAC;
const INSTRUCTION_LDY_AX: Byte = 0xBC;
const INSTRUCTION_LDX_IM: Byte = 0xA2;
const INSTRUCTION_LDX_ZP: Byte = 0xA6;
const INSTRUCTION_LDX_ZPY: Byte = 0xB6;
const INSTRUCTION_LDX_A: Byte = 0xAE;
const INSTRUCTION_LDX_AY: Byte = 0xBE;
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
const INSTRUCTION_CMP_AX: Byte = 0xDD;
const INSTRUCTION_CMP_AY: Byte = 0xD9;
const INSTRUCTION_CMP_INX: Byte = 0xC1;
const INSTRUCTION_CMP_INY: Byte = 0xD1;
const INSTRUCTION_CPX_IM: Byte = 0xE0;
const INSTRUCTION_CPX_ZP: Byte = 0xE4;
const INSTRUCTION_CPX_A: Byte = 0xEC;
const INSTRUCTION_CPY_IM: Byte = 0xC0;
const INSTRUCTION_CPY_ZP: Byte = 0xC4;
const INSTRUCTION_CPY_A: Byte = 0xCC;
const INSTRUCTION_INC_ZP: Byte = 0xE6;
const INSTRUCTION_INC_ZPX: Byte = 0xF6;
const INSTRUCTION_INC_A: Byte = 0xEE;
const INSTRUCTION_INC_AX: Byte = 0xFE;
const INSTRUCTION_INX_IM: Byte = 0xE8;
const INSTRUCTION_INY_IM: Byte = 0xC8;
const INSTRUCTION_DEC_ZP: Byte = 0xC6;
const INSTRUCTION_DEC_ZPX: Byte = 0xD6;
const INSTRUCTION_DEC_A: Byte = 0xCE;
const INSTRUCTION_DEC_AX: Byte = 0xDE;
const INSTRUCTION_DEX_IM: Byte = 0xCA;
const INSTRUCTION_DEY_IM: Byte = 0x88;
const INSTRUCTION_STA_ZP: Byte = 0x85;
const INSTRUCTION_STA_ZPX: Byte = 0x95;
const INSTRUCTION_STA_A: Byte = 0x8D;
const INSTRUCTION_STA_AX: Byte = 0x9D;
const INSTRUCTION_STA_AY: Byte = 0x99;
const INSTRUCTION_STA_INX: Byte = 0x81;
const INSTRUCTION_STA_INY: Byte = 0x91;
const INSTRUCTION_STX_ZP: Byte = 0x86;
const INSTRUCTION_STX_ZPY: Byte = 0x96;
const INSTRUCTION_STX_A: Byte = 0x8E;
const INSTRUCTION_STY_ZP: Byte = 0x84;
const INSTRUCTION_STY_ZPX: Byte = 0x94;
const INSTRUCTION_STY_A: Byte = 0x8C;
const INSTRUCTION_ORA_IM: Byte = 0x09;
const INSTRUCTION_ORA_ZP: Byte = 0x05;
const INSTRUCTION_ORA_ZPX: Byte = 0x15;
const INSTRUCTION_ORA_A: Byte = 0x0D;
const INSTRUCTION_ORA_AX: Byte = 0x1D;
const INSTRUCTION_ORA_AY: Byte = 0x19;
const INSTRUCTION_ORA_INX: Byte = 0x01;
const INSTRUCTION_ORA_INY: Byte = 0x11;
const INSTRUCTION_NOP: Byte = 0xEA;
const INSTRUCTION_CLC: Byte = 0x18;
const INSTRUCTION_CLD: Byte = 0xD8;
const INSTRUCTION_CLI: Byte = 0x58;
const INSTRUCTION_CLV: Byte = 0xB8;
const INSTRUCTION_SEC: Byte = 0x38;
const INSTRUCTION_SED: Byte = 0xF8;
const INSTRUCTION_SEI: Byte = 0x78;
const INSTRUCTION_BRK: Byte = 0x00;

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

type OpcodeHandler = fn(&mut CPU) -> ();

pub struct CPU {
    cycle: u64,
    program_counter: Word,
    stack_pointer: Byte,
    accumulator: Byte,
    index_register_x: Byte,
    index_register_y: Byte,
    processor_status: processor_status::ProcessorStatus,
    memory: Rc<RefCell<dyn Memory>>,
    opcode_handlers: HashMap<Byte, OpcodeHandler>,
}

const BRK_INTERRUPT_VECTOR: Word = 0xFFFE;
const RESET_VECTOR: Word = 0xFFFC;

impl CPU {
    pub fn new(memory: Rc<RefCell<dyn Memory>>) -> Self {
        let opcode_handlers: HashMap<Byte, OpcodeHandler> = HashMap::from([
            (INSTRUCTION_LDA_IM, lda_im as OpcodeHandler),
            (INSTRUCTION_LDA_ZP, lda_zp),
            (INSTRUCTION_LDA_ZPX, lda_zpx),
            (INSTRUCTION_LDA_A, lda_a),
            (INSTRUCTION_LDA_AX, lda_ax),
            (INSTRUCTION_LDA_AY, lda_ay),
            (INSTRUCTION_LDA_INX, lda_inx),
            (INSTRUCTION_LDA_INY, lda_iny),
            (INSTRUCTION_LDY_IM, ldy_im),
            (INSTRUCTION_LDY_ZP, ldy_zp),
            (INSTRUCTION_LDY_ZPX, ldy_zpx),
            (INSTRUCTION_LDY_A, ldy_a),
            (INSTRUCTION_LDY_AX, ldy_ax),
            (INSTRUCTION_LDX_IM, ldx_im),
            (INSTRUCTION_LDX_ZP, ldx_zp),
            (INSTRUCTION_LDX_ZPY, ldx_zpy),
            (INSTRUCTION_LDX_A, ldx_a),
            (INSTRUCTION_LDX_AY, ldx_ay),
            (INSTRUCTION_JMP_A, jmp_a),
            (INSTRUCTION_JMP_IN, jmp_in),
            (INSTRUCTION_JSR_A, jsr_a),
            (INSTRUCTION_RTS, rts),
            (INSTRUCTION_BCC, bcc),
            (INSTRUCTION_BCS, bcs),
            (INSTRUCTION_BEQ, beq),
            (INSTRUCTION_BNE, bne),
            (INSTRUCTION_CMP_IM, cmp_im),
            (INSTRUCTION_CMP_ZP, cmp_zp),
            (INSTRUCTION_CMP_ZPX, cmp_zpx),
            (INSTRUCTION_CMP_A, cmp_a),
            (INSTRUCTION_CMP_AX, cmp_ax),
            (INSTRUCTION_CMP_AY, cmp_ay),
            (INSTRUCTION_CMP_INX, cmp_inx),
            (INSTRUCTION_CMP_INY, cmp_iny),
            (INSTRUCTION_CPX_IM, cpx_im),
            (INSTRUCTION_CPX_ZP, cpx_zp),
            (INSTRUCTION_CPX_A, cpx_a),
            (INSTRUCTION_CPY_IM, cpy_im),
            (INSTRUCTION_CPY_ZP, cpy_zp),
            (INSTRUCTION_CPY_A, cpy_a),
            (INSTRUCTION_INC_ZP, inc_zp),
            (INSTRUCTION_INC_ZPX, inc_zpx),
            (INSTRUCTION_INC_A, inc_a),
            (INSTRUCTION_INC_AX, inc_ax),
            (INSTRUCTION_INX_IM, inx_im),
            (INSTRUCTION_INY_IM, iny_im),
            (INSTRUCTION_DEC_ZP, dec_zp),
            (INSTRUCTION_DEC_ZPX, dec_zpx),
            (INSTRUCTION_DEC_A, dec_a),
            (INSTRUCTION_DEC_AX, dec_ax),
            (INSTRUCTION_DEX_IM, dex_im),
            (INSTRUCTION_DEY_IM, dey_im),
            (INSTRUCTION_STA_ZP, sta_zp),
            (INSTRUCTION_STA_ZPX, sta_zpx),
            (INSTRUCTION_STA_A, sta_a),
            (INSTRUCTION_STA_AX, sta_ax),
            (INSTRUCTION_STA_AY, sta_ay),
            (INSTRUCTION_STA_INX, sta_inx),
            (INSTRUCTION_STA_INY, sta_iny),
            (INSTRUCTION_STX_ZP, stx_zp),
            (INSTRUCTION_STX_ZPY, stx_zpy),
            (INSTRUCTION_STX_A, stx_a),
            (INSTRUCTION_STY_ZP, sty_zp),
            (INSTRUCTION_STY_ZPX, sty_zpx),
            (INSTRUCTION_STY_A, sty_a),
            (INSTRUCTION_ORA_IM, ora_im),
            (INSTRUCTION_ORA_ZP, ora_zp),
            (INSTRUCTION_ORA_ZPX, ora_zpx),
            (INSTRUCTION_ORA_A, ora_a),
            (INSTRUCTION_ORA_AX, ora_ax),
            (INSTRUCTION_ORA_AY, ora_ay),
            (INSTRUCTION_ORA_INX, ora_inx),
            (INSTRUCTION_ORA_INY, ora_iny),
            (INSTRUCTION_NOP, nop),
            (INSTRUCTION_CLC, clc),
            (INSTRUCTION_CLD, cld),
            (INSTRUCTION_CLI, cli),
            (INSTRUCTION_CLV, clv),
            (INSTRUCTION_SEC, sec),
            (INSTRUCTION_SED, sed),
            (INSTRUCTION_SEI, sei),
            (INSTRUCTION_BRK, brk),
        ]);

        return CPU {
            cycle: 0,
            program_counter: RESET_VECTOR,
            stack_pointer: 0x00,
            accumulator: 0,
            index_register_x: 0,
            index_register_y: 0,
            processor_status: processor_status::ProcessorStatus::default(),
            memory: memory,
            opcode_handlers,
        };
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
            Registers::ProcessorStatus => self.processor_status.set(value),
            Registers::StackPointer => self.stack_pointer = value,
        };
    }

    fn get_register(&self, register: Registers) -> u8 {
        return match register {
            Registers::Accumulator => self.accumulator,
            Registers::IndexX => self.index_register_x,
            Registers::IndexY => self.index_register_y,
            Registers::ProcessorStatus => self.processor_status.into(),
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

    fn fetch_zero_page_address_with_y_offset(&mut self) -> Word {
        let zero_page_addr = self.fetch_zero_page_address_lsb();
        return self.sum_with_y(zero_page_addr).into();
    }

    fn fetch_zero_page_address_with_x_offset(&mut self) -> Word {
        let zero_page_addr = self.fetch_zero_page_address_lsb();
        return self.sum_with_x(zero_page_addr).into();
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
        self.put_into_memory(stack_addr, val);
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
        let val = self.access_memory(stack_addr);

        return val;
    }

    fn pop_word_from_stack(&mut self) -> Word {
        let lo = self.pop_byte_from_stack();
        let hi = self.pop_byte_from_stack();

        return Word::from_le_bytes([lo, hi]);
    }

    pub fn offset_program_counter(&mut self, offset: u8) {
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
    ) -> Option<u8> {
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

        return Some(modified_value);
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

fn addressing_takes_extra_cycle_to_fix(addr_mode: AddressingMode) -> bool {
    return addr_mode == AddressingMode::AbsoluteX
        || addr_mode == AddressingMode::AbsoluteY
        || addr_mode == AddressingMode::IndirectIndexY;
}

#[cfg(test)]
mod tests;
