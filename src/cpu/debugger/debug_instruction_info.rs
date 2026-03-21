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
            display_option_debug_info!(target_addr.value(), "${:X},X")
          }
          AddressingMode::AbsoluteY => {
            display_option_debug_info!(target_addr.value(), "${:X},Y")
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
