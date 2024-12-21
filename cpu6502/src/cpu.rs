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

#[derive(Copy, Clone, PartialEq)]
enum TaskCycleVariant {
    Aborted,
    Partial,
    Full,
}

type OpcodeHandler = fn(&mut CPU) -> ();
type ScheduledCycle = Box<dyn Fn(&mut CPU) -> TaskCycleVariant>;

type InstructionCtx = Option<Word>;
struct InstructionExecution {
    opcode: Byte,
    ctx: InstructionCtx,
    starting_cycle: u64,
    length: u64,
    cycle_queue: VecDeque<ScheduledCycle>,
}

pub struct CPU<'a> {
    chip_variant: ChipVariant,
    current_opcode: Option<Byte>,
    current_instruction: Option<InstructionExecution>,
    cycle: u64,
    program_counter: Word,
    stack_pointer: Byte,
    accumulator: Byte,
    index_register_x: Byte,
    index_register_y: Byte,
    processor_status: processor_status::ProcessorStatus,
    memory: &'a RefCell<dyn Memory>,
    opcode_handlers: HashMap<Byte, OpcodeHandler>,
    address_output: Word,
}

impl<'a> CPU<'a> {
    fn new(memory: &'a RefCell<dyn Memory>, chip_variant: ChipVariant) -> Self {
        return CPU {
            chip_variant: chip_variant,
            current_opcode: None,
            current_instruction: None,
            cycle: 0,
            program_counter: RESET_VECTOR,
            stack_pointer: 0x00,
            accumulator: 0,
            index_register_x: 0,
            index_register_y: 0,
            processor_status: processor_status::ProcessorStatus::default(),
            memory: memory,
            opcode_handlers: instructions::get_instructions(),
            address_output: 0,
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

    fn offset_address_output(&mut self, offset: Byte) -> Vec<ScheduledCycle> {
        let mut cycles: Vec<ScheduledCycle> = Vec::new();

        cycles.push(Box::new(move |cpu| {
            let [lo, hi] = cpu.address_output.to_le_bytes();
            let (new_lo, carry) = lo.overflowing_add(offset);
            cpu.address_output = Word::from_le_bytes([new_lo, hi]);

            if carry {
                cpu.set_ctx_hi(0x1);
            } else {
                cpu.set_ctx_hi(0x0);
            }

            return TaskCycleVariant::Full;
        }));

        cycles.push(Box::new(|cpu| {
            let carry = match cpu.get_current_instruction_ctx() {
                Some(val) => val.to_le_bytes()[1],
                None => panic!("unexpected lack of instruction ctx for offset address output"),
            };

            if carry == 0 {
                return TaskCycleVariant::Aborted;
            }

            let [lo, hi] = cpu.address_output.to_le_bytes();
            let new_hi = hi.wrapping_add(1);
            cpu.address_output = Word::from_le_bytes([lo, new_hi]);

            return TaskCycleVariant::Full;
        }));

        return cycles;
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
        self.queued_decrement_register(Registers::StackPointer);
    }

    fn pop_byte_from_stack(&mut self) -> Byte {
        self.queued_increment_register(Registers::StackPointer);
        let stack_addr: Word = STACK_PAGE_HI | (self.stack_pointer as u16);
        let val = self.access_memory(stack_addr);

        return val;
    }

    fn set_address_output<T: Into<Word>>(&mut self, val: T) {
        self.address_output = val.into();
    }

    fn set_address_output_lo(&mut self, lo: Byte) {
        self.address_output = Word::from_le_bytes([lo, self.address_output.to_le_bytes()[1]]);
    }

    fn set_address_output_hi(&mut self, hi: Byte) {
        self.address_output = Word::from_le_bytes([self.address_output.to_le_bytes()[0], hi]);
    }

    fn get_current_instruction_ctx(&mut self) -> &mut Option<Word> {
        return match &mut self.current_instruction {
            Some(current_instruciton) => &mut current_instruciton.ctx,
            None => panic!("cannot get ctx for non-exisiting instruction"),
        };
    }

    fn set_ctx(&mut self, val: Word) {
        return match &mut self.current_instruction {
            Some(current_instruciton) => current_instruciton.ctx = Some(val),
            None => panic!("cannot set ctx for non-exisiting instruction"),
        };
    }

    fn set_ctx_lo(&mut self, lo: Byte) {
        let ctx = self.get_current_instruction_ctx();
        let hi = match &ctx {
            Some(ctx) => ctx.to_le_bytes()[1],
            None => 0,
        };

        *ctx = Some(Word::from_le_bytes([lo, hi]));
    }

    fn set_ctx_hi(&mut self, hi: Byte) {
        let ctx = self.get_current_instruction_ctx();
        let lo = match &ctx {
            Some(ctx) => ctx.to_le_bytes()[0],
            None => 0,
        };

        *ctx = Some(Word::from_le_bytes([lo, hi]));
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
        let current_instruction = match &mut self.current_instruction {
            Some(current) => current,
            None => panic!("could not execute next cycle - there is no instruction scheduled"),
        };

        match current_instruction.cycle_queue.pop_front() {
            Some(next_cycle_runner) => {
                let cycle_variant = next_cycle_runner(self);
                if cycle_variant == TaskCycleVariant::Full {
                    self.tick()
                };
            }
            None => {
                panic!(
                    "could not run a queued cycle since there are no cycles queued for execution"
                )
            }
        }
    }

    fn schedule_instruction(&mut self, cycles: Vec<ScheduledCycle>) {
        let cycles_count = cycles.len();
        self.current_instruction = Some(InstructionExecution {
            ctx: None,
            cycle_queue: cycles.into(),
            opcode: self.current_opcode.unwrap_or(0),
            starting_cycle: self.cycle,
            length: 0,
        });

        // TODO: this is temporary until all instructions are implemented in queued cycles form.
        // after converting all instructions "run_next_cycle" will be called separately on demand
        for _ in 1..=cycles_count {
            self.run_next_cycle();
        }
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

    fn queued_get_address(&mut self, addr_mode: AddressingMode) -> Vec<ScheduledCycle> {
        let mut cycles: Vec<ScheduledCycle> = Vec::new();
        match addr_mode {
            AddressingMode::ZeroPage => {
                cycles.push(Box::new(|cpu| {
                    let addr: Byte = cpu.access_memory(cpu.program_counter);
                    cpu.set_address_output(addr);
                    cpu.queued_increment_program_counter();

                    return TaskCycleVariant::Full;
                }));
            }
            AddressingMode::ZeroPageY => {
                cycles.push(Box::new(|cpu| {
                    let addr: Byte = cpu.access_memory(cpu.program_counter);
                    cpu.set_address_output(addr);
                    cpu.queued_increment_program_counter();

                    return TaskCycleVariant::Full;
                }));

                cycles.push(Box::new(|cpu| {
                    let addr_output = cpu.address_output;
                    let final_address = addr_output.wrapping_add(cpu.index_register_y.into());
                    cpu.set_address_output(final_address);

                    return TaskCycleVariant::Full;
                }));
            }
            AddressingMode::ZeroPageX => {
                cycles.push(Box::new(|cpu| {
                    let addr: Byte = cpu.access_memory(cpu.program_counter);
                    cpu.set_address_output(addr);
                    cpu.queued_increment_program_counter();

                    return TaskCycleVariant::Full;
                }));

                cycles.push(Box::new(|cpu| {
                    let addr_output = cpu.address_output;
                    let final_address = addr_output.wrapping_add(cpu.index_register_x.into());
                    cpu.set_address_output(final_address);

                    return TaskCycleVariant::Full;
                }));
            }
            AddressingMode::Absolute => {
                cycles.push(Box::new(|cpu| {
                    let addr_lo = cpu.access_memory(cpu.program_counter);
                    cpu.set_address_output_lo(addr_lo);
                    cpu.queued_increment_program_counter();

                    return TaskCycleVariant::Full;
                }));

                cycles.push(Box::new(|cpu| {
                    let addr_hi = cpu.access_memory(cpu.program_counter);
                    cpu.set_address_output_hi(addr_hi);
                    cpu.queued_increment_program_counter();

                    return TaskCycleVariant::Full;
                }));
            }
            AddressingMode::AbsoluteX => {
                cycles.push(Box::new(|cpu| {
                    let addr_lo = cpu.access_memory(cpu.program_counter);
                    cpu.set_address_output_lo(addr_lo);
                    cpu.queued_increment_program_counter();

                    return TaskCycleVariant::Full;
                }));

                cycles.push(Box::new(|cpu| {
                    let addr_hi = cpu.access_memory(cpu.program_counter);
                    cpu.set_address_output_hi(addr_hi);
                    cpu.queued_increment_program_counter();

                    return TaskCycleVariant::Full;
                }));

                let mut offset_cycles = self.offset_address_output(self.index_register_x);
                cycles.append(&mut offset_cycles);
            }
            AddressingMode::AbsoluteY => {
                cycles.push(Box::new(|cpu| {
                    let addr_lo = cpu.access_memory(cpu.program_counter);
                    cpu.set_address_output_lo(addr_lo);
                    cpu.queued_increment_program_counter();

                    return TaskCycleVariant::Full;
                }));

                cycles.push(Box::new(|cpu| {
                    let addr_hi = cpu.access_memory(cpu.program_counter);
                    cpu.set_address_output_hi(addr_hi);
                    cpu.queued_increment_program_counter();

                    return TaskCycleVariant::Full;
                }));

                let mut offset_cycles = self.offset_address_output(self.index_register_y);
                cycles.append(&mut offset_cycles);
            }
            AddressingMode::Indirect => {
                cycles.push(Box::new(|cpu| {
                    let addr_lo = cpu.access_memory(cpu.program_counter);
                    cpu.set_ctx_lo(addr_lo);
                    cpu.queued_increment_program_counter();

                    return TaskCycleVariant::Full;
                }));

                cycles.push(Box::new(|cpu| {
                    let addr_hi = cpu.access_memory(cpu.program_counter);
                    cpu.set_ctx_hi(addr_hi);
                    cpu.queued_increment_program_counter();

                    return TaskCycleVariant::Full;
                }));

                if self.chip_variant != ChipVariant::NMOS {
                    cycles.push(Box::new(|_| TaskCycleVariant::Full)); // dummy tick used for fixing incorrect address

                    cycles.push(Box::new(|cpu| {
                        let addr = match cpu.get_current_instruction_ctx() {
                            Some(addr) => *addr,
                            None => panic!("could not retrieve address from ctx"),
                        };
                        let addr_lo = cpu.access_memory(addr);
                        cpu.set_address_output_lo(addr_lo);

                        return TaskCycleVariant::Full;
                    }));

                    cycles.push(Box::new(|cpu| {
                        let addr = match cpu.get_current_instruction_ctx() {
                            Some(addr) => *addr,
                            None => panic!("could not retrieve address from ctx"),
                        };
                        let addr_hi = cpu.access_memory(addr + 1);
                        cpu.set_address_output_hi(addr_hi);

                        return TaskCycleVariant::Full;
                    }));
                    return cycles;
                }

                cycles.push(Box::new(|cpu| {
                    let addr = match cpu.get_current_instruction_ctx() {
                        Some(addr) => *addr,
                        None => panic!("could not retrieve address from ctx"),
                    };
                    let addr_lo = cpu.access_memory(addr);
                    cpu.set_address_output_lo(addr_lo);

                    return TaskCycleVariant::Full;
                }));

                cycles.push(Box::new(|cpu| {
                    let addr = match cpu.get_current_instruction_ctx() {
                        Some(addr) => *addr,
                        None => panic!("could not retrieve address from ctx"),
                    };
                    let should_incorrectly_jump = addr & 0x00FF == 0x00FF;
                    let mut target_addr = addr + 1;
                    if should_incorrectly_jump {
                        target_addr = addr & 0xFF00;
                    };
                    let addr_hi = cpu.access_memory(target_addr);
                    cpu.set_address_output_hi(addr_hi);

                    return TaskCycleVariant::Full;
                }));
            }
            AddressingMode::IndexIndirectX => {
                cycles.push(Box::new(|cpu| {
                    let addr: Byte = cpu.access_memory(cpu.program_counter);
                    cpu.set_address_output(addr);
                    cpu.queued_increment_program_counter();

                    return TaskCycleVariant::Full;
                }));

                cycles.push(Box::new(|cpu| {
                    let addr_output = cpu.address_output;
                    let target_address = addr_output.wrapping_add(cpu.index_register_x.into());
                    cpu.set_ctx(target_address);

                    return TaskCycleVariant::Full;
                }));

                cycles.push(Box::new(|cpu| {
                    let tgt_addr = match cpu.get_current_instruction_ctx() {
                        Some(addr) => *addr,
                        None => panic!("could not retrieve address from ctx"),
                    };

                    let addr_lo = cpu.access_memory(tgt_addr);
                    cpu.set_address_output_lo(addr_lo);

                    return TaskCycleVariant::Full;
                }));

                cycles.push(Box::new(|cpu| {
                    let tgt_addr = match cpu.get_current_instruction_ctx() {
                        Some(addr) => *addr,
                        None => panic!("could not retrieve address from ctx"),
                    };
                    let addr_hi = cpu.access_memory(tgt_addr.wrapping_add(1));
                    cpu.set_address_output_hi(addr_hi);

                    return TaskCycleVariant::Full;
                }));
            }
            AddressingMode::IndirectIndexY => {
                cycles.push(Box::new(|cpu| {
                    let addr: Byte = cpu.access_memory(cpu.program_counter);
                    cpu.set_ctx(addr.into());
                    cpu.queued_increment_program_counter();

                    return TaskCycleVariant::Full;
                }));

                cycles.push(Box::new(|cpu| {
                    let tgt_addr = match cpu.get_current_instruction_ctx() {
                        Some(addr) => *addr,
                        None => panic!("could not retrieve address from ctx"),
                    };

                    let addr_lo = cpu.access_memory(tgt_addr);
                    cpu.set_address_output_lo(addr_lo);

                    return TaskCycleVariant::Full;
                }));

                cycles.push(Box::new(|cpu| {
                    let tgt_addr = match cpu.get_current_instruction_ctx() {
                        Some(addr) => *addr,
                        None => panic!("could not retrieve address from ctx"),
                    };
                    let addr_hi = cpu.access_memory(tgt_addr.wrapping_add(1));
                    cpu.set_address_output_hi(addr_hi);

                    return TaskCycleVariant::Full;
                }));

                let mut offset_cycles = self.offset_address_output(self.index_register_y);
                cycles.append(&mut offset_cycles);
            }
            AddressingMode::Immediate => {
                cycles.push(Box::new(|cpu| {
                    let addr = cpu.program_counter;
                    cpu.set_address_output(addr);
                    cpu.queued_increment_program_counter();

                    return TaskCycleVariant::Partial;
                }));
            }
            AddressingMode::Implicit | AddressingMode::Relative => {}
            _ => {
                panic!("incorrect or unimplemented addressing used for queued fetch address");
            }
        }

        return cycles;
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
