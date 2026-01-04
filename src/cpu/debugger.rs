use std::fmt::Display;

use ringbuffer::{AllocRingBuffer, RingBuffer};

use crate::{
    consts::{Byte, Word, DEFAULT_INSTRUCTION_HISTORY_CAPACITY},
    cpu::CPU,
};

pub struct DebugInstructionInfo {
    pub addr: Word,
    pub opcode: Byte,
    pub starting_cycle: usize,
}

pub struct Debugger {
    instructions: AllocRingBuffer<DebugInstructionInfo>,
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            instructions: AllocRingBuffer::new(DEFAULT_INSTRUCTION_HISTORY_CAPACITY),
        }
    }

    pub fn probe(&mut self, cpu: &CPU) {
        if !cpu.sync() {
            return;
        }

        match &cpu.current_instruction {
            Some(instruction) => self.instructions.push(DebugInstructionInfo {
                addr: instruction.addr,
                opcode: instruction.opcode,
                starting_cycle: instruction.starting_cycle,
            }),
            None => todo!(),
        }
    }

    pub fn get_last_instruction(&self) -> Option<&DebugInstructionInfo> {
        self.instructions.back()
    }
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for DebugInstructionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}@{:#04X}: {:#04X}",
            self.starting_cycle, self.addr, self.opcode
        )
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod get_last_instruction {
        use crate::cpu::{
            debugger::Debugger,
            instructions::{LDA_IM, NOP},
            tests::MemoryMock,
            CPU,
        };

        #[test]
        fn should_return_last_ran_instruction() {
            let mut memory = MemoryMock::new(&[NOP, LDA_IM, 0xFF]);
            let mut cpu = CPU::new_nmos();
            cpu.program_counter = 0x00;

            let mut uut = Debugger::new();

            cpu.tick(&mut memory);
            uut.probe(&cpu);

            let mut last_instruction = uut
                .get_last_instruction()
                .expect("last instruction is unexpectedly None");
            let mut instruction_info = format!("{}", last_instruction);
            assert_eq!(instruction_info, "1@0x00: 0xEA");

            cpu.tick(&mut memory);
            uut.probe(&cpu);
            cpu.tick(&mut memory);
            uut.probe(&cpu);

            last_instruction = uut
                .get_last_instruction()
                .expect("last instruction is unexpectedly None");
            instruction_info = format!("{}", last_instruction);
            assert_eq!(instruction_info, "3@0x01: 0xA9");
        }

        #[test]
        fn should_return_none_when_no_instructions_were_ran_yet() {
            let cpu = CPU::new_nmos();
            let mut uut = Debugger::new();

            uut.probe(&cpu);

            assert!(uut.get_last_instruction().is_none());
        }
    }
}
