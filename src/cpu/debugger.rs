use std::ops::RangeInclusive;

use ringbuffer::{AllocRingBuffer, RingBuffer};

use crate::{
  consts::{Byte, DEFAULT_INSTRUCTION_HISTORY_CAPACITY, Word},
  cpu::{
    CPU,
    debugger::{debug_instruction_info::DebugInstructionInfo, registers::Registers},
    processor_status::ProcessorStatus,
  },
  memory::Memory,
};

pub mod debug_instruction_info;
pub mod registers;

#[derive(Debug, PartialEq)]
pub enum Traps {
  AddressRange(RangeInclusive<Word>, Word),
}

#[derive(Debug, PartialEq)]
pub enum TrapConditions {
  AddressRange(RangeInclusive<Word>),
}

#[derive(Debug, PartialEq)]
pub struct ProbeResult {
  pub events: Vec<ProbeEvent>,
  pub registers: Registers,
  pub processor_status: ProcessorStatus,
}

#[derive(Debug, PartialEq)]
pub enum ProbeEvent {
  NextInstruction,
  InstructionDone,
  AddressingDone,
  TrapHit(Traps),
  MemoryModification((Byte, Byte)),
}

pub trait Symbols {
  fn get(&self, addr: &Word) -> Option<String>;
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

  pub fn probe(&mut self, cpu: &CPU, memory: &dyn Memory) -> ProbeResult {
    let mut result = ProbeResult {
      events: Vec::new(),
      registers: Registers {
        a: cpu.accumulator,
        x: cpu.index_register_x,
        y: cpu.index_register_y,
      },
      processor_status: cpu.processor_status,
    };

    if cpu.current_instruction.is_none() && cpu.cycle > 0 {
      result.events.push(ProbeEvent::InstructionDone);
    } else if cpu.sync()
      && let Some(instruction) = &cpu.current_instruction
    {
      self.instructions.push(DebugInstructionInfo {
        addr: instruction.addr,
        addr_symbol: None,
        opcode: instruction.opcode,
        name: instruction.name,
        starting_cycle: instruction.starting_cycle,
        target_addr: None,
        target_val: None,
        target_symbol: None,
      });
      result.events.push(ProbeEvent::NextInstruction);
    }

    let Some(last_instruction) = &mut self.instructions.back_mut() else {
      return result;
    };

    let target_addr = cpu.addr;
    let addressing_done = last_instruction.target_addr.is_none() && cpu.addr.done;
    if addressing_done {
      if let Some(addr) = target_addr.value() {
        last_instruction.target_val = Some(memory[addr])
      }

      last_instruction.target_addr = Some(target_addr);
      result.events.push(ProbeEvent::AddressingDone);
    }

    for trap in self.traps.iter() {
      match trap {
        TrapConditions::AddressRange(rng) => {
          if addressing_done
            && let Some(target_addr_val) = target_addr.value()
            && rng.contains(&target_addr_val)
          {
            result.events.push(ProbeEvent::TrapHit(Traps::AddressRange(
              rng.clone(),
              target_addr_val,
            )));
          }
        }
      }
    }

    result
  }

  pub fn probe_with_symbols<S: Symbols>(
    &mut self,
    cpu: &CPU,
    memory: &dyn Memory,
    symbols: &S,
  ) -> ProbeResult {
    let result = self.probe(cpu, memory);
    let Some(last_instruction) = &mut self.instructions.back_mut() else {
      return result;
    };

    if result.events.contains(&ProbeEvent::NextInstruction) {
      last_instruction.addr_symbol = symbols.get(&last_instruction.addr);
    }

    if result.events.contains(&ProbeEvent::AddressingDone) {
      last_instruction.target_symbol = last_instruction
        .target_addr
        .and_then(|addr| addr.value())
        .and_then(|addr| symbols.get(&addr));
    }

    result
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

#[cfg(test)]
mod tests {

  #[cfg(test)]
  mod get_last_instruction {
    use crate::cpu::{
      CPU,
      addressing::address::Address,
      debugger::Debugger,
      instructions::{LDA_A, NOP},
      tests::MemoryMock,
    };

    #[test]
    fn should_return_last_ran_instruction_and_update_its_target_address() {
      let mut memory = MemoryMock::new(&[NOP, LDA_A, 0x04, 0x00, 0x99]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.addr = Address::new();

      let mut uut = Debugger::new();

      cpu.tick(&mut memory);
      uut.probe(&cpu, &memory);

      let mut last_instruction = uut
        .get_last_instruction()
        .expect("last instruction is unexpectedly None");
      let mut instruction_info = format!("{}", last_instruction);
      assert_eq!(instruction_info, "1@0x00: NOP");

      cpu.tick(&mut memory);
      uut.probe(&cpu, &memory);
      last_instruction = uut
        .get_last_instruction()
        .expect("last instruction is unexpectedly None");
      instruction_info = format!("{}", last_instruction);
      // second cycle of NOP
      assert_eq!(instruction_info, "1@0x00: NOP");

      cpu.tick(&mut memory);
      uut.probe(&cpu, &memory);

      last_instruction = uut
        .get_last_instruction()
        .expect("last instruction is unexpectedly None");
      instruction_info = format!("{}", last_instruction);
      assert_eq!(instruction_info, "3@0x01: LDA ?");

      cpu.tick(&mut memory);
      uut.probe(&cpu, &memory);

      last_instruction = uut
        .get_last_instruction()
        .expect("last instruction is unexpectedly None");
      instruction_info = format!("{}", last_instruction);
      assert_eq!(instruction_info, "3@0x01: LDA ?");

      cpu.tick(&mut memory);
      uut.probe(&cpu, &memory);

      last_instruction = uut
        .get_last_instruction()
        .expect("last instruction is unexpectedly None");
      instruction_info = format!("{}", last_instruction);
      assert_eq!(instruction_info, "3@0x01: LDA $4");
    }

    #[test]
    fn should_return_none_when_no_instructions_were_ran_yet() {
      let memory = MemoryMock::default();
      let cpu = CPU::new_nmos();
      let mut uut = Debugger::new();

      uut.probe(&cpu, &memory);

      assert!(uut.get_last_instruction().is_none());
    }
  }

  #[cfg(test)]
  mod probe {
    use crate::cpu::{
      CPU,
      addressing::address::Address,
      debugger::{Debugger, ProbeEvent, Registers},
      instructions::{LDA_A, LDA_IM, LDX_A, LDX_IM, LDY_A, LDY_IM, NOP},
      tests::MemoryMock,
    };

    #[test]
    fn should_return_instruction_done_on_last_cycle_of_instruction() {
      let mut memory = MemoryMock::new(&[LDA_A, 0x04, 0x00, NOP, 0x56]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.addr = Address::new();

      let mut uut = Debugger::new();

      // fetch of LDA_A
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(!result.events.contains(&ProbeEvent::InstructionDone));

      // tick to fetch lo address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(!result.events.contains(&ProbeEvent::InstructionDone));

      // tick to fetch hi address & addressing done
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(!result.events.contains(&ProbeEvent::InstructionDone));

      // fetch of value at address; end of instruction
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(result.events.contains(&ProbeEvent::InstructionDone));

      // fetch of NOP
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(!result.events.contains(&ProbeEvent::InstructionDone));

      // end of instruction
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(result.events.contains(&ProbeEvent::InstructionDone));
    }

    #[test]
    fn should_return_addresing_done_on_first_cycle_when_tgt_address_is_known() {
      let mut memory = MemoryMock::new(&[LDA_A, 0x04, 0x00, NOP, 0x56]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.addr = Address::new();

      let mut uut = Debugger::new();

      // fetch of LDA_A
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(!result.events.contains(&ProbeEvent::AddressingDone));

      // tick to fetch lo address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(!result.events.contains(&ProbeEvent::AddressingDone));

      // tick to fetch hi address & addressing done
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(result.events.contains(&ProbeEvent::AddressingDone));

      // fetch of value at address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(!result.events.contains(&ProbeEvent::AddressingDone));

      // fetch of NOP
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(result.events.contains(&ProbeEvent::AddressingDone));
    }

    #[test]
    fn should_return_addressing_ranges_when_traps_hit_after_last_cycle_of_addressing() {
      let none_of_traps_hit_check = |ev: &ProbeEvent| {
        *ev != ProbeEvent::TrapHit(crate::cpu::debugger::Traps::AddressRange(0x04..=0x04, 0x04))
          && *ev
            != ProbeEvent::TrapHit(crate::cpu::debugger::Traps::AddressRange(0x07..=0x07, 0x07))
          && *ev != ProbeEvent::TrapHit(crate::cpu::debugger::Traps::AddressRange(0x1..=0x1, 0x1))
      };

      let mut memory = MemoryMock::new(&[LDA_A, 0x04, 0x00, LDX_A, 0x07, 0x00, LDY_A, 0x01, 0x00]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.addr = Address::new();

      let mut uut = Debugger::new();
      uut.trap_between_addresses(0x0001..=0x0001);
      uut.trap_between_addresses(0x0004..=0x0004);
      uut.trap_between_addresses(0x0007..=0x0007);

      // fetch LDA_A
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(result.events.iter().all(none_of_traps_hit_check));

      // fetch lo of address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(result.events.iter().all(none_of_traps_hit_check));

      // fetch hi of address & addressing done
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(result.events.contains(&ProbeEvent::TrapHit(
        crate::cpu::debugger::Traps::AddressRange(0x04..=0x04, 0x04)
      )),);

      // fetch of value at address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(result.events.iter().all(none_of_traps_hit_check));

      // fetch of next LDX_A
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(result.events.iter().all(none_of_traps_hit_check));

      // fetch lo of address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(result.events.iter().all(none_of_traps_hit_check));

      // fetch hi of address & addressing done
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(result.events.contains(&ProbeEvent::TrapHit(
        crate::cpu::debugger::Traps::AddressRange(0x07..=0x07, 0x07)
      )),);

      // fetch of value at address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(result.events.iter().all(none_of_traps_hit_check));

      // fetch of next LDY_A
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(result.events.iter().all(none_of_traps_hit_check));

      // fetch lo of address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(result.events.iter().all(none_of_traps_hit_check));

      // fetch hi of address & addressing done
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(result.events.contains(&ProbeEvent::TrapHit(
        crate::cpu::debugger::Traps::AddressRange(0x1..=0x1, 0x1)
      )),);

      // fetch of value at address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert!(result.events.iter().all(none_of_traps_hit_check));
    }

    #[test]
    fn should_return_registers_and_processor_status() {
      let mut memory = MemoryMock::new(&[LDA_A, 0x04, 0x00, LDX_A, 0x00, 0x00, LDY_A, 0x01, 0x00]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.addr = Address::new();

      let mut uut = Debugger::new();

      // fetch LDA_A
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(
        result.registers,
        Registers {
          a: 0x0,
          x: 0x0,
          y: 0x0
        }
      );
      assert_eq!(result.processor_status, 0b00100000);

      // fetch lo of address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(
        result.registers,
        Registers {
          a: 0x0,
          x: 0x0,
          y: 0x0
        }
      );
      assert_eq!(result.processor_status, 0b00100000);

      // fetch hi of address & addressing done
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(
        result.registers,
        Registers {
          a: 0x0,
          x: 0x0,
          y: 0x0
        }
      );
      assert_eq!(result.processor_status, 0b00100000);

      // fetch of value at address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(
        result.registers,
        Registers {
          a: 0x0,
          x: 0x0,
          y: 0x0
        }
      );
      assert_eq!(result.processor_status, 0b00100010);

      // fetch of LDX_A
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(
        result.registers,
        Registers {
          a: 0x0,
          x: 0x0,
          y: 0x0
        }
      );
      assert_eq!(result.processor_status, 0b00100010);

      // fetch lo of address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(
        result.registers,
        Registers {
          a: 0x0,
          x: 0x0,
          y: 0x0
        }
      );
      assert_eq!(result.processor_status, 0b00100010);

      // fetch hi of address & addressing done
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(
        result.registers,
        Registers {
          a: 0x0,
          x: 0x0,
          y: 0x0
        }
      );
      assert_eq!(result.processor_status, 0b00100010);

      // fetch of value at address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(
        result.registers,
        Registers {
          a: 0x0,
          x: 0xAD,
          y: 0x0
        }
      );
      assert_eq!(result.processor_status, 0b10100000);

      // fetch of next LDY_A
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(
        result.registers,
        Registers {
          a: 0x0,
          x: 0xAD,
          y: 0x0
        }
      );
      assert_eq!(result.processor_status, 0b10100000);

      // fetch lo of address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(
        result.registers,
        Registers {
          a: 0x0,
          x: 0xAD,
          y: 0x0
        }
      );
      assert_eq!(result.processor_status, 0b10100000);

      // fetch hi of address & addressing done
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(
        result.registers,
        Registers {
          a: 0x0,
          x: 0xAD,
          y: 0x0
        }
      );
      assert_eq!(result.processor_status, 0b10100000);

      // fetch of value at address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(
        result.registers,
        Registers {
          a: 0x0,
          x: 0xAD,
          y: 0x4
        }
      );
      assert_eq!(result.processor_status, 0b00100000);
    }

    #[test]
    fn should_fill_target_val_of_last_instruction_when_addressing_is_immediate() {
      let mut memory = MemoryMock::new(&[LDA_IM, 0x04, LDX_IM, 0x07, LDY_IM, 0x01]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.addr = Address::new();
      let mut uut = Debugger::new();

      // fetch instruction LDA_IM
      cpu.tick(&mut memory);
      _ = uut.probe(&cpu, &memory);
      let last_instruction = uut
        .get_last_instruction()
        .expect("Could not get last instruction");
      assert_eq!(last_instruction.target_val, None);

      // fetch address, store value
      cpu.tick(&mut memory);
      _ = uut.probe(&cpu, &memory);
      let last_instruction = uut
        .get_last_instruction()
        .expect("Could not get last instruction");
      assert_eq!(last_instruction.target_val, Some(0x04));

      // fetch instruction LDX_IM
      cpu.tick(&mut memory);
      _ = uut.probe(&cpu, &memory);
      let last_instruction = uut
        .get_last_instruction()
        .expect("Could not get last instruction");
      assert_eq!(last_instruction.target_val, None);

      // fetch address, store value
      cpu.tick(&mut memory);
      _ = uut.probe(&cpu, &memory);
      let last_instruction = uut
        .get_last_instruction()
        .expect("Could not get last instruction");
      assert_eq!(last_instruction.target_val, Some(0x07));

      // fetch instruction LDY_IM
      cpu.tick(&mut memory);
      _ = uut.probe(&cpu, &memory);
      let last_instruction = uut
        .get_last_instruction()
        .expect("Could not get last instruction");
      assert_eq!(last_instruction.target_val, None);

      // fetch address, store value
      cpu.tick(&mut memory);
      _ = uut.probe(&cpu, &memory);
      let last_instruction = uut
        .get_last_instruction()
        .expect("Could not get last instruction");
      assert_eq!(last_instruction.target_val, Some(0x01));
    }
  }

  #[cfg(test)]
  mod probe_with_symbols {

    use crate::{
      consts::Word,
      cpu::{
        CPU,
        addressing::address::Address,
        debugger::{Debugger, Symbols},
        instructions::{LDA_A, LDA_IM, LDX_A, LDX_IM, LDY_IM},
        tests::MemoryMock,
      },
    };

    struct SymbolsMock {}
    impl Symbols for SymbolsMock {
      fn get(&self, addr: &Word) -> Option<String> {
        if *addr == 0x00 {
          Some(String::from(".START"))
        } else if *addr == 0x04 {
          Some(String::from(".END"))
        } else {
          None
        }
      }
    }

    #[test]
    fn should_fill_instruction_address_symbol_when_instruction_addr_can_be_matched_against_symbol_during_new_instruction_fetch()
     {
      let symbols_mock = SymbolsMock {};
      let mut memory = MemoryMock::new(&[LDA_IM, 0x04, LDX_IM, 0x07, LDY_IM, 0x01]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.addr = Address::new();
      let mut uut = Debugger::new();

      // fetch instruction LDA_IM
      cpu.tick(&mut memory);
      _ = uut.probe_with_symbols(&cpu, &memory, &symbols_mock);
      let mut last_instruction = uut
        .get_last_instruction()
        .expect("Could not get last instruction");
      assert_eq!(last_instruction.addr, 0x00);
      assert_eq!(last_instruction.addr_symbol, Some(String::from(".START")));

      // fetch address, store value
      cpu.tick(&mut memory);
      _ = uut.probe_with_symbols(&cpu, &memory, &symbols_mock);

      // fetch instruction LDX_IM
      cpu.tick(&mut memory);
      _ = uut.probe_with_symbols(&cpu, &memory, &symbols_mock);
      last_instruction = uut
        .get_last_instruction()
        .expect("Could not get last instruction");
      assert_eq!(last_instruction.addr, 0x02);
      assert_eq!(last_instruction.addr_symbol, None);

      // fetch address, store value
      cpu.tick(&mut memory);
      _ = uut.probe_with_symbols(&cpu, &memory, &symbols_mock);

      // fetch instruction LDY_IM
      cpu.tick(&mut memory);
      _ = uut.probe_with_symbols(&cpu, &memory, &symbols_mock);
      last_instruction = uut
        .get_last_instruction()
        .expect("Could not get last instruction");
      assert_eq!(last_instruction.addr, 0x04);
      assert_eq!(last_instruction.addr_symbol, Some(String::from(".END")));
    }

    #[test]
    fn should_fill_target_address_symbol_when_target_addr_can_be_matched_against_symbol_after_addressing_is_done()
     {
      let symbols_mock = SymbolsMock {};
      let mut memory = MemoryMock::new(&[LDA_A, 0x04, 0x00, LDX_A, 0x00, 0x00]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.addr = Address::new();
      let mut uut = Debugger::new();

      // fetch instruction LDA_A
      cpu.tick(&mut memory);
      _ = uut.probe_with_symbols(&cpu, &memory, &symbols_mock);

      // fetch address lo
      cpu.tick(&mut memory);
      _ = uut.probe_with_symbols(&cpu, &memory, &symbols_mock);

      // fetch address hi
      cpu.tick(&mut memory);
      _ = uut.probe_with_symbols(&cpu, &memory, &symbols_mock);
      let mut last_instruction = uut
        .get_last_instruction()
        .expect("Could not get last instruction");
      assert_eq!(last_instruction.target_addr.unwrap().value(), Some(0x04));
      assert_eq!(last_instruction.target_symbol, Some(String::from(".END")));

      // fetch value
      cpu.tick(&mut memory);
      _ = uut.probe_with_symbols(&cpu, &memory, &symbols_mock);

      // fetch instruction LDX_A
      cpu.tick(&mut memory);
      _ = uut.probe_with_symbols(&cpu, &memory, &symbols_mock);

      // fetch address lo
      cpu.tick(&mut memory);
      _ = uut.probe_with_symbols(&cpu, &memory, &symbols_mock);

      // fetch address hi
      cpu.tick(&mut memory);
      _ = uut.probe_with_symbols(&cpu, &memory, &symbols_mock);
      last_instruction = uut
        .get_last_instruction()
        .expect("Could not get last instruction");
      assert_eq!(last_instruction.target_addr.unwrap().value(), Some(0x00));
      assert_eq!(last_instruction.target_symbol, Some(String::from(".START")));
    }
  }
}
