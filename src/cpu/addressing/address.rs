use crate::{
  consts::{Byte, Word},
  cpu::addressing::AddressingMode,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Address {
  indirect: Option<Word>,
  val: Option<Word>,
  pub mode: Option<AddressingMode>,
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

  pub fn reset_implicit(&mut self) {
    self.indirect = None;
    self.val = None;
    self.mode = Some(AddressingMode::Implicit);
    self.done = true;
  }

  pub fn reset_acc(&mut self) {
    self.indirect = None;
    self.val = None;
    self.mode = Some(AddressingMode::Accumulator);
    self.done = true;
  }
}
