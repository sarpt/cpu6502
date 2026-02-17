use std::fmt::Display;

use crate::{
  consts::{Byte, Word},
  cpu::addressing::AddressingMode,
};

#[derive(Default, Clone, Copy)]
pub struct Address {
  indirect: Option<Word>,
  val: Option<Word>,
  mode: Option<AddressingMode>,
  pub done: bool,
}

impl Address {
  pub fn new() -> Self {
    Address {
      indirect: None,
      val: None,
      mode: None,
      done: false,
    }
  }

  pub fn value(&self) -> Option<Word> {
    self.val
  }

  pub fn indirect(&self) -> Option<Word> {
    self.indirect
  }

  pub fn set<T: Into<Word>>(&mut self, val: T) {
    self.val = Some(val.into());
  }

  pub fn set_indirect_lo(&mut self, lo: Byte) {
    let hi: Byte = match self.indirect {
      Some(val) => val.to_le_bytes()[1],
      None => 0,
    };

    self.indirect = Some(Word::from_le_bytes([lo, hi]));
  }

  pub fn set_indirect_hi(&mut self, hi: Byte) {
    let lo: Byte = match self.indirect {
      Some(val) => val.to_le_bytes()[0],
      None => 0,
    };

    self.indirect = Some(Word::from_le_bytes([lo, hi]));
  }

  pub fn set_lo(&mut self, lo: Byte) {
    let hi: Byte = match self.val {
      Some(val) => val.to_le_bytes()[1],
      None => 0,
    };

    self.val = Some(Word::from_le_bytes([lo, hi]));
  }

  pub fn set_hi(&mut self, hi: Byte) {
    let lo: Byte = match self.val {
      Some(val) => val.to_le_bytes()[0],
      None => 0,
    };

    self.val = Some(Word::from_le_bytes([lo, hi]));
  }

  pub fn reset(&mut self, mode: AddressingMode) {
    self.indirect = None;
    self.val = None;
    self.mode = Some(mode);
    self.done = false;
  }
}

impl Display for Address {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let val = self
      .mode
      .and_then(|mode| match mode {
        AddressingMode::Immediate => Some(String::from("")),
        AddressingMode::Indirect => self.indirect.map(|addr_val| format!("(${addr_val:#04X})")),
        AddressingMode::ZeroPage => self.val.map(|addr_val| format!("${addr_val:#02X}")),
        AddressingMode::ZeroPageX => self.val.map(|addr_val| format!("${addr_val:#02X},X")),
        AddressingMode::ZeroPageY => self.val.map(|addr_val| format!("${addr_val:#02X},Y")),
        AddressingMode::Absolute => self.val.map(|addr_val| format!("${addr_val:#04X}")),
        AddressingMode::AbsoluteX => self.val.map(|addr_val| format!("${addr_val:#04X},X")),
        AddressingMode::AbsoluteY => self.val.map(|addr_val| format!("${addr_val:#04X},Y")),
        AddressingMode::IndexIndirectX => self
          .indirect
          .map(|addr_val| format!("(${addr_val:#02X},X)")),
        AddressingMode::IndirectIndexY => self
          .indirect
          .map(|addr_val| format!("(${addr_val:#02X}),Y")),
        AddressingMode::Accumulator => Some(String::from("A")),
      })
      .unwrap_or_else(|| String::from("?"));
    write!(f, "{val}")
  }
}
