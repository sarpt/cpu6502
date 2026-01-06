use crate::consts::Byte;

#[derive(Clone, Copy)]
pub enum Flags {
  Carry = 0,
  Zero = 1,
  InterruptDisable = 2,
  DecimalMode = 3,
  Break = 4,
  Overflow = 6,
  Negative = 7,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ProcessorStatus {
  flags: Byte,
}

impl PartialEq<Byte> for ProcessorStatus {
  fn eq(&self, other: &Byte) -> bool {
    self.flags == *other
  }
}

impl PartialEq for ProcessorStatus {
  fn eq(&self, other: &Self) -> bool {
    self.flags == other.flags
  }
}

impl Eq for ProcessorStatus {}

impl From<ProcessorStatus> for Byte {
  fn from(val: ProcessorStatus) -> Self {
    val.flags
  }
}

impl From<u8> for ProcessorStatus {
  fn from(value: u8) -> Self {
    ProcessorStatus { flags: value }
  }
}

impl ProcessorStatus {
  pub fn change_break_flag(&mut self, value_set: bool) {
    self.change_flag(Flags::Break, value_set);
  }

  pub fn get_break_flag(&self) -> bool {
    self.get_flag(Flags::Break)
  }

  pub fn change_carry_flag(&mut self, value_set: bool) {
    self.change_flag(Flags::Carry, value_set);
  }

  pub fn get_carry_flag(&self) -> bool {
    self.get_flag(Flags::Carry)
  }

  pub fn change_decimal_mode_flag(&mut self, value_set: bool) {
    self.change_flag(Flags::DecimalMode, value_set);
  }

  pub fn get_decimal_mode_flag(&self) -> bool {
    self.get_flag(Flags::DecimalMode)
  }

  pub fn change_interrupt_disable_flag(&mut self, value_set: bool) {
    self.change_flag(Flags::InterruptDisable, value_set);
  }

  pub fn get_interrupt_disable_flag(&self) -> bool {
    self.get_flag(Flags::InterruptDisable)
  }

  pub fn change_overflow_flag(&mut self, value_set: bool) {
    self.change_flag(Flags::Overflow, value_set);
  }

  pub fn get_overflow_flag(&self) -> bool {
    self.get_flag(Flags::Overflow)
  }

  pub fn change_negative_flag(&mut self, value_set: bool) {
    self.change_flag(Flags::Negative, value_set);
  }

  pub fn get_negative_flag(&self) -> bool {
    self.get_flag(Flags::Negative)
  }

  pub fn set(&mut self, val: Byte) {
    self.flags = val;
  }

  pub fn change_zero_flag(&mut self, value_set: bool) {
    self.change_flag(Flags::Zero, value_set);
  }

  pub fn get_zero_flag(&self) -> bool {
    self.get_flag(Flags::Zero)
  }

  pub fn change_flag(&mut self, flag: Flags, value_set: bool) {
    let shift: u8 = flag as u8;
    if value_set {
      self.flags |= 1 << shift;
    } else {
      self.flags &= !(1 << shift);
    }
  }

  pub fn get_flag(&self, flag: Flags) -> bool {
    let shift: u8 = flag as u8;
    (self.flags & (1 << shift)) > 0
  }
}
