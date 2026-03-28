use std::fmt::Display;

use crate::{
  consts::{Byte, Word},
  cpu::addressing::{AddressingMode, address::Address},
};

pub struct DebugInstructionInfo {
  pub addr: Word,
  pub addr_symbol: Option<String>,
  pub opcode: Byte,
  pub name: &'static str,
  pub starting_cycle: usize,
  pub target_addr: Option<Address>,
  pub target_val: Option<Byte>,
  pub target_symbol: Option<String>,
}

impl Display for DebugInstructionInfo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let instruction_address = self
      .addr_symbol
      .clone()
      .unwrap_or(format!("{:#04X}", self.addr));
    macro_rules! display_debug_info {
      ($addr_format:literal, $target_addr:ident) => {
        write!(
          f,
          concat!("{}@{}: {} ", $addr_format),
          self.starting_cycle, instruction_address, self.name, $target_addr
        )
      };
      ($target_addr:literal) => {
        write!(
          f,
          "{}@{}: {} {}",
          self.starting_cycle, instruction_address, self.name, $target_addr
        )
      };
      () => {
        write!(
          f,
          "{}@{}: {}",
          self.starting_cycle, instruction_address, self.name
        )
      };
    }

    macro_rules! display_option_debug_info {
      ($option:expr, $fmt: literal) => {
        match $option {
          Some(tgt) => display_debug_info!($fmt, tgt),
          None => display_debug_info!("?"),
        }
      };
    }

    match &self.target_symbol {
      Some(target_symbol) => display_debug_info!("{}", target_symbol),
      None => {
        let Some(target_addr) = self.target_addr else {
          return display_debug_info!("?");
        };

        let Some(mode) = target_addr.mode else {
          return display_debug_info!("?");
        };

        match mode {
          AddressingMode::Implicit => display_debug_info!(),
          AddressingMode::Immediate => display_option_debug_info!(self.target_val, "#{}"),
          AddressingMode::Relative => {
            display_option_debug_info!(target_addr.indirect().map(|v| v as i8), "*{:+}")
          }
          AddressingMode::Indirect => {
            display_option_debug_info!(target_addr.indirect(), "(${:X})")
          }
          AddressingMode::ZeroPage => {
            display_option_debug_info!(target_addr.value(), "${:X}")
          }
          AddressingMode::ZeroPageX => {
            display_option_debug_info!(target_addr.value(), "${:X},X")
          }
          AddressingMode::ZeroPageY => {
            display_option_debug_info!(target_addr.value(), "${:X},Y")
          }
          AddressingMode::Absolute => {
            display_option_debug_info!(target_addr.value(), "${:X}")
          }
          AddressingMode::AbsoluteX => {
            display_option_debug_info!(target_addr.indirect(), "${:X},X")
          }
          AddressingMode::AbsoluteY => {
            display_option_debug_info!(target_addr.indirect(), "${:X},Y")
          }
          AddressingMode::IndexIndirectX => {
            display_option_debug_info!(target_addr.indirect(), "(${:X},X)")
          }
          AddressingMode::IndirectIndexY => {
            display_option_debug_info!(target_addr.indirect(), "(${:X}),Y")
          }
          AddressingMode::Accumulator => display_debug_info!("A"),
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
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
          addr_symbol: None,
          opcode: 0xAD,
          name: "LDA",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
          target_symbol: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: LDA $5955");
      }

      #[test]
      fn should_show_absolute_x_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::AbsoluteX);
        addr.set_indirect_lo(0x59u8);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          addr_symbol: None,
          opcode: 0xBD,
          name: "LDA",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
          target_symbol: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: LDA $59,X");
      }

      #[test]
      fn should_show_absolute_y_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::AbsoluteY);
        addr.set_indirect_lo(0x59u8);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          addr_symbol: None,
          opcode: 0xB9,
          name: "LDA",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
          target_symbol: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: LDA $59,Y");
      }

      #[test]
      fn should_show_accumulator_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::Accumulator);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          addr_symbol: None,
          opcode: 0x4A,
          name: "LSR",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
          target_symbol: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: LSR A");
      }

      #[test]
      fn should_show_immediate_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::Immediate);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          addr_symbol: None,
          opcode: 0xA0,
          name: "LDY",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: Some(89),
          target_symbol: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: LDY #89");
      }

      #[test]
      fn should_show_implicit_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::Implicit);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          addr_symbol: None,
          opcode: 0xEA,
          name: "NOP",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
          target_symbol: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: NOP");
      }

      #[test]
      fn should_show_indirect_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::Indirect);
        addr.set_indirect_lo(0x59);
        addr.set_indirect_hi(0x25);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          addr_symbol: None,
          opcode: 0x6C,
          name: "JMP",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
          target_symbol: None,
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
          addr_symbol: None,
          opcode: 0xA1,
          name: "LDA",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
          target_symbol: None,
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
          addr_symbol: None,
          opcode: 0xB1,
          name: "LDA",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
          target_symbol: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: LDA ($59),Y");
      }

      #[test]
      fn should_show_relative_address_instruction_when_offset_is_positive_() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::Relative);
        addr.set_indirect_lo(0x4);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          addr_symbol: None,
          opcode: 0x30,
          name: "BMI",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
          target_symbol: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: BMI *+4");
      }

      #[test]
      fn should_show_relative_address_instruction_when_offset_is_negative_() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::Relative);
        addr.set_indirect_lo(0xFD);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          addr_symbol: None,
          opcode: 0x30,
          name: "BMI",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
          target_symbol: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: BMI *-3");
      }

      #[test]
      fn should_show_zero_page_address_instruction() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::ZeroPage);
        addr.set_lo(0x59u8);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          addr_symbol: None,
          opcode: 0xA5,
          name: "LDA",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
          target_symbol: None,
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
          addr_symbol: None,
          opcode: 0xB5,
          name: "LDA",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
          target_symbol: None,
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
          addr_symbol: None,
          opcode: 0xB6,
          name: "LDX",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
          target_symbol: None,
        };

        assert_eq!(uut.to_string(), "3@0x21: LDX $59,Y");
      }

      #[test]
      fn should_show_target_symbol_instead_of_address_when_available() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::Absolute);
        addr.set(0x5955u16);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          addr_symbol: None,
          opcode: 0xAD,
          name: "LDA",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
          target_symbol: Some(String::from(".PEEK")),
        };

        assert_eq!(uut.to_string(), "3@0x21: LDA .PEEK");
      }

      #[test]
      fn should_show_instruction_symbol_instead_of_address_when_available() {
        let mut addr = Address::new();
        addr.reset(AddressingMode::Absolute);
        addr.set(0x5955u16);
        let uut = DebugInstructionInfo {
          addr: 0x21,
          addr_symbol: Some(String::from(".MONRD")),
          opcode: 0xAD,
          name: "LDA",
          starting_cycle: 3,
          target_addr: Some(addr),
          target_val: None,
          target_symbol: None,
        };

        assert_eq!(uut.to_string(), "3@.MONRD: LDA $5955");
      }
    }
  }
}
