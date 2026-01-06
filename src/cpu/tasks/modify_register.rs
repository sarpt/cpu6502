use crate::{
  consts::Byte,
  cpu::{Registers, CPU},
  memory::Memory,
};

use super::Tasks;

enum ModificationVariant {
  Inc,
  Dec,
  ShiftLeft,
  ShiftRight,
  RotateLeft,
  RotateRight,
}

pub struct ModifyRegisterTasks {
  variant: ModificationVariant,
  register: Registers,
  value: Byte,
  done: bool,
}

impl ModifyRegisterTasks {
  pub fn new_inc(register: Registers) -> Self {
    ModifyRegisterTasks {
      done: false,
      register,
      value: Byte::default(),
      variant: ModificationVariant::Inc,
    }
  }

  pub fn new_dec(register: Registers) -> Self {
    ModifyRegisterTasks {
      done: false,
      register,
      value: Byte::default(),
      variant: ModificationVariant::Dec,
    }
  }

  pub fn new_shift_left(register: Registers) -> Self {
    ModifyRegisterTasks {
      done: false,
      register,
      value: Byte::default(),
      variant: ModificationVariant::ShiftLeft,
    }
  }

  pub fn new_shift_right(register: Registers) -> Self {
    ModifyRegisterTasks {
      done: false,
      register,
      value: Byte::default(),
      variant: ModificationVariant::ShiftRight,
    }
  }

  pub fn new_rotate_left(register: Registers) -> Self {
    ModifyRegisterTasks {
      done: false,
      register,
      value: Byte::default(),
      variant: ModificationVariant::RotateLeft,
    }
  }

  pub fn new_rotate_right(register: Registers) -> Self {
    ModifyRegisterTasks {
      done: false,
      register,
      value: Byte::default(),
      variant: ModificationVariant::RotateRight,
    }
  }
}

impl Tasks for ModifyRegisterTasks {
  fn done(&self) -> bool {
    self.done
  }

  fn tick(&mut self, cpu: &mut CPU, _: &mut dyn Memory) -> bool {
    if self.done() {
      panic!("tick mustn't be called when done")
    }

    let previous_value = cpu.get_register(self.register);
    self.value = previous_value;
    match self.variant {
      ModificationVariant::Inc => self.value = self.value.wrapping_add(1),
      ModificationVariant::Dec => self.value = self.value.wrapping_sub(1),
      ModificationVariant::ShiftLeft => {
        self.value <<= 1;
      }
      ModificationVariant::ShiftRight => {
        self.value >>= 1;
      }
      ModificationVariant::RotateLeft => {
        let mod_value = self.value << 1;
        if !cpu.processor_status.get_carry_flag() {
          self.value = mod_value;
        } else {
          self.value = mod_value | 0b00000001;
        }
      }
      ModificationVariant::RotateRight => {
        let mod_value = self.value >> 1;
        if !cpu.processor_status.get_carry_flag() {
          self.value = mod_value;
        } else {
          self.value = mod_value | 0b10000000;
        }
      }
    }

    cpu.set_register(self.register, self.value);
    match self.variant {
      ModificationVariant::ShiftLeft | ModificationVariant::RotateLeft => {
        cpu
          .processor_status
          .change_carry_flag(previous_value & 0b10000000 > 0);
      }
      ModificationVariant::ShiftRight | ModificationVariant::RotateRight => {
        cpu
          .processor_status
          .change_carry_flag(previous_value & 0b00000001 > 0);
      }
      _ => {}
    }

    self.done = true;
    self.done
  }
}
