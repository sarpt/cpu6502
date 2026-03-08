use std::{fmt::Display, ops::RangeInclusive};

use ringbuffer::{AllocRingBuffer, RingBuffer};

use crate::{
  consts::{Byte, DEFAULT_INSTRUCTION_HISTORY_CAPACITY, Word},
  cpu::{
    CPU,
    addressing::{AddressingMode, address::Address},
  },
  memory::Memory,
};

pub struct DebugInstructionInfo {
  pub addr: Word,
  pub opcode: Byte,
  pub name: &'static str,
  pub starting_cycle: usize,
  pub target_addr: Option<Address>,
  pub target_val: Option<Byte>,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Registers {
  pub a: Byte,
  pub x: Byte,
  pub y: Byte,
}

impl Debugger {
  pub fn new() -> Self {
    Debugger {
      instructions: AllocRingBuffer::new(DEFAULT_INSTRUCTION_HISTORY_CAPACITY),
      traps: Vec::new(),
    }
  }

  pub fn probe(&mut self, cpu: &CPU, memory: &dyn Memory) -> (Vec<ProbeResult>, Registers) {
    let mut results: Vec<ProbeResult> = Vec::new();
    let registers = Registers {
      a: cpu.accumulator,
      x: cpu.index_register_x,
      y: cpu.index_register_y,
    };

    if cpu.sync()
      && let Some(instruction) = &cpu.current_instruction
    {
      self.instructions.push(DebugInstructionInfo {
        addr: instruction.addr,
        opcode: instruction.opcode,
        name: instruction.name,
        starting_cycle: instruction.starting_cycle,
        target_addr: None,
        target_val: None,
      });
      results.push(ProbeResult::NextInstruction);
    }

    let Some(last_instruction) = &mut self.instructions.back_mut() else {
      return (results, registers);
    };

    let target_addr = cpu.addr;
    let addressing_done = last_instruction.target_addr.is_none() && cpu.addr.done;
    if addressing_done {
      if let Some(addr) = target_addr.value() {
        last_instruction.target_val = Some(memory[addr])
      }

      last_instruction.target_addr = Some(target_addr);
      results.push(ProbeResult::AddressingDone);
    }

    for trap in self.traps.iter() {
      match trap {
        TrapConditions::AddressRange(rng) => {
          if addressing_done
            && let Some(target_addr_val) = target_addr.value()
            && rng.contains(&target_addr_val)
          {
            results.push(ProbeResult::TrapHit(Traps::AddressRange(
              rng.clone(),
              target_addr_val,
            )));
          }
        }
      }
    }

    (results, registers)
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
    let target_addr = self
      .target_addr
      .and_then(|target_addr| {
        let mode = target_addr.mode?;

        match mode {
          AddressingMode::Implicit => Some(String::new()),
          AddressingMode::Immediate => self.target_val.map(|tgt| format!("#{tgt}")),
          AddressingMode::Relative => target_addr
            .indirect()
            .map(|addr_val| format!("*+{addr_val:X}")),
          AddressingMode::Indirect => target_addr
            .indirect()
            .map(|addr_val| format!("(${addr_val:X})")),
          AddressingMode::ZeroPage => target_addr.value().map(|addr_val| format!("${addr_val:X}")),
          AddressingMode::ZeroPageX => target_addr
            .value()
            .map(|addr_val| format!("${addr_val:X},X")),
          AddressingMode::ZeroPageY => target_addr
            .value()
            .map(|addr_val| format!("${addr_val:X},Y")),
          AddressingMode::Absolute => target_addr.value().map(|addr_val| format!("${addr_val:X}")),
          AddressingMode::AbsoluteX => target_addr
            .value()
            .map(|addr_val| format!("${addr_val:X},X")),
          AddressingMode::AbsoluteY => target_addr
            .value()
            .map(|addr_val| format!("${addr_val:X},Y")),
          AddressingMode::IndexIndirectX => target_addr
            .indirect()
            .map(|addr_val| format!("(${addr_val:X},X)")),
          AddressingMode::IndirectIndexY => target_addr
            .indirect()
            .map(|addr_val| format!("(${addr_val:X}),Y")),
          AddressingMode::Accumulator => Some(String::from("A")),
        }
      })
      .unwrap_or_else(|| String::from("?"));

    write!(
      f,
      "{}@{:#04X}: {} {}",
      self.starting_cycle, self.addr, self.name, target_addr
    )
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
      assert_eq!(instruction_info, "1@0x00: NOP ");

      cpu.tick(&mut memory);
      uut.probe(&cpu, &memory);
      last_instruction = uut
        .get_last_instruction()
        .expect("last instruction is unexpectedly None");
      instruction_info = format!("{}", last_instruction);
      // second cycle of NOP
      assert_eq!(instruction_info, "1@0x00: NOP ");

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
      debugger::{Debugger, ProbeResult, Registers},
      instructions::{LDA_A, LDA_IM, LDX_A, LDX_IM, LDY_A, LDY_IM, NOP},
      tests::MemoryMock,
    };

    #[test]
    fn should_return_addresing_done_on_first_cycle_when_addressing_is_done() {
      let mut memory = MemoryMock::new(&[LDA_A, 0x04, 0x00, NOP, 0x56]);
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.addr = Address::new();

      let mut uut = Debugger::new();

      // fetch of LDA_A
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(&result.0, &[ProbeResult::NextInstruction]);
      assert_eq!(
        result.1,
        Registers {
          a: 0x0,
          x: 0x0,
          y: 0x0
        }
      );

      // tick to fetch lo address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(&result.0, &[]);
      assert_eq!(
        result.1,
        Registers {
          a: 0x0,
          x: 0x0,
          y: 0x0
        }
      );

      // tick to fetch hi address & addressing done
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(&result.0, &[ProbeResult::AddressingDone]);
      assert_eq!(
        result.1,
        Registers {
          a: 0x0,
          x: 0x0,
          y: 0x0
        }
      );

      // fetch of value at address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(&result.0, &[]);
      assert_eq!(
        result.1,
        Registers {
          a: 0x56,
          x: 0x0,
          y: 0x0
        }
      );

      // fetch of NOP
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(
        &result.0,
        &[ProbeResult::NextInstruction, ProbeResult::AddressingDone]
      );
      assert_eq!(
        result.1,
        Registers {
          a: 0x56,
          x: 0x0,
          y: 0x0
        }
      );
    }

    #[test]
    fn should_return_addressing_ranges_when_traps_hit_after_last_cycle_of_addressing() {
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
      assert_eq!(&result.0, &[ProbeResult::NextInstruction]);
      assert_eq!(
        result.1,
        Registers {
          a: 0x0,
          x: 0x0,
          y: 0x0
        }
      );

      // fetch lo of address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(&result.0, &[]);
      assert_eq!(
        result.1,
        Registers {
          a: 0x0,
          x: 0x0,
          y: 0x0
        }
      );

      // fetch hi of address & addressing done
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(
        &result.0,
        &[
          ProbeResult::AddressingDone,
          ProbeResult::TrapHit(crate::cpu::debugger::Traps::AddressRange(0x04..=0x04, 0x04))
        ]
      );
      assert_eq!(
        result.1,
        Registers {
          a: 0x0,
          x: 0x0,
          y: 0x0
        }
      );

      // fetch of value at address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(&result.0, &[]);
      assert_eq!(
        result.1,
        Registers {
          a: 0x7,
          x: 0x0,
          y: 0x0
        }
      );

      // fetch of next LDX_A
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(&result.0, &[ProbeResult::NextInstruction]);
      assert_eq!(
        result.1,
        Registers {
          a: 0x7,
          x: 0x0,
          y: 0x0
        }
      );

      // fetch lo of address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(&result.0, &[]);
      assert_eq!(
        result.1,
        Registers {
          a: 0x7,
          x: 0x0,
          y: 0x0
        }
      );

      // fetch hi of address & addressing done
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(
        &result.0,
        &[
          ProbeResult::AddressingDone,
          ProbeResult::TrapHit(crate::cpu::debugger::Traps::AddressRange(0x07..=0x07, 0x07))
        ]
      );
      assert_eq!(
        result.1,
        Registers {
          a: 0x7,
          x: 0x0,
          y: 0x0
        }
      );

      // fetch of value at address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(&result.0, &[]);
      assert_eq!(
        result.1,
        Registers {
          a: 0x7,
          x: 0x1,
          y: 0x0
        }
      );

      // fetch of next LDY_A
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(&result.0, &[ProbeResult::NextInstruction]);
      assert_eq!(
        result.1,
        Registers {
          a: 0x7,
          x: 0x1,
          y: 0x0
        }
      );

      // fetch lo of address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(&result.0, &[]);
      assert_eq!(
        result.1,
        Registers {
          a: 0x7,
          x: 0x1,
          y: 0x0
        }
      );

      // fetch hi of address & addressing done
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(
        &result.0,
        &[
          ProbeResult::AddressingDone,
          ProbeResult::TrapHit(crate::cpu::debugger::Traps::AddressRange(0x1..=0x1, 0x1))
        ]
      );
      assert_eq!(
        result.1,
        Registers {
          a: 0x7,
          x: 0x1,
          y: 0x0
        }
      );

      // fetch of value at address
      cpu.tick(&mut memory);
      let result = uut.probe(&cpu, &memory);
      assert_eq!(&result.0, &[]);
      assert_eq!(
        result.1,
        Registers {
          a: 0x7,
          x: 0x1,
          y: 0x4
        }
      );
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
  mod debug_instruction_info {

    #[cfg(test)]
    mod display {
      use crate::cpu::{
        addressing::{AddressingMode, address::Address},
        debugger::DebugInstructionInfo,
      };

      #[test]
      fn should_show_absolute_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::Absolute);
        addr.set(0x5955u16);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          opcode: 0xAD,
          name: "LDA",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: LDA $5955");
      }

      #[test]
      fn should_show_absolute_x_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::AbsoluteX);
        addr.set_lo(0x59u8);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          opcode: 0xBD,
          name: "LDA",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: LDA $59,X");
      }

      #[test]
      fn should_show_absolute_y_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::AbsoluteY);
        addr.set_lo(0x59u8);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          opcode: 0xB9,
          name: "LDA",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: LDA $59,Y");
      }

      #[test]
      fn should_show_accumulator_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::Accumulator);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          opcode: 0x4A,
          name: "LSR",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: LSR A");
      }

      #[test]
      fn should_show_immediate_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::Immediate);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          opcode: 0xA0,
          name: "LDY",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: Some(89),
        };

        assert_eq!(uut.to_string(), "3@0x21: LDY #89");
      }

      #[test]
      fn should_show_implicit_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::Implicit);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          opcode: 0xEA,
          name: "NOP",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: NOP ");
      }

      #[test]
      fn should_show_indirect_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::Indirect);
        addr.set_indirect_lo(0x59);
        addr.set_indirect_hi(0x25);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          opcode: 0x6C,
          name: "JMP",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: JMP ($2559)");
      }

      #[test]
      fn should_show_index_indirect_x_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::IndexIndirectX);
        addr.set_indirect_lo(0x59);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          opcode: 0xA1,
          name: "LDA",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: LDA ($59,X)");
      }

      #[test]
      fn should_show_indirect_index_y_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::IndirectIndexY);
        addr.set_indirect_lo(0x59);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          opcode: 0xB1,
          name: "LDA",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: LDA ($59),Y");
      }

      #[test]
      fn should_show_relative_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::Relative);
        addr.set_indirect_lo(0x4);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          opcode: 0x30,
          name: "BMI",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: BMI *+4");
      }

      #[test]
      fn should_show_zero_page_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::ZeroPage);
        addr.set_lo(0x59u8);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          opcode: 0xA5,
          name: "LDA",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: LDA $59");
      }

      #[test]
      fn should_show_zero_page_x_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::ZeroPageX);
        addr.set_lo(0x59u8);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          opcode: 0xB5,
          name: "LDA",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: LDA $59,X");
      }

      #[test]
      fn should_show_zero_page_y_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::ZeroPageY);
        addr.set_lo(0x59u8);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          opcode: 0xB6,
          name: "LDX",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: LDX $59,Y");
      }
    }
  }
}
