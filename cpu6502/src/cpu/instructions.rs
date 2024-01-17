use super::{
    processor_status::Flags, AddressingMode, MemoryModifications, Registers, BRK_INTERRUPT_VECTOR,
    CPU,
};

fn ld(cpu: &mut CPU, addr_mode: AddressingMode, register: Registers) {
    let value = match cpu.read_memory(addr_mode) {
        Some(value) => value,
        None => panic!("ld used with incorrect address mode"),
    };

    cpu.set_register(register, value);
    cpu.set_status_of_register(register);
}

pub fn lda_im(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Immediate, Registers::Accumulator);
}

pub fn lda_zp(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPage, Registers::Accumulator);
}

pub fn lda_zpx(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPageX, Registers::Accumulator);
}

pub fn lda_a(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Absolute, Registers::Accumulator);
}

pub fn lda_ax(cpu: &mut CPU) {
    ld(cpu, AddressingMode::AbsoluteX, Registers::Accumulator);
}

pub fn lda_ay(cpu: &mut CPU) {
    ld(cpu, AddressingMode::AbsoluteY, Registers::Accumulator);
}

pub fn lda_inx(cpu: &mut CPU) {
    ld(cpu, AddressingMode::IndexIndirectX, Registers::Accumulator);
}

pub fn lda_iny(cpu: &mut CPU) {
    ld(cpu, AddressingMode::IndirectIndexY, Registers::Accumulator);
}

pub fn ldy_im(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Immediate, Registers::IndexY);
}

pub fn ldy_zp(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPage, Registers::IndexY);
}

pub fn ldy_zpx(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPageX, Registers::IndexY);
}

pub fn ldy_a(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Absolute, Registers::IndexY);
}

pub fn ldy_ax(cpu: &mut CPU) {
    ld(cpu, AddressingMode::AbsoluteX, Registers::IndexY);
}

pub fn ldx_im(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Immediate, Registers::IndexX);
}

pub fn ldx_zp(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPage, Registers::IndexX);
}

pub fn ldx_zpy(cpu: &mut CPU) {
    ld(cpu, AddressingMode::ZeroPageY, Registers::IndexX);
}

pub fn ldx_a(cpu: &mut CPU) {
    ld(cpu, AddressingMode::Absolute, Registers::IndexX);
}

pub fn ldx_ay(cpu: &mut CPU) {
    ld(cpu, AddressingMode::AbsoluteY, Registers::IndexX);
}

pub fn jsr_a(cpu: &mut CPU) {
    let jump_addr = match cpu.get_address(AddressingMode::Absolute, super::MemoryOperation::Read) {
        Some(address) => address,
        None => panic!("couldn't fetch address during a jsr"),
    };

    cpu.push_word_to_stack(cpu.program_counter - 1);
    cpu.program_counter = jump_addr;
    cpu.cycle += 1;
}

pub fn rts(cpu: &mut CPU) {
    cpu.access_memory(cpu.program_counter); // fetch and discard
    cpu.cycle += 1;

    cpu.program_counter = cpu.pop_word_from_stack();
    cpu.cycle += 1;
    cpu.increment_program_counter();
}

fn jmp(cpu: &mut CPU, addr_mode: AddressingMode) {
    match cpu.get_address(addr_mode, super::MemoryOperation::Read) {
        Some(address) => cpu.program_counter = address,
        None => panic!("jmp used with incorrect addressing mode"),
    }
}

pub fn jmp_a(cpu: &mut CPU) {
    jmp(cpu, AddressingMode::Absolute);
}

pub fn jmp_in(cpu: &mut CPU) {
    jmp(cpu, AddressingMode::Indirect);
}

fn branch(cpu: &mut CPU, condition: fn(&CPU) -> bool) {
    let operand = cpu.access_memory(cpu.program_counter);
    cpu.increment_program_counter();
    if !condition(cpu) {
        return;
    }

    cpu.offset_program_counter(operand)
}

pub fn bcc(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return !cpu.processor_status.get_carry_flag();
    });
}

pub fn bcs(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return cpu.processor_status.get_carry_flag();
    });
}

pub fn bne(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return !cpu.processor_status.get_zero_flag();
    });
}

pub fn beq(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return cpu.processor_status.get_zero_flag();
    });
}

fn compare(cpu: &mut CPU, addr_mode: AddressingMode, register: Registers) {
    let value = match cpu.read_memory(addr_mode) {
        Some(value) => value,
        None => panic!("compare used with incorrect address mode"),
    };

    cpu.set_cmp_status(register, value);
}

pub fn cmp_im(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Immediate, Registers::Accumulator);
}

pub fn cmp_zp(cpu: &mut CPU) {
    compare(cpu, AddressingMode::ZeroPage, Registers::Accumulator);
}

pub fn cmp_zpx(cpu: &mut CPU) {
    compare(cpu, AddressingMode::ZeroPageX, Registers::Accumulator);
}

pub fn cmp_a(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Absolute, Registers::Accumulator);
}

pub fn cmp_ax(cpu: &mut CPU) {
    compare(cpu, AddressingMode::AbsoluteX, Registers::Accumulator);
}

pub fn cmp_ay(cpu: &mut CPU) {
    compare(cpu, AddressingMode::AbsoluteY, Registers::Accumulator);
}

pub fn cmp_inx(cpu: &mut CPU) {
    compare(cpu, AddressingMode::IndexIndirectX, Registers::Accumulator);
}

pub fn cmp_iny(cpu: &mut CPU) {
    compare(cpu, AddressingMode::IndirectIndexY, Registers::Accumulator);
}

pub fn cpx_im(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Immediate, Registers::IndexX);
}

pub fn cpx_zp(cpu: &mut CPU) {
    compare(cpu, AddressingMode::ZeroPage, Registers::IndexX);
}

pub fn cpx_a(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Absolute, Registers::IndexX);
}

pub fn cpy_im(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Immediate, Registers::IndexY);
}

pub fn cpy_zp(cpu: &mut CPU) {
    compare(cpu, AddressingMode::ZeroPage, Registers::IndexY);
}

pub fn cpy_a(cpu: &mut CPU) {
    compare(cpu, AddressingMode::Absolute, Registers::IndexY);
}

fn decrement_register(cpu: &mut CPU, register: Registers) {
    match register {
        Registers::IndexX | Registers::IndexY => {
            cpu.decrement_register(register);
        }
        _ => panic!("decrement_register used with incorrect register"),
    }
}

pub fn dec_zp(cpu: &mut CPU) {
    cpu.modify_memory(AddressingMode::ZeroPage, MemoryModifications::Decrement);
}

pub fn dec_zpx(cpu: &mut CPU) {
    cpu.modify_memory(AddressingMode::ZeroPageX, MemoryModifications::Decrement);
}

pub fn dec_a(cpu: &mut CPU) {
    cpu.modify_memory(AddressingMode::Absolute, MemoryModifications::Decrement);
}

pub fn dec_ax(cpu: &mut CPU) {
    cpu.modify_memory(AddressingMode::AbsoluteX, MemoryModifications::Decrement);
}

pub fn dex_im(cpu: &mut CPU) {
    decrement_register(cpu, Registers::IndexX);
}

pub fn dey_im(cpu: &mut CPU) {
    decrement_register(cpu, Registers::IndexY);
}

fn increment_register(cpu: &mut CPU, register: Registers) {
    match register {
        Registers::IndexX | Registers::IndexY => {
            cpu.increment_register(register);
        }
        _ => panic!("increment_register used with incorrect register"),
    }
}

pub fn inc_zp(cpu: &mut CPU) {
    cpu.modify_memory(AddressingMode::ZeroPage, MemoryModifications::Increment);
}

pub fn inc_zpx(cpu: &mut CPU) {
    cpu.modify_memory(AddressingMode::ZeroPageX, MemoryModifications::Increment);
}

pub fn inc_a(cpu: &mut CPU) {
    cpu.modify_memory(AddressingMode::Absolute, MemoryModifications::Increment);
}

pub fn inc_ax(cpu: &mut CPU) {
    cpu.modify_memory(AddressingMode::AbsoluteX, MemoryModifications::Increment);
}

pub fn inx_im(cpu: &mut CPU) {
    increment_register(cpu, Registers::IndexX);
}

pub fn iny_im(cpu: &mut CPU) {
    increment_register(cpu, Registers::IndexY);
}

pub fn store(cpu: &mut CPU, addr_mode: AddressingMode, register: Registers) {
    let value = cpu.get_register(register);
    match cpu.write_memory(addr_mode, value) {
        Some(()) => (),
        None => panic!("store_in_memory used with incorrect address mode"),
    }
}

pub fn sta_zp(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPage, Registers::Accumulator);
}

pub fn sta_zpx(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPageX, Registers::Accumulator);
}

pub fn sta_a(cpu: &mut CPU) {
    store(cpu, AddressingMode::Absolute, Registers::Accumulator);
}

pub fn sta_ax(cpu: &mut CPU) {
    store(cpu, AddressingMode::AbsoluteX, Registers::Accumulator);
}

pub fn sta_ay(cpu: &mut CPU) {
    store(cpu, AddressingMode::AbsoluteY, Registers::Accumulator);
}

pub fn sta_inx(cpu: &mut CPU) {
    store(cpu, AddressingMode::IndexIndirectX, Registers::Accumulator);
}

pub fn sta_iny(cpu: &mut CPU) {
    store(cpu, AddressingMode::IndirectIndexY, Registers::Accumulator);
}

pub fn stx_zp(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPage, Registers::IndexX);
}

pub fn stx_zpy(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPageY, Registers::IndexX);
}

pub fn stx_a(cpu: &mut CPU) {
    store(cpu, AddressingMode::Absolute, Registers::IndexX);
}

pub fn sty_zp(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPage, Registers::IndexY);
}

pub fn sty_zpx(cpu: &mut CPU) {
    store(cpu, AddressingMode::ZeroPageX, Registers::IndexY);
}

pub fn sty_a(cpu: &mut CPU) {
    store(cpu, AddressingMode::Absolute, Registers::IndexY);
}

pub fn ora(cpu: &mut CPU, addr_mode: AddressingMode) {
    let value = match cpu.read_memory(addr_mode) {
        Some(value) => value,
        None => panic!("ora used with incorrect addressing mode"),
    };

    let result_value = cpu.get_register(Registers::Accumulator) | value;

    cpu.set_register(Registers::Accumulator, result_value);
    cpu.set_status_of_register(Registers::Accumulator);
}

pub fn ora_im(cpu: &mut CPU) {
    ora(cpu, AddressingMode::Immediate);
}

pub fn ora_zp(cpu: &mut CPU) {
    ora(cpu, AddressingMode::ZeroPage);
}

pub fn ora_zpx(cpu: &mut CPU) {
    ora(cpu, AddressingMode::ZeroPageX);
}

pub fn ora_a(cpu: &mut CPU) {
    ora(cpu, AddressingMode::Absolute);
}

pub fn ora_ax(cpu: &mut CPU) {
    ora(cpu, AddressingMode::AbsoluteX);
}

pub fn ora_ay(cpu: &mut CPU) {
    ora(cpu, AddressingMode::AbsoluteY);
}

pub fn ora_inx(cpu: &mut CPU) {
    ora(cpu, AddressingMode::IndexIndirectX);
}

pub fn ora_iny(cpu: &mut CPU) {
    ora(cpu, AddressingMode::IndirectIndexY);
}

pub fn nop(cpu: &mut CPU) {
    cpu.increment_program_counter();
}

fn change_flag_value(cpu: &mut CPU, flag: Flags, value: bool) {
    cpu.processor_status.change_flag(flag, value);
    cpu.cycle += 1;
}

pub fn clc(cpu: &mut CPU) {
    change_flag_value(cpu, Flags::Carry, false);
}

pub fn cld(cpu: &mut CPU) {
    change_flag_value(cpu, Flags::DecimalMode, false);
}

pub fn cli(cpu: &mut CPU) {
    change_flag_value(cpu, Flags::InterruptDisable, false);
}

pub fn clv(cpu: &mut CPU) {
    change_flag_value(cpu, Flags::Overflow, false);
}

pub fn sec(cpu: &mut CPU) {
    change_flag_value(cpu, Flags::Carry, true);
}

pub fn sed(cpu: &mut CPU) {
    change_flag_value(cpu, Flags::DecimalMode, true);
}

pub fn sei(cpu: &mut CPU) {
    change_flag_value(cpu, Flags::InterruptDisable, true);
}

pub fn brk(cpu: &mut CPU) {
    cpu.access_memory(cpu.program_counter); // fetch and discard
    cpu.increment_program_counter();

    cpu.push_word_to_stack(cpu.program_counter);
    cpu.push_byte_to_stack(cpu.processor_status.into());
    cpu.program_counter = cpu.fetch_address_from(BRK_INTERRUPT_VECTOR);

    cpu.processor_status.change_break_flag(true);
}

#[cfg(test)]
mod tests;
