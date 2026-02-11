use std::{fmt::Display, ops::RangeInclusive};

use ringbuffer::{AllocRingBuffer, RingBuffer};

use crate::{
  consts::{Byte, DEFAULT_INSTRUCTION_HISTORY_CAPACITY, Word},
  cpu::{CPU, addressing::address::Address},
};

pub struct DebugInstructionInfo {
  pub addr: Word,
  pub opcode: Byte,
  pub name: &'static str,
  pub starting_cycle: usize,
  pub target_addr: Option<Address>,
}

#[derive(Debug, PartialEq)]
pub enum Traps {
  AddressRange(RangeInclusive<Word>, Word),
}

#[derive(Debug, PartialEq)]
pub enum TrapConditions {
  AddressRange(RangeInclusive<Word>),
}

#[derive(Debug, PartialEq)]
pub enum ProbeResult {
  NextInstruction,
  AddressingDone,
  TrapHit(Traps),
}

pub struct Debugger {
  instructions: AllocRingBuffer<DebugInstructionInfo>,
  traps: Vec<TrapConditions>,
}

impl Debugger {
  pub fn new() -> Self {
    Debugger {
      instructions: AllocRingBuffer::new(DEFAULT_INSTRUCTION_HISTORY_CAPACITY),
      traps: Vec::new(),
    }
  }

  pub fn probe(&mut self, cpu: &CPU) -> Vec<ProbeResult> {
    let mut probe_results: Vec<ProbeResult> = Vec::new();

    if cpu.sync()
      && let Some(instruction) = &cpu.current_instruction
    {
      self.instructions.push(DebugInstructionInfo {
        addr: instruction.addr,
        opcode: instruction.opcode,
        name: instruction.name,
        starting_cycle: instruction.starting_cycle,
        target_addr: None,
      });
      probe_results.push(ProbeResult::NextInstruction);
    }

    let Some(last_instruction) = &mut self.instructions.back_mut() else {
      return probe_results;
    };

    let target_addr = cpu.addr;
    let addressing_done = last_instruction.target_addr.is_none() && cpu.addr.value().is_some();
    if addressing_done {
      last_instruction.target_addr = Some(target_addr);
      probe_results.push(ProbeResult::AddressingDone);
    }

    for trap in self.traps.iter() {
      match trap {
        TrapConditions::AddressRange(rng) => {
          if addressing_done
            && let Some(target_addr_val) = target_addr.value()
            && rng.contains(&target_addr_val)
          {
            probe_results.push(ProbeResult::TrapHit(Traps::AddressRange(
              rng.clone(),
              target_addr_val,
            )));
          }
        }
      }
    }

    probe_results
  }

  pub fn get_last_instruction(&self) -> Option<&DebugInstructionInfo> {
    self.instructions.back()
  }

  pub fn trap_between_addresses(&mut self, addrs: RangeInclusive<Word>) {
    self.traps.push(TrapConditions::AddressRange(addrs))
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
      "{}@{:#04X}: {} ({:#04X}) [{}]",
      self.starting_cycle,
      self.addr,
      self.name,
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
      assert_eq!(instruction_info, "1@0x00: NOP (0xEA) [?]");

      cpu.tick(&mut memory);
      uut.probe(&cpu);
      last_instruction = uut
        .get_last_instruction()
        .expect("last instruction is unexpectedly None");
      instruction_info = format!("{}", last_instruction);
      // second cycle of NOP
      assert_eq!(instruction_info, "1@0x00: NOP (0xEA) [?]");

      cpu.tick(&mut memory);
      uut.probe(&cpu);

      last_instruction = uut
        .get_last_instruction()
        .expect("last instruction is unexpectedly None");
      instruction_info = format!("{}", last_instruction);
      assert_eq!(instruction_info, "3@0x01: LDA_IM (0xA9) [?]");

      cpu.tick(&mut memory);
      cpu.addr.reset(AddressingMode::Immediate);
      cpu.addr.set(0x02u8);
      uut.probe(&cpu);

      last_instruction = uut
        .get_last_instruction()
        .expect("last instruction is unexpectedly None");
      instruction_info = format!("{}", last_instruction);
      assert_eq!(instruction_info, "3@0x01: LDA_IM (0xA9) [IM->0x02]");
    }

    #[test]
    fn should_return_none_when_no_instructions_were_ran_yet() {
      let cpu = CPU::new_nmos();
      let mut uut = Debugger::new();

      uut.probe(&cpu);

      assert!(uut.get_last_instruction().is_none());
    }
  }

  #[cfg(test)]
  mod probe {
    use crate::cpu::{
      CPU,
      addressing::address::Address,
      debugger::{Debugger, ProbeResult},
      instructions::{LDA_A, NOP},
      tests::MemoryMock,
    };

    #[test]
    fn should_return_addresing_done_on_first_cycle_when_address_is_known() {
      let mut memory = MemoryMock::new(&[LDA_A, 0x03, NOP, 0x56]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.addr = Address::new();

      let mut uut = Debugger::new();

      cpu.tick(&mut memory);
      let result = uut.probe(&cpu);
      assert_eq!(&result, &[ProbeResult::NextInstruction]);

      cpu.tick(&mut memory);
      let result = uut.probe(&cpu);
      assert_eq!(&result, &[ProbeResult::AddressingDone]);

      cpu.tick(&mut memory);
      let result = uut.probe(&cpu);
      assert_eq!(&result, &[]);

      cpu.tick(&mut memory);
      let result = uut.probe(&cpu);
      assert_eq!(&result, &[]);

      cpu.tick(&mut memory);
      let result = uut.probe(&cpu);
      assert_eq!(&result, &[ProbeResult::NextInstruction]);
    }

    #[test]
    fn should_return_addressing_ranges_when_traps_hit_after_last_cycle_of_addressing() {
      let mut memory = MemoryMock::new(&[LDA_A, 0x04, 0x00, LDA_A, 0x22, 0x00, LDA_A, 0x99, 0x00]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.addr = Address::new();

      let mut uut = Debugger::new();
      uut.trap_between_addresses(0x01..=0x04);
      uut.trap_between_addresses(0x80..=0xA0);

      cpu.tick(&mut memory);
      let result = uut.probe(&cpu);
      assert_eq!(&result, &[ProbeResult::NextInstruction]);

      cpu.tick(&mut memory);
      let result = uut.probe(&cpu);
      assert_eq!(
        &result,
        &[
          ProbeResult::AddressingDone,
          ProbeResult::TrapHit(crate::cpu::debugger::Traps::AddressRange(0x01..=0x04, 0x04))
        ]
      );

      cpu.tick(&mut memory);
      let result = uut.probe(&cpu);
      assert_eq!(&result, &[]);

      cpu.tick(&mut memory);
      let result = uut.probe(&cpu);
      assert_eq!(&result, &[]);

      cpu.tick(&mut memory);
      let result = uut.probe(&cpu);
      assert_eq!(&result, &[ProbeResult::NextInstruction]);

      cpu.tick(&mut memory);
      let result = uut.probe(&cpu);
      assert_eq!(&result, &[ProbeResult::AddressingDone]);

      cpu.tick(&mut memory);
      let result = uut.probe(&cpu);
      assert_eq!(&result, &[]);

      cpu.tick(&mut memory);
      let result = uut.probe(&cpu);
      assert_eq!(&result, &[]);

      cpu.tick(&mut memory);
      let result = uut.probe(&cpu);
      assert_eq!(&result, &[ProbeResult::NextInstruction]);

      cpu.tick(&mut memory);
      let result = uut.probe(&cpu);
      assert_eq!(
        &result,
        &[
          ProbeResult::AddressingDone,
          ProbeResult::TrapHit(crate::cpu::debugger::Traps::AddressRange(0x80..=0xA0, 0x99))
        ]
      );
    }
  }
}
