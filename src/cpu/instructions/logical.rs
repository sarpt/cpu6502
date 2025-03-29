use crate::cpu::{AddressingMode, Registers, Tasks, CPU};

enum Variant {
    And,
    Eor,
    Ora,
    Bit,
}

struct LogicalTasks {
    done: bool,
    read_memory_tasks: Box<dyn Tasks>,
    variant: Variant,
}

impl LogicalTasks {
    pub fn new_and(read_memory_tasks: Box<dyn Tasks>) -> Self {
        return LogicalTasks {
            done: false,
            read_memory_tasks,
            variant: Variant::And,
        };
    }

    pub fn new_eor(read_memory_tasks: Box<dyn Tasks>) -> Self {
        return LogicalTasks {
            done: false,
            read_memory_tasks,
            variant: Variant::Eor,
        };
    }

    pub fn new_ora(read_memory_tasks: Box<dyn Tasks>) -> Self {
        return LogicalTasks {
            done: false,
            read_memory_tasks,
            variant: Variant::Ora,
        };
    }

    pub fn new_bit(read_memory_tasks: Box<dyn Tasks>) -> Self {
        return LogicalTasks {
            done: false,
            read_memory_tasks,
            variant: Variant::Bit,
        };
    }
}

impl Tasks for LogicalTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        if self.done {
            panic!("tick mustn't be called when done")
        }

        if !self.read_memory_tasks.done() {
            if !self.read_memory_tasks.tick(cpu) {
                return false;
            }
        }

        let value = match cpu.get_current_instruction_ctx() {
            Some(ctx) => ctx.to_le_bytes()[0],
            None => panic!("unexpected lack of value in instruction context after memory read"),
        };

        match self.variant {
            Variant::And => {
                let result_value = cpu.get_register(Registers::Accumulator) & value;

                cpu.set_register(Registers::Accumulator, result_value);
            }
            Variant::Eor => {
                let result_value = cpu.get_register(Registers::Accumulator) ^ value;

                cpu.set_register(Registers::Accumulator, result_value);
            }
            Variant::Ora => {
                let result_value = cpu.get_register(Registers::Accumulator) | value;

                cpu.set_register(Registers::Accumulator, result_value);
            }
            Variant::Bit => {
                cpu.set_bit_status(cpu.accumulator & value);
            }
        }
        self.done = true;

        return self.done;
    }
}

pub fn and(cpu: &mut CPU, addr_mode: Option<AddressingMode>) -> Box<dyn Tasks> {
    let read_memory_tasks = cpu.read_memory(addr_mode, None);
    return Box::new(LogicalTasks::new_and(read_memory_tasks));
}

pub fn and_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, None);
}

pub fn and_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, Some(AddressingMode::ZeroPage));
}

pub fn and_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, Some(AddressingMode::ZeroPageX));
}

pub fn and_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, Some(AddressingMode::Absolute));
}

pub fn and_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, Some(AddressingMode::AbsoluteX));
}

pub fn and_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, Some(AddressingMode::AbsoluteY));
}

pub fn and_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, Some(AddressingMode::IndexIndirectX));
}

pub fn and_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
    return and(cpu, Some(AddressingMode::IndirectIndexY));
}

pub fn eor(cpu: &mut CPU, addr_mode: Option<AddressingMode>) -> Box<dyn Tasks> {
    let read_memory_tasks = cpu.read_memory(addr_mode, None);
    return Box::new(LogicalTasks::new_eor(read_memory_tasks));
}

pub fn eor_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, None);
}

pub fn eor_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, Some(AddressingMode::ZeroPage));
}

pub fn eor_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, Some(AddressingMode::ZeroPageX));
}

pub fn eor_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, Some(AddressingMode::Absolute));
}

pub fn eor_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, Some(AddressingMode::AbsoluteX));
}

pub fn eor_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, Some(AddressingMode::AbsoluteY));
}

pub fn eor_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, Some(AddressingMode::IndexIndirectX));
}

pub fn eor_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
    return eor(cpu, Some(AddressingMode::IndirectIndexY));
}

pub fn ora(cpu: &mut CPU, addr_mode: Option<AddressingMode>) -> Box<dyn Tasks> {
    let read_memory_tasks = cpu.read_memory(addr_mode, None);
    return Box::new(LogicalTasks::new_ora(read_memory_tasks));
}

pub fn ora_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, None);
}

pub fn ora_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, Some(AddressingMode::ZeroPage));
}

pub fn ora_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, Some(AddressingMode::ZeroPageX));
}

pub fn ora_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, Some(AddressingMode::Absolute));
}

pub fn ora_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, Some(AddressingMode::AbsoluteX));
}

pub fn ora_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, Some(AddressingMode::AbsoluteY));
}

pub fn ora_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, Some(AddressingMode::IndexIndirectX));
}

pub fn ora_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
    return ora(cpu, Some(AddressingMode::IndirectIndexY));
}

pub fn bit(cpu: &mut CPU, addr_mode: Option<AddressingMode>) -> Box<dyn Tasks> {
    let read_memory_tasks = cpu.read_memory(addr_mode, None);
    return Box::new(LogicalTasks::new_bit(read_memory_tasks));
}

pub fn bit_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return bit(cpu, Some(AddressingMode::ZeroPage));
}

pub fn bit_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return bit(cpu, Some(AddressingMode::Absolute));
}

#[cfg(test)]
mod tests;
