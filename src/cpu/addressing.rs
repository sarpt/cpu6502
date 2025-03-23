use crate::consts::{Byte, Word};

use super::{
    tasks::{GenericTasks, Tasks},
    AddressingMode, ChipVariant, CPU,
};

pub struct ZeroPageAddressingTasks {
    done: bool,
}

impl ZeroPageAddressingTasks {
    pub fn new() -> Self {
        return ZeroPageAddressingTasks { done: false };
    }
}

impl Tasks for ZeroPageAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> (bool, bool) {
        if self.done {
            return (false, self.done);
        }

        let addr: Byte = cpu.access_memory(cpu.program_counter);
        cpu.set_address_output(addr);
        cpu.increment_program_counter();
        self.done = true;

        return (true, self.done);
    }
}

pub struct ImmediateAddressingTasks {
    done: bool,
}

impl ImmediateAddressingTasks {
    pub fn new() -> Self {
        return ImmediateAddressingTasks { done: false };
    }
}

impl Tasks for ImmediateAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> (bool, bool) {
        if self.done {
            return (false, self.done);
        }

        let addr = cpu.program_counter;
        cpu.set_address_output(addr);
        cpu.increment_program_counter();
        self.done = true;

        return (false, self.done);
    }
}

enum OffsetVariant {
    X,
    Y,
}

enum AbsoluteOffsetStep {
    MemoryAccessLo,
    MemoryAccessHi,
    OffsetLo,
    OffsetHi,
}

pub struct AbsoluteOffsetAddressingTasks {
    done: bool,
    step: AbsoluteOffsetStep,
    variant: OffsetVariant,
}

impl AbsoluteOffsetAddressingTasks {
    pub fn new_offset_by_x() -> Self {
        return AbsoluteOffsetAddressingTasks {
            done: false,
            step: AbsoluteOffsetStep::MemoryAccessLo,
            variant: OffsetVariant::X,
        };
    }

    pub fn new_offset_by_y() -> Self {
        return AbsoluteOffsetAddressingTasks {
            done: false,
            step: AbsoluteOffsetStep::MemoryAccessLo,
            variant: OffsetVariant::Y,
        };
    }
}

impl Tasks for AbsoluteOffsetAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> (bool, bool) {
        if self.done {
            return (false, self.done);
        }

        match self.step {
            AbsoluteOffsetStep::MemoryAccessLo => {
                let addr_lo = cpu.access_memory(cpu.program_counter);
                cpu.set_address_output_lo(addr_lo);
                cpu.increment_program_counter();
                self.step = AbsoluteOffsetStep::MemoryAccessHi;

                return (true, false);
            }
            AbsoluteOffsetStep::MemoryAccessHi => {
                let addr_hi = cpu.access_memory(cpu.program_counter);
                cpu.set_address_output_hi(addr_hi);
                cpu.increment_program_counter();
                self.step = AbsoluteOffsetStep::OffsetLo;

                return (true, false);
            }
            AbsoluteOffsetStep::OffsetLo => {
                let offset = match self.variant {
                    OffsetVariant::X => cpu.index_register_x,
                    OffsetVariant::Y => cpu.index_register_y,
                };
                let [lo, hi] = cpu.address_output.to_le_bytes();
                let (new_lo, carry) = lo.overflowing_add(offset);
                cpu.address_output = Word::from_le_bytes([new_lo, hi]);
                self.step = AbsoluteOffsetStep::OffsetHi;

                if !carry {
                    self.done = true;
                }
                return (true, self.done);
            }
            AbsoluteOffsetStep::OffsetHi => {
                let [lo, hi] = cpu.address_output.to_le_bytes();
                let new_hi = hi.wrapping_add(1);
                cpu.address_output = Word::from_le_bytes([lo, new_hi]);

                self.done = true;
                return (true, self.done);
            }
        }
    }
}

enum ZeroPageOffsetStep {
    ZeroPageAccess,
    Offset,
}

pub struct ZeroPageOffsetAddressingTasks {
    done: bool,
    step: ZeroPageOffsetStep,
    variant: OffsetVariant,
}

impl ZeroPageOffsetAddressingTasks {
    pub fn new_offset_by_x() -> Self {
        return ZeroPageOffsetAddressingTasks {
            done: false,
            step: ZeroPageOffsetStep::ZeroPageAccess,
            variant: OffsetVariant::X,
        };
    }

    pub fn new_offset_by_y() -> Self {
        return ZeroPageOffsetAddressingTasks {
            done: false,
            step: ZeroPageOffsetStep::ZeroPageAccess,
            variant: OffsetVariant::Y,
        };
    }
}

impl Tasks for ZeroPageOffsetAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> (bool, bool) {
        if self.done {
            return (false, self.done);
        }

        match self.step {
            ZeroPageOffsetStep::ZeroPageAccess => {
                let addr: Byte = cpu.access_memory(cpu.program_counter);
                cpu.set_address_output(addr);
                cpu.increment_program_counter();
                self.step = ZeroPageOffsetStep::Offset;

                return (true, false);
            }
            ZeroPageOffsetStep::Offset => {
                let offset: Byte = match self.variant {
                    OffsetVariant::X => cpu.index_register_x.into(),
                    OffsetVariant::Y => cpu.index_register_y.into(),
                };
                let addr_output = cpu.address_output as Byte;
                let final_address = addr_output.wrapping_add(offset);
                cpu.set_address_output(final_address);

                self.done = true;
                return (true, self.done);
            }
        }
    }
}

enum AbsoluteStep {
    MemoryLo,
    MemoryHi,
}

pub struct AbsoluteAddressingTasks {
    done: bool,
    step: AbsoluteStep,
}

impl AbsoluteAddressingTasks {
    pub fn new() -> Self {
        return AbsoluteAddressingTasks {
            done: false,
            step: AbsoluteStep::MemoryLo,
        };
    }
}

impl Tasks for AbsoluteAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> (bool, bool) {
        if self.done {
            return (false, self.done);
        }

        match self.step {
            AbsoluteStep::MemoryLo => {
                let addr_lo = cpu.access_memory(cpu.program_counter);
                cpu.set_address_output_lo(addr_lo);
                cpu.increment_program_counter();
                self.step = AbsoluteStep::MemoryHi;

                return (true, false);
            }
            AbsoluteStep::MemoryHi => {
                let addr_hi = cpu.access_memory(cpu.program_counter);
                cpu.set_address_output_hi(addr_hi);
                cpu.increment_program_counter();

                self.done = true;
                return (true, self.done);
            }
        }
    }
}

enum IndirectIndexYStep {
    MemoryAccess,
    IndirectAccessLo,
    IndirectAccessHi,
    OffsetLo,
    OffsetHi,
}

pub struct IndirectIndexYAddressingTasks {
    done: bool,
    step: IndirectIndexYStep,
    tgt_addr: Word,
}

impl IndirectIndexYAddressingTasks {
    pub fn new() -> Self {
        return IndirectIndexYAddressingTasks {
            done: false,
            step: IndirectIndexYStep::MemoryAccess,
            tgt_addr: Word::default(),
        };
    }
}

impl Tasks for IndirectIndexYAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> (bool, bool) {
        if self.done {
            return (false, self.done);
        }

        match self.step {
            IndirectIndexYStep::MemoryAccess => {
                let addr: Byte = cpu.access_memory(cpu.program_counter);
                self.tgt_addr = addr.into();
                cpu.increment_program_counter();
                self.step = IndirectIndexYStep::IndirectAccessLo;

                return (true, false);
            }
            IndirectIndexYStep::IndirectAccessLo => {
                let addr_lo = cpu.access_memory(self.tgt_addr);
                cpu.set_address_output_lo(addr_lo);
                self.step = IndirectIndexYStep::IndirectAccessHi;

                return (true, false);
            }
            IndirectIndexYStep::IndirectAccessHi => {
                let addr_hi = cpu.access_memory(self.tgt_addr.wrapping_add(1));
                cpu.set_address_output_hi(addr_hi);
                self.step = IndirectIndexYStep::OffsetLo;

                return (true, false);
            }
            IndirectIndexYStep::OffsetLo => {
                let [lo, hi] = cpu.address_output.to_le_bytes();
                let (new_lo, carry) = lo.overflowing_add(cpu.index_register_y);
                cpu.address_output = Word::from_le_bytes([new_lo, hi]);
                self.step = IndirectIndexYStep::OffsetHi;

                if !carry {
                    self.done = true;
                }
                return (true, self.done);
            }
            IndirectIndexYStep::OffsetHi => {
                let [lo, hi] = cpu.address_output.to_le_bytes();
                let new_hi = hi.wrapping_add(1);
                cpu.address_output = Word::from_le_bytes([lo, new_hi]);

                self.done = true;
                return (true, self.done);
            }
        }
    }
}

enum IndexIndirectXStep {
    IndirectAccess,
    SumWithX,
    MemoryAccessLo,
    MemoryAccessHi,
}
pub struct IndexIndirectXAddressingTasks {
    done: bool,
    step: IndexIndirectXStep,
    tgt_addr: Word,
}

impl IndexIndirectXAddressingTasks {
    pub fn new() -> Self {
        return IndexIndirectXAddressingTasks {
            done: false,
            step: IndexIndirectXStep::IndirectAccess,
            tgt_addr: Word::default(),
        };
    }
}

impl Tasks for IndexIndirectXAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> (bool, bool) {
        if self.done {
            return (false, self.done);
        }

        match self.step {
            IndexIndirectXStep::IndirectAccess => {
                let addr: Byte = cpu.access_memory(cpu.program_counter);
                cpu.set_address_output(addr);
                cpu.increment_program_counter();
                self.step = IndexIndirectXStep::SumWithX;

                return (true, false);
            }
            IndexIndirectXStep::SumWithX => {
                let addr_output = cpu.address_output;
                self.tgt_addr = addr_output.wrapping_add(cpu.index_register_x.into());
                self.step = IndexIndirectXStep::MemoryAccessLo;

                return (true, false);
            }
            IndexIndirectXStep::MemoryAccessLo => {
                let addr_lo = cpu.access_memory(self.tgt_addr);
                cpu.set_address_output_lo(addr_lo);
                self.step = IndexIndirectXStep::MemoryAccessHi;

                return (true, false);
            }
            IndexIndirectXStep::MemoryAccessHi => {
                let addr_hi = cpu.access_memory(self.tgt_addr.wrapping_add(1));
                cpu.set_address_output_hi(addr_hi);

                self.done = true;
                return (true, self.done);
            }
        }
    }
}

enum IndirectStep {
    IndirectFetchLo,
    IndirectFetchHi,
    AddrFixing,
    MemoryAccessLo,
    FixedMemoryAccessHi,
    IncorrectMemoryAccessHi,
}

pub struct IndirectAddressingTasks {
    fixed_addressing: bool,
    done: bool,
    step: IndirectStep,
    tgt_addr_lo: Byte,
    tgt_addr_hi: Byte,
}

impl IndirectAddressingTasks {
    pub fn new_fixed_addressing() -> Self {
        return IndirectAddressingTasks {
            fixed_addressing: true,
            done: false,
            step: IndirectStep::IndirectFetchLo,
            tgt_addr_lo: Byte::default(),
            tgt_addr_hi: Byte::default(),
        };
    }

    pub fn new_incorrect_addressing() -> Self {
        return IndirectAddressingTasks {
            fixed_addressing: false,
            done: false,
            step: IndirectStep::IndirectFetchLo,
            tgt_addr_lo: Byte::default(),
            tgt_addr_hi: Byte::default(),
        };
    }
}

impl Tasks for IndirectAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> (bool, bool) {
        if self.done {
            return (false, self.done);
        }

        match self.step {
            IndirectStep::IndirectFetchLo => {
                self.tgt_addr_lo = cpu.access_memory(cpu.program_counter);
                cpu.increment_program_counter();
                self.step = IndirectStep::IndirectFetchHi;

                return (true, false);
            }
            IndirectStep::IndirectFetchHi => {
                self.tgt_addr_hi = cpu.access_memory(cpu.program_counter);
                cpu.increment_program_counter();
                if self.fixed_addressing {
                    self.step = IndirectStep::AddrFixing;
                } else {
                    self.step = IndirectStep::MemoryAccessLo;
                }

                return (true, false);
            }
            IndirectStep::AddrFixing => {
                self.step = IndirectStep::MemoryAccessLo;

                return (true, false);
            }
            IndirectStep::MemoryAccessLo => {
                let addr = Word::from_le_bytes([self.tgt_addr_lo, self.tgt_addr_hi]);
                let addr_lo = cpu.access_memory(addr);
                cpu.set_address_output_lo(addr_lo);

                if self.fixed_addressing {
                    self.step = IndirectStep::FixedMemoryAccessHi;
                } else {
                    self.step = IndirectStep::IncorrectMemoryAccessHi;
                }

                return (true, false);
            }
            IndirectStep::FixedMemoryAccessHi => {
                let addr = Word::from_le_bytes([self.tgt_addr_lo, self.tgt_addr_hi]);
                let addr_hi = cpu.access_memory(addr + 1);
                cpu.set_address_output_hi(addr_hi);

                self.done = true;
                return (true, self.done);
            }
            IndirectStep::IncorrectMemoryAccessHi => {
                let addr = Word::from_le_bytes([self.tgt_addr_lo, self.tgt_addr_hi]);
                let should_incorrectly_jump = self.tgt_addr_lo == 0xFF;
                let mut target_addr = addr + 1;
                if should_incorrectly_jump {
                    target_addr = addr & 0xFF00;
                };
                let addr_hi = cpu.access_memory(target_addr);
                cpu.set_address_output_hi(addr_hi);

                self.done = true;
                return (true, self.done);
            }
        }
    }
}

pub fn get_addressing_tasks(cpu: &CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
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
            if cpu.chip_variant == ChipVariant::NMOS {
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

#[cfg(test)]
mod get_addressing_tasks {
    #[cfg(test)]
    mod absolute_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_return_address_from_next_word_in_memory_relative_to_program_counter() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x01;
            cpu.address_output = 0x0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Absolute);
            run_tasks(&mut cpu, tasks);
            assert_eq!(cpu.address_output, 0xCBFF);
        }

        #[test]
        fn should_advance_program_counter_twice() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x01;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Absolute);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x03);
        }

        #[test]
        fn should_take_two_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x01;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Absolute);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod absolute_x_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_return_address_offset_by_index_register_x_from_next_word_in_memory_relative_to_program_counter(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_x = 0x01;
            cpu.address_output = 0x0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::AbsoluteX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0x52CC);
        }

        #[test]
        fn should_advance_program_counter_twice() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_x = 0x01;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::AbsoluteX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x04);
        }

        #[test]
        fn should_take_three_cycles_when_not_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_x = 0x01;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::AbsoluteX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_x = 0xFF;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::AbsoluteX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod absolute_y_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_return_address_offset_by_index_register_y_from_next_word_in_memory_relative_to_program_counter(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.index_register_y = 0x01;
            cpu.program_counter = 0x02;
            cpu.address_output = 0x0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::AbsoluteY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0x52CC);
        }

        #[test]
        fn should_advance_program_counter_twice() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.index_register_y = 0x01;
            cpu.program_counter = 0x02;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::AbsoluteY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x04);
        }

        #[test]
        fn should_take_three_cycles_when_not_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_y = 0x01;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::AbsoluteY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_y = 0xFF;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::AbsoluteY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod immediate_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_return_program_counter_address() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.address_output = 0x0;
            cpu.program_counter = 0xCB;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Immediate);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0xCB);
        }

        #[test]
        fn should_advance_program_counter() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0xCB;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Immediate);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0xCC);
        }

        #[test]
        fn should_not_take_any_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0xCB;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Immediate);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 0);
        }
    }

    #[cfg(test)]
    mod index_indirect_x_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_return_address_stored_in_place_pointed_by_zero_page_address_in_next_byte_relative_to_program_counter_summed_with_index_register_x(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x01, 0xFF, 0x03, 0xDD, 0x25]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.address_output = 0x0;
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x01;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::IndexIndirectX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0xDD03);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let memory = &RefCell::new(MemoryMock::new(&[0x01, 0xFF, 0x03, 0xDD, 0x25]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x01;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::IndexIndirectX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x01);
        }

        #[test]
        fn should_take_four_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[0x01, 0xFF, 0x03, 0xDD, 0x25]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x01;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::IndexIndirectX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod indirect_index_y_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_return_address_offset_by_index_register_y_which_is_stored_at_zero_page_address() {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.address_output = 0x0;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::IndirectIndexY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0xDD05);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::IndirectIndexY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x01);
        }

        #[test]
        fn should_take_four_cycles_when_not_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::IndirectIndexY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 4);
        }

        #[test]
        fn should_take_five_cycles_when_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.index_register_y = 0xFF;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::IndirectIndexY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 5);
        }
    }

    #[cfg(test)]
    mod zero_page_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_return_address_in_zero_page_from_next_byte_in_memory_relative_to_program_counter()
        {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.address_output = 0x0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPage);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0x00CB);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPage);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x03);
        }

        #[test]
        fn should_take_one_cycle() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPage);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod zero_page_x_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_return_address_in_zero_page_from_next_byte_in_memory_relative_to_program_counter_summed_with_index_register_x(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_x = 0x03;
            cpu.address_output = 0x0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPageX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0x00CE);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_x = 0x03;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPageX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x03);
        }

        #[test]
        fn should_take_two_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_x = 0x03;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPageX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod zero_page_y_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_return_address_in_zero_page_from_next_byte_in_memory_relative_to_program_counter_summed_with_index_register_y(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x03;
            cpu.index_register_y = 0x03;
            cpu.address_output = 0x0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPageY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0x0055);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_y = 0x03;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPageY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x03);
        }

        #[test]
        fn should_take_two_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_y = 0x03;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPageY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod indirect_addressing {
        #[cfg(test)]
        mod common {
            use std::cell::RefCell;

            use crate::cpu::{
                addressing::get_addressing_tasks,
                tests::{run_tasks, MemoryMock},
                AddressingMode, CPU,
            };

            #[test]
            fn should_return_address_from_place_in_memory_stored_in_next_word_relative_to_program_counter(
            ) {
                let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
                let mut cpu = CPU::new_nmos(&memory);
                cpu.program_counter = 0x00;
                cpu.address_output = 0x0;

                let tasks = get_addressing_tasks(&cpu, AddressingMode::Indirect);
                run_tasks(&mut cpu, tasks);

                assert_eq!(cpu.address_output, 0x0001);
            }

            #[test]
            fn should_advance_program_counter_twice() {
                let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
                let mut cpu = CPU::new_nmos(&memory);
                cpu.program_counter = 0x00;

                let tasks = get_addressing_tasks(&cpu, AddressingMode::Indirect);
                run_tasks(&mut cpu, tasks);

                assert_eq!(cpu.program_counter, 0x02);
            }
        }

        #[cfg(test)]
        mod nmos {
            use std::cell::RefCell;

            use crate::{
                consts::Byte,
                cpu::{
                    addressing::get_addressing_tasks,
                    tests::{run_tasks, MemoryMock},
                    AddressingMode, CPU,
                },
            };

            #[test]
            fn should_take_four_cycles() {
                let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
                let mut cpu = CPU::new_nmos(&memory);
                cpu.program_counter = 0x02;
                cpu.cycle = 0;

                let tasks = get_addressing_tasks(&cpu, AddressingMode::Indirect);
                run_tasks(&mut cpu, tasks);

                assert_eq!(cpu.cycle, 4);
            }

            #[test]
            fn should_incorrectly_fetch_target_address_when_indirect_address_is_falling_on_a_page_boundary_and_take_lo_from_correct_address_but_use_indirect_address_for_hi(
            ) {
                const INDIRECT_ADDR_LO: Byte = 0xFF;
                const INDIRECT_ADDR_HI: Byte = 0x00;
                const TARGET_ADDR_LO: Byte = 0xA5;
                const TARGET_ADDR_HI: Byte = 0xCC;
                const INCORRECT_TARGET_ADDR_HI: Byte = 0x09;

                let mut program: [Byte; 512] = [0x00; 512];
                program[0x0000] = INCORRECT_TARGET_ADDR_HI;
                program[0x0001] = INDIRECT_ADDR_LO;
                program[0x0002] = INDIRECT_ADDR_HI;
                program[0x00FF] = TARGET_ADDR_LO;
                program[0x0100] = TARGET_ADDR_HI;

                let memory = RefCell::new(MemoryMock::new(&program));
                let mut cpu = CPU::new_nmos(&memory);
                cpu.program_counter = 0x0001;
                cpu.address_output = 0x0;
                cpu.cycle = 0;

                let tasks = get_addressing_tasks(&cpu, AddressingMode::Indirect);
                run_tasks(&mut cpu, tasks);

                assert_eq!(cpu.address_output, 0x09A5);
            }
        }

        #[cfg(test)]
        mod cmos {
            use std::cell::RefCell;

            use crate::{
                consts::Byte,
                cpu::{
                    addressing::get_addressing_tasks,
                    tests::{run_tasks, MemoryMock},
                    AddressingMode, CPU,
                },
            };

            #[test]
            fn should_take_five_cycles() {
                let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
                let mut cpu = CPU::new_rockwell_cmos(&memory);
                cpu.program_counter = 0x02;
                cpu.cycle = 0;

                let tasks = get_addressing_tasks(&cpu, AddressingMode::Indirect);
                run_tasks(&mut cpu, tasks);

                assert_eq!(cpu.cycle, 5);
            }

            #[test]
            fn should_correctly_fetch_target_address_when_indirect_address_is_falling_on_a_page_boundary(
            ) {
                const INDIRECT_ADDR_LO: Byte = 0xFF;
                const INDIRECT_ADDR_HI: Byte = 0x00;
                const TARGET_ADDR_LO: Byte = 0xA5;
                const TARGET_ADDR_HI: Byte = 0xCC;

                let mut program: [Byte; 512] = [0x00; 512];
                program[0x0001] = INDIRECT_ADDR_LO;
                program[0x0002] = INDIRECT_ADDR_HI;
                program[0x00FF] = TARGET_ADDR_LO;
                program[0x0100] = TARGET_ADDR_HI;

                let memory = RefCell::new(MemoryMock::new(&program));
                let mut cpu = CPU::new_rockwell_cmos(&memory);
                cpu.program_counter = 0x0001;
                cpu.address_output = 0x0;
                cpu.cycle = 0;

                let tasks = get_addressing_tasks(&cpu, AddressingMode::Indirect);
                run_tasks(&mut cpu, tasks);

                assert_eq!(cpu.address_output, 0xCCA5);
            }
        }
    }

    #[cfg(test)]
    mod implicit_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_not_change_address_output() {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.address_output = 0x0;
            cpu.program_counter = 0x00;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Implicit);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0x0);
        }

        #[test]
        fn should_not_advance_program_counter() {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.program_counter = 0x00;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Implicit);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x00);
        }

        #[test]
        fn should_take_zero_cycles() {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.program_counter = 0x02;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Implicit);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 0);
        }
    }

    #[cfg(test)]
    mod relative_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_not_change_address_output() {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.program_counter = 0x00;
            cpu.address_output = 0x0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Relative);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0x0);
        }

        #[test]
        fn should_not_advance_program_counter() {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.program_counter = 0x00;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Relative);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x00);
        }

        #[test]
        fn should_take_zero_cycles() {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.program_counter = 0x02;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Relative);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 0);
        }
    }
}
