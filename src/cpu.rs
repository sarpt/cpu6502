use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use addressing::{
    AbsoluteAddressingTasks, AbsoluteOffsetAddressingTasks, ImmediateAddressingTasks,
    IndexIndirectXAddressingTasks, IndirectAddressingTasks, IndirectIndexYAddressingTasks,
    ZeroPageAddressingTasks, ZeroPageOffsetAddressingTasks,
};
use tasks::{GenericTasks, TaskCycleVariant, Tasks};

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

type OpcodeHandler = fn(&mut CPU) -> Box<dyn Tasks>;

type InstructionCtx = Option<Word>;
struct InstructionExecution {
    opcode: Byte,
    ctx: InstructionCtx,
    starting_cycle: usize,
    tasks: Box<dyn Tasks>,
}

pub struct CPU<'a> {
    chip_variant: ChipVariant,
    current_instruction: Option<InstructionExecution>,
    cycle: usize,
    program_counter: Word,
    stack_pointer: Byte,
    accumulator: Byte,
    index_register_x: Byte,
    index_register_y: Byte,
    processor_status: processor_status::ProcessorStatus,
    memory: &'a RefCell<dyn Memory>,
    opcode_handlers: HashMap<Byte, OpcodeHandler>,
    address_output: Word,
    sync: bool,
}

impl<'a> CPU<'a> {
    fn new(memory: &'a RefCell<dyn Memory>, chip_variant: ChipVariant) -> Self {
        return CPU {
            chip_variant: chip_variant,
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
        match &mut self.current_instruction {
            Some(current_instruction) => {
                self.sync = false;
                let mut tasks = std::mem::replace(
                    &mut current_instruction.tasks,
                    Box::new(GenericTasks::new()),
                );
                let (took_cycles, tasks_done) = tasks.tick(self);
                if took_cycles {
                    self.cycle += 1;
                }
                if tasks_done {
                    self.current_instruction = None;
                    return;
                }

                let current_instruction_after_running_task = self
                    .current_instruction
                    .as_mut()
                    .expect("non-instruction fetching tick encountered current_instruction as none after running task");
                std::mem::swap(
                    &mut current_instruction_after_running_task.tasks,
                    &mut tasks,
                );
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

    fn access_memory(&mut self, addr: Word) -> Byte {
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

    fn set_address_output<T: Into<Word>>(&mut self, val: T) {
        self.address_output = val.into();
    }

    fn set_address_output_lo(&mut self, lo: Byte) {
        self.address_output = Word::from_le_bytes([lo, self.address_output.to_le_bytes()[1]]);
    }

    fn set_address_output_hi(&mut self, hi: Byte) {
        self.address_output = Word::from_le_bytes([self.address_output.to_le_bytes()[0], hi]);
    }

    fn get_current_instruction_ctx(&self) -> Option<Word> {
        return match &self.current_instruction {
            Some(current_instruciton) => current_instruciton.ctx,
            None => panic!("cannot get ctx for non-exisiting instruction"),
        };
    }

    fn set_current_instruction_ctx(&mut self, vals: (Option<Byte>, Option<Byte>)) {
        match &mut self.current_instruction {
            Some(current_instruction) => {
                let [old_lo, old_hi] = current_instruction.ctx.unwrap_or(0).to_le_bytes();
                let new_lo = vals.0.unwrap_or(old_lo);
                let new_hi = vals.1.unwrap_or(old_hi);
                let new_val = Word::from_le_bytes([new_lo, new_hi]);
                current_instruction.ctx = Some(new_val);
            }
            None => panic!("cannot get ctx for non-exisiting instruction"),
        };
    }

    fn set_ctx_lo(&mut self, lo: Byte) {
        self.set_current_instruction_ctx((Some(lo), None));
    }

    fn set_ctx_hi(&mut self, hi: Byte) {
        self.set_current_instruction_ctx((None, Some(hi)));
    }

    fn read_memory(
        &mut self,
        addr_mode: AddressingMode,
        value_reader: Option<Box<dyn Fn(&mut CPU, Byte) -> ()>>,
    ) -> Box<dyn Tasks> {
        let addr_tasks = self.get_address(addr_mode);
        let mut tasks = GenericTasks::new_dependent(addr_tasks);

        tasks.push(Rc::new(move |cpu: &mut CPU| {
            let value = cpu.access_memory(cpu.address_output);
            cpu.set_ctx_lo(value);

            if let Some(vr) = &value_reader {
                vr(cpu, value)
            }

            if access_cycle_has_been_done_during_address_fixing(addr_mode) {
                return TaskCycleVariant::Partial;
            }

            return TaskCycleVariant::Full;
        }));

        return Box::new(tasks);
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
            ctx: None,
            tasks: tasks,
            opcode: opcode,
            starting_cycle: self.cycle,
        };
    }

    fn get_address(&mut self, addr_mode: AddressingMode) -> Box<dyn Tasks> {
        match addr_mode {
            AddressingMode::ZeroPage => {
                return Box::new(ZeroPageAddressingTasks::new());
            }
            AddressingMode::ZeroPageX => {
                return Box::new(ZeroPageOffsetAddressingTasks::new_offset_by_x());
            }
            AddressingMode::ZeroPageY => {
                return Box::new(ZeroPageOffsetAddressingTasks::new_offset_by_y());
            }
            AddressingMode::Absolute => {
                return Box::new(AbsoluteAddressingTasks::new());
            }
            AddressingMode::AbsoluteX => {
                return Box::new(AbsoluteOffsetAddressingTasks::new_offset_by_x());
            }
            AddressingMode::AbsoluteY => {
                return Box::new(AbsoluteOffsetAddressingTasks::new_offset_by_y());
            }
            AddressingMode::Indirect => {
                if self.chip_variant == ChipVariant::NMOS {
                    return Box::new(IndirectAddressingTasks::new_incorrect_addressing());
                } else {
                    return Box::new(IndirectAddressingTasks::new_fixed_addressing());
                }
            }
            AddressingMode::IndexIndirectX => {
                return Box::new(IndexIndirectXAddressingTasks::new());
            }
            AddressingMode::IndirectIndexY => {
                return Box::new(IndirectIndexYAddressingTasks::new());
            }
            AddressingMode::Immediate => {
                return Box::new(ImmediateAddressingTasks::new());
            }
            _ => {
                return Box::new(GenericTasks::new());
            }
        }
    }
}

fn access_cycle_has_been_done_during_address_fixing(addr_mode: AddressingMode) -> bool {
    return addr_mode == AddressingMode::AbsoluteX
        || addr_mode == AddressingMode::AbsoluteY
        || addr_mode == AddressingMode::IndirectIndexY;
}

#[cfg(test)]
mod tests;
