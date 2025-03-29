use crate::cpu::{AddressingMode, Registers, Tasks, CPU};

struct AndTasks {
    done: bool,
    read_memory_tasks: Box<dyn Tasks>,
}

impl AndTasks {
    pub fn new(read_memory_tasks: Box<dyn Tasks>) -> Self {
        return AndTasks {
            done: false,
            read_memory_tasks,
        };
    }
}

impl Tasks for AndTasks {
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
        let result_value = cpu.get_register(Registers::Accumulator) & value;

        cpu.set_register(Registers::Accumulator, result_value);
        self.done = true;

        return self.done;
    }
}

pub fn and(cpu: &mut CPU, addr_mode: Option<AddressingMode>) -> Box<dyn Tasks> {
    let read_memory_tasks = cpu.read_memory(addr_mode, None);
    return Box::new(AndTasks::new(read_memory_tasks));
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

struct EorTasks {
    done: bool,
    read_memory_tasks: Box<dyn Tasks>,
}

impl EorTasks {
    pub fn new(read_memory_tasks: Box<dyn Tasks>) -> Self {
        return EorTasks {
            done: false,
            read_memory_tasks,
        };
    }
}

impl Tasks for EorTasks {
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
        let result_value = cpu.get_register(Registers::Accumulator) ^ value;

        cpu.set_register(Registers::Accumulator, result_value);
        self.done = true;

        return self.done;
    }
}

pub fn eor(cpu: &mut CPU, addr_mode: Option<AddressingMode>) -> Box<dyn Tasks> {
    let read_memory_tasks = cpu.read_memory(addr_mode, None);
    return Box::new(EorTasks::new(read_memory_tasks));
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

struct OraTasks {
    done: bool,
    read_memory_tasks: Box<dyn Tasks>,
}

impl OraTasks {
    pub fn new(read_memory_tasks: Box<dyn Tasks>) -> Self {
        return OraTasks {
            done: false,
            read_memory_tasks,
        };
    }
}

impl Tasks for OraTasks {
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
        let result_value = cpu.get_register(Registers::Accumulator) | value;

        cpu.set_register(Registers::Accumulator, result_value);
        self.done = true;

        return self.done;
    }
}

pub fn ora(cpu: &mut CPU, addr_mode: Option<AddressingMode>) -> Box<dyn Tasks> {
    let read_memory_tasks = cpu.read_memory(addr_mode, None);
    return Box::new(OraTasks::new(read_memory_tasks));
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

struct BitTasks {
    done: bool,
    read_memory_tasks: Box<dyn Tasks>,
}

impl BitTasks {
    pub fn new(read_memory_tasks: Box<dyn Tasks>) -> Self {
        return BitTasks {
            done: false,
            read_memory_tasks,
        };
    }
}

impl Tasks for BitTasks {
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
        cpu.set_bit_status(cpu.accumulator & value);
        self.done = true;

        return self.done;
    }
}

pub fn bit(cpu: &mut CPU, addr_mode: Option<AddressingMode>) -> Box<dyn Tasks> {
    let read_memory_tasks = cpu.read_memory(addr_mode, None);
    return Box::new(BitTasks::new(read_memory_tasks));
}

pub fn bit_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return bit(cpu, Some(AddressingMode::ZeroPage));
}

pub fn bit_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return bit(cpu, Some(AddressingMode::Absolute));
}

#[cfg(test)]
mod tests;
