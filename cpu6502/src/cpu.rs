use std::{cell::RefCell, collections::HashMap, rc::Rc};

use self::instructions::*;
use self::opcodes::*;
use super::consts::{Byte, Word};
use crate::consts::RESET_VECTOR;
use crate::{consts::STACK_PAGE_HI, memory::Memory};

mod instructions;
mod processor_status;
mod opcodes;

type Instruction = Byte;

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


impl CPU {
    pub fn new(memory: Rc<RefCell<dyn Memory>>) -> Self {
        let opcode_handlers: HashMap<Byte, OpcodeHandler> = HashMap::from([
            (LDA_IM, lda_im as OpcodeHandler),
            (LDA_ZP, lda_zp),
            (LDA_ZPX, lda_zpx),
            (LDA_A, lda_a),
            (LDA_AX, lda_ax),
            (LDA_AY, lda_ay),
            (LDA_INX, lda_inx),
            (LDA_INY, lda_iny),
            (LDY_IM, ldy_im),
            (LDY_ZP, ldy_zp),
            (LDY_ZPX, ldy_zpx),
            (LDY_A, ldy_a),
            (LDY_AX, ldy_ax),
            (LDX_IM, ldx_im),
            (LDX_ZP, ldx_zp),
            (LDX_ZPY, ldx_zpy),
            (LDX_A, ldx_a),
            (LDX_AY, ldx_ay),
            (JMP_A, jmp_a),
            (JMP_IN, jmp_in),
            (JSR_A, jsr_a),
            (RTS, rts),
            (BCC, bcc),
            (BCS, bcs),
            (BEQ, beq),
            (BNE, bne),
            (CMP_IM, cmp_im),
            (CMP_ZP, cmp_zp),
            (CMP_ZPX, cmp_zpx),
            (CMP_A, cmp_a),
            (CMP_AX, cmp_ax),
            (CMP_AY, cmp_ay),
            (CMP_INX, cmp_inx),
            (CMP_INY, cmp_iny),
            (CPX_IM, cpx_im),
            (CPX_ZP, cpx_zp),
            (CPX_A, cpx_a),
            (CPY_IM, cpy_im),
            (CPY_ZP, cpy_zp),
            (CPY_A, cpy_a),
            (INC_ZP, inc_zp),
            (INC_ZPX, inc_zpx),
            (INC_A, inc_a),
            (INC_AX, inc_ax),
            (INX_IM, inx_im),
            (INY_IM, iny_im),
            (DEC_ZP, dec_zp),
            (DEC_ZPX, dec_zpx),
            (DEC_A, dec_a),
            (DEC_AX, dec_ax),
            (DEX_IM, dex_im),
            (DEY_IM, dey_im),
            (STA_ZP, sta_zp),
            (STA_ZPX, sta_zpx),
            (STA_A, sta_a),
            (STA_AX, sta_ax),
            (STA_AY, sta_ay),
            (STA_INX, sta_inx),
            (STA_INY, sta_iny),
            (STX_ZP, stx_zp),
            (STX_ZPY, stx_zpy),
            (STX_A, stx_a),
            (STY_ZP, sty_zp),
            (STY_ZPX, sty_zpx),
            (STY_A, sty_a),
            (ORA_IM, ora_im),
            (ORA_ZP, ora_zp),
            (ORA_ZPX, ora_zpx),
            (ORA_A, ora_a),
            (ORA_AX, ora_ax),
            (ORA_AY, ora_ay),
            (ORA_INX, ora_inx),
            (ORA_INY, ora_iny),
            (NOP, nop),
            (CLC, clc),
            (CLD, cld),
            (CLI, cli),
            (CLV, clv),
            (SEC, sec),
            (SED, sed),
            (SEI, sei),
            (BRK, brk),
            (BIT_A, bit_a),
            (BIT_ZP, bit_zp),
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

    fn set_bit_status(&mut self, value: Byte) {
        self.processor_status.change_zero_flag(value == 0);
        self.processor_status
            .change_overflow_flag((value & 0b01000000) > 0);
        self.processor_status
            .change_negative_flag((value & 0b10000000) > 0);
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
