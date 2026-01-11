use std::fmt::Display;

use ringbuffer::{AllocRingBuffer, RingBuffer};

use crate::{
  consts::{Byte, DEFAULT_INSTRUCTION_HISTORY_CAPACITY, Word},
  cpu::{CPU, addressing::address::Address},
};

pub struct DebugInstructionInfo {
  pub addr: Word,
  pub opcode: Byte,
  pub starting_cycle: usize,
  pub target_addr: Option<Address>,
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
    if cpu.sync()
      && let Some(instruction) = &cpu.current_instruction
    {
      self.instructions.push(DebugInstructionInfo {
        addr: instruction.addr,
        opcode: instruction.opcode,
        starting_cycle: instruction.starting_cycle,
        target_addr: None,
      })
    }

    let last_instruction = self.instructions.back_mut();
    if let Some(inst) = last_instruction
      && cpu.addr.value().is_some()
    {
      inst.target_addr = Some(cpu.addr);
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
      "{}@{:#04X}: {:#04X} [{}]",
      self.starting_cycle,
      self.addr,
      self.opcode,
      self
        .target_addr
        .map_or(String::from("?"), |addr| addr.to_string())
    )
  }
}

#[cfg(test)]
mod tests {

  #[cfg(test)]
  mod get_last_instruction {
    use crate::cpu::{
      CPU,
      addressing::{AddressingMode, address::Address},
      debugger::Debugger,
      instructions::{LDA_IM, NOP},
      tests::MemoryMock,
    };

    #[test]
    fn should_return_last_ran_instruction_and_update_its_target_address() {
      let mut memory = MemoryMock::new(&[NOP, LDA_IM, 0xFF]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.addr = Address::new();

      let mut uut = Debugger::new();

      cpu.tick(&mut memory);
      uut.probe(&cpu);

      let mut last_instruction = uut
        .get_last_instruction()
        .expect("last instruction is unexpectedly None");
      let mut instruction_info = format!("{}", last_instruction);
      assert_eq!(instruction_info, "1@0x00: 0xEA [?]");

      cpu.tick(&mut memory);
      uut.probe(&cpu);
      last_instruction = uut
        .get_last_instruction()
        .expect("last instruction is unexpectedly None");
      instruction_info = format!("{}", last_instruction);
      // second cycle of NOP
      assert_eq!(instruction_info, "1@0x00: 0xEA [?]");

      cpu.tick(&mut memory);
      uut.probe(&cpu);

      last_instruction = uut
        .get_last_instruction()
        .expect("last instruction is unexpectedly None");
      instruction_info = format!("{}", last_instruction);
      assert_eq!(instruction_info, "3@0x01: 0xA9 [?]");

      cpu.tick(&mut memory);
      cpu.addr.reset(AddressingMode::Immediate);
      cpu.addr.set(0x02u8);
      uut.probe(&cpu);

      last_instruction = uut
        .get_last_instruction()
        .expect("last instruction is unexpectedly None");
      instruction_info = format!("{}", last_instruction);
      assert_eq!(instruction_info, "3@0x01: 0xA9 [IM->0x02]");
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
