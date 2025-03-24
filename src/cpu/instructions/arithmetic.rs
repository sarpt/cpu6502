use crate::{
    consts::Byte,
    cpu::{AddressingMode, Registers, Tasks, CPU},
};

fn compare(
    cpu: &mut CPU,
    addr_mode: Option<AddressingMode>,
    register: Registers,
) -> Box<dyn Tasks> {
    let read_memory_tasks = cpu.read_memory(addr_mode, None);
    return Box::new(CompareTasks::new(read_memory_tasks, register));
}

struct CompareTasks {
    done: bool,
    read_memory_tasks: Box<dyn Tasks>,
    register: Registers,
}

impl CompareTasks {
    pub fn new(read_memory_tasks: Box<dyn Tasks>, register: Registers) -> Self {
        return CompareTasks {
            read_memory_tasks,
            done: false,
            register,
        };
    }
}

impl Tasks for CompareTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        if self.done {
            panic!("tick should not be called when done")
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
        cpu.set_cmp_status(self.register, value);
        self.done = true;

        return true;
    }
}

pub fn cmp_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return compare(cpu, None, Registers::Accumulator);
}

pub fn cmp_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return compare(cpu, Some(AddressingMode::ZeroPage), Registers::Accumulator);
}

pub fn cmp_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return compare(cpu, Some(AddressingMode::ZeroPageX), Registers::Accumulator);
}

pub fn cmp_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return compare(cpu, Some(AddressingMode::Absolute), Registers::Accumulator);
}

pub fn cmp_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return compare(cpu, Some(AddressingMode::AbsoluteX), Registers::Accumulator);
}

pub fn cmp_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    return compare(cpu, Some(AddressingMode::AbsoluteY), Registers::Accumulator);
}

pub fn cmp_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return compare(
        cpu,
        Some(AddressingMode::IndexIndirectX),
        Registers::Accumulator,
    );
}

pub fn cmp_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
    return compare(
        cpu,
        Some(AddressingMode::IndirectIndexY),
        Registers::Accumulator,
    );
}

pub fn cpx_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return compare(cpu, None, Registers::IndexX);
}

pub fn cpx_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return compare(cpu, Some(AddressingMode::ZeroPage), Registers::IndexX);
}

pub fn cpx_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return compare(cpu, Some(AddressingMode::Absolute), Registers::IndexX);
}

pub fn cpy_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return compare(cpu, None, Registers::IndexY);
}

pub fn cpy_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return compare(cpu, Some(AddressingMode::ZeroPage), Registers::IndexY);
}

pub fn cpy_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return compare(cpu, Some(AddressingMode::Absolute), Registers::IndexY);
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FlagOp {
    Unchanged,
    Set,
    Clear,
}

fn adc(val: Byte, acc: Byte, _carry: bool) -> (Byte, FlagOp, FlagOp) {
    let (result, carry) = acc.overflowing_add(val);
    // if a sign (0x80) of a result differs from signs of both inputs
    let overflow = (acc ^ result) & (val ^ result) & 0x80 > 0;

    let carry_op = if carry {
        FlagOp::Set
    } else {
        FlagOp::Unchanged
    };
    let overflow_op = if overflow {
        FlagOp::Set
    } else {
        FlagOp::Unchanged
    };
    return (result, carry_op, overflow_op);
}

fn sbc(val: Byte, acc: Byte, carry: bool) -> (Byte, FlagOp, FlagOp) {
    let (result, carry) = acc.overflowing_add(0xFF - val + (carry as u8));
    // if a sign (0x80) of a result differs from sign of accumulator
    // and ones-complement of value sign differs from sign of result
    let overflow = (acc ^ result) & ((0xFF - val) ^ result) & 0x80 > 0;

    let carry_op = if carry {
        FlagOp::Clear
    } else {
        FlagOp::Unchanged
    };
    let overflow_op = if overflow {
        FlagOp::Set
    } else {
        FlagOp::Unchanged
    };
    return (result, carry_op, overflow_op);
}

pub fn operations_with_carry(
    cpu: &mut CPU,
    addr_mode: Option<AddressingMode>,
    op: fn(val: Byte, acc: Byte, carry: bool) -> (Byte, FlagOp, FlagOp),
) -> Box<dyn Tasks> {
    let cb: Box<dyn Fn(&mut CPU, Byte) -> ()> = Box::new(move |cpu, value| {
        let accumulator = cpu.get_register(Registers::Accumulator);
        let (value, carry, overflow) =
            op(value, accumulator, cpu.processor_status.get_carry_flag());

        cpu.set_register(Registers::Accumulator, value);

        if carry != FlagOp::Unchanged {
            cpu.processor_status.change_carry_flag(carry == FlagOp::Set)
        }
        if overflow != FlagOp::Unchanged {
            cpu.processor_status
                .change_overflow_flag(overflow == FlagOp::Set)
        }
    });

    return cpu.read_memory(addr_mode, Some(cb));
}

pub fn adc_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return operations_with_carry(cpu, None, adc);
}

pub fn adc_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return operations_with_carry(cpu, Some(AddressingMode::ZeroPage), adc);
}

pub fn adc_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return operations_with_carry(cpu, Some(AddressingMode::ZeroPageX), adc);
}

pub fn adc_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return operations_with_carry(cpu, Some(AddressingMode::Absolute), adc);
}

pub fn adc_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return operations_with_carry(cpu, Some(AddressingMode::AbsoluteX), adc);
}

pub fn adc_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    return operations_with_carry(cpu, Some(AddressingMode::AbsoluteY), adc);
}

pub fn adc_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return operations_with_carry(cpu, Some(AddressingMode::IndexIndirectX), adc);
}

pub fn adc_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
    return operations_with_carry(cpu, Some(AddressingMode::IndirectIndexY), adc);
}

pub fn sbc_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return operations_with_carry(cpu, None, sbc);
}

pub fn sbc_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return operations_with_carry(cpu, Some(AddressingMode::ZeroPage), sbc);
}

pub fn sbc_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return operations_with_carry(cpu, Some(AddressingMode::ZeroPageX), sbc);
}

pub fn sbc_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return operations_with_carry(cpu, Some(AddressingMode::Absolute), sbc);
}

pub fn sbc_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return operations_with_carry(cpu, Some(AddressingMode::AbsoluteX), sbc);
}

pub fn sbc_ay(cpu: &mut CPU) -> Box<dyn Tasks> {
    return operations_with_carry(cpu, Some(AddressingMode::AbsoluteY), sbc);
}

pub fn sbc_inx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return operations_with_carry(cpu, Some(AddressingMode::IndexIndirectX), sbc);
}

pub fn sbc_iny(cpu: &mut CPU) -> Box<dyn Tasks> {
    return operations_with_carry(cpu, Some(AddressingMode::IndirectIndexY), sbc);
}

#[cfg(test)]
mod tests;
