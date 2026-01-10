use std::fmt::Display;

use crate::{
  consts::{Byte, Word},
  cpu::addressing::AddressingMode,
};

#[derive(Default, Clone, Copy)]
pub struct Address {
  val: Option<Word>,
  mode: Option<AddressingMode>,
}

impl Address {
  pub fn new() -> Self {
    Address {
      val: None,
      mode: None,
    }
  }

  pub fn value(&self) -> Option<Word> {
    self.val
  }

  pub fn set<T: Into<Word>>(&mut self, val: T) {
    self.val = Some(val.into());
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
    self.val = Some(0u16);
    self.mode = Some(mode);
  }
}

impl Display for Address {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}->{:#04X}",
      self
        .mode
        .map_or(String::from("None"), |mode| mode.to_string()),
      self.val.unwrap_or(0)
    )
  }
}
