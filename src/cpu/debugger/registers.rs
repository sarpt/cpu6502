use std::fmt::Display;

use crate::consts::Byte;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Registers {
  pub a: Byte,
  pub x: Byte,
  pub y: Byte,
}

impl Display for Registers {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(
      f,
      "A: {} / ${:02X}; X: {} / ${:02X}; Y: {} / ${:02X}",
      self.a, self.a, self.x, self.x, self.y, self.y
    )
  }
}

#[cfg(test)]
mod tests {

  #[cfg(test)]
  mod display {
    use crate::cpu::debugger::registers::Registers;

    #[test]
    fn should_display_registers_information() {
      let uut = Registers {
        a: 0x59,
        x: 0xFD,
        y: 0xA0,
      };

      assert_eq!(format!("{uut}"), "A: 89 / $59; X: 253 / $FD; Y: 160 / $A0\n".to_owned());
    }
  }
}

