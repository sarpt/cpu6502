use std::fs::OpenOptions;
use std::path::PathBuf;

use serde::de::Error;
use serde::{Deserialize, Deserializer};

use crate::consts::{Byte, Word};
use crate::cpu::CPU;
use crate::cpu::processor_status::ProcessorStatus;

use crate::memory::Generic64kMem;

const LEGAL_OPCODES: [u8; 151] = [
  0x69, 0x65, 0x75, 0x6D, 0x7D, 0x79, 0x61, 0x71, 0x29, 0x25, 0x35, 0x2D, 0x3D, 0x39, 0x21, 0x31,
  0x0A, 0x06, 0x16, 0x0E, 0x1E, 0x90, 0xB0, 0xF0, 0x24, 0x2C, 0x30, 0xD0, 0x10, 0x00, 0x50, 0x70,
  0x18, 0xD8, 0x58, 0xB8, 0xC9, 0xC5, 0xD5, 0xCD, 0xDD, 0xD9, 0xC1, 0xD1, 0xE0, 0xE4, 0xEC, 0xC0,
  0xC4, 0xCC, 0xCE, 0xDE, 0xC6, 0xD6, 0xCA, 0x88, 0x49, 0x45, 0x55, 0x4D, 0x5D, 0x59, 0x41, 0x51,
  0xE6, 0xF6, 0xEE, 0xFE, 0xE8, 0xC8, 0x4C, 0x6C, 0x20, 0xA9, 0xA5, 0xB5, 0xAD, 0xBD, 0xB9, 0xA1,
  0xB1, 0xA0, 0xA4, 0xB4, 0xAC, 0xBC, 0xA2, 0xA6, 0xB6, 0xAE, 0xBE, 0x4A, 0x46, 0x56, 0x4E, 0x5E,
  0xEA, 0x09, 0x05, 0x15, 0x0D, 0x1D, 0x19, 0x01, 0x11, 0x48, 0x08, 0x68, 0x28, 0x2A, 0x26, 0x36,
  0x2E, 0x3E, 0x6A, 0x66, 0x76, 0x6E, 0x7E, 0x40, 0x60, 0x85, 0x95, 0x8D, 0x9D, 0x99, 0x81, 0x91,
  0x86, 0x96, 0x8E, 0x84, 0x94, 0x8C, 0x38, 0xF8, 0x78, 0xE9, 0xE5, 0xF5, 0xED, 0xFD, 0xF9, 0xE1,
  0xF1, 0xAA, 0xA8, 0xBA, 0x8A, 0x9A, 0x98,
];

const DECIMAL_DEPENDENT_OPCODES: [u8; 16] = [
  0x69, 0x65, 0x75, 0x6D, 0x7D, 0x79, 0x61, 0x71, 0xE9, 0xE5, 0xF5, 0xED, 0xFD, 0xF9, 0xE1, 0xF1,
];

#[test]
#[ignore = "takes a long time to finish"]
fn nmos6502_tests() {
  for i in LEGAL_OPCODES {
    let filename = format!("{i:02x}.json");
    let specs = load_spec(&filename);

    for spec in specs {
      macro_rules! spec_assert {
        ($l_val: expr, $r_val: expr, $text: tt) => {
          let prefix = format_args!($text);
          let suffix = format_args!("for test \"{}\" in file \"{filename}\"", &spec.name);
          assert_eq!($l_val, $r_val, "{} {}", prefix, suffix)
        };
      }

      let mut uut = CPU::new_nmos();
      let mut memory = Generic64kMem::new();

      uut.reset(&memory);

      // skip decimal adc and sbc operations (not implemented yet)
      if DECIMAL_DEPENDENT_OPCODES.contains(&i) && spec.initial_status.p & 0b00001000 > 1 {
        continue;
      }
      uut.processor_status.set(spec.initial_status.p);
      uut.accumulator = spec.initial_status.a;
      uut.index_register_x = spec.initial_status.x;
      uut.index_register_y = spec.initial_status.y;
      uut.stack_pointer = spec.initial_status.s;
      uut.program_counter = spec.initial_status.pc;

      for [addr, val] in spec.initial_status.ram {
        memory[addr] = val as u8;
      }

      for (idx, cycle) in spec.cycles.iter().enumerate() {
        uut.tick(&mut memory);
        let Some(last_op) = memory.get_last_operation() else {
          panic!("unexpected lack of operation on memory after a cycle");
        };

        let addr = match last_op {
          crate::memory::Operation::Read(addr) => {
            spec_assert!(
              "read",
              cycle.operation,
              "mismatched memory operation during cycle {idx}"
            );
            addr
          }
          crate::memory::Operation::Write(addr) => {
            spec_assert!(
              "write",
              cycle.operation,
              "mismatched memory operation during cycle {idx}"
            );
            addr
          }
        };
        spec_assert!(
          addr,
          cycle.addr,
          "mismatched address access during cycle {idx}"
        );
        let val = memory.data[addr as usize];
        spec_assert!(val, cycle.val, "mismatched value during cycle {idx}");
      }

      let cycles_count = spec.cycles.len();
      spec_assert!(
        uut.current_instruction.is_none(),
        true,
        "instruction is not finished after {cycles_count} cycles"
      );
      spec_assert!(
        uut.program_counter,
        spec.final_status.pc,
        "program counter mismatch"
      );
      spec_assert!(uut.accumulator, spec.final_status.a, "accumulator mismatch");
      spec_assert!(
        uut.index_register_x,
        spec.final_status.x,
        "index register x mismatch"
      );
      spec_assert!(
        uut.index_register_y,
        spec.final_status.y,
        "index register y mismatch"
      );
      spec_assert!(
        uut.stack_pointer,
        spec.final_status.s,
        "stack pointer mismatch"
      );
      spec_assert!(
        format!("{}", uut.processor_status),
        format!(
          "{}",
          std::convert::Into::<ProcessorStatus>::into(spec.final_status.p)
        ),
        "processor status mismatch"
      );

      for [addr, expected_val] in spec.final_status.ram {
        spec_assert!(
          memory[addr],
          expected_val as u8,
          "memory val @ addr \"{addr:#04X}\" mismatch"
        );
      }
    }
  }
}

#[derive(Deserialize)]
struct THTestSpecStatus {
  pub pc: u16,
  pub s: u8,
  pub a: u8,
  pub x: u8,
  pub y: u8,
  pub p: u8,
  pub ram: Vec<[u16; 2]>,
}

fn load_spec(name: &str) -> Vec<THTestSpec> {
  let path = PathBuf::from_iter(&["test_data", "65x02", "6502", "v1", name]);
  let reader = OpenOptions::new()
    .read(true)
    .write(false)
    .create(false)
    .open(path)
    .unwrap();

  let result: Vec<THTestSpec> = serde_json::from_reader(reader).unwrap();
  result
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Cycle {
  Text(String),
  Number(u16),
}

#[derive(Deserialize)]
struct THTestSpec {
  pub name: String,
  #[serde(rename = "initial")]
  pub initial_status: THTestSpecStatus,
  #[serde(rename = "final")]
  pub final_status: THTestSpecStatus,
  #[serde(deserialize_with = "parse_cycles")]
  pub cycles: Vec<CycleInfo>,
}

struct CycleInfo {
  addr: Word,
  val: Byte,
  operation: String,
}

fn parse_cycles<'de, D>(deserializer: D) -> Result<Vec<CycleInfo>, D::Error>
where
  D: Deserializer<'de>,
{
  let cycles: Vec<[Cycle; 3]> = Deserialize::deserialize(deserializer)?;

  let mut result: Vec<CycleInfo> = vec![];
  for cycle in cycles {
    let Cycle::Number(cycle_addr) = cycle[0] else {
      return Err(Error::custom(
        "the first value in cycle information is supposed to be a number",
      ));
    };

    let Cycle::Number(cycle_val) = cycle[1] else {
      return Err(Error::custom(
        "the second value in cycle information is supposed to be a number",
      ));
    };

    let Cycle::Text(cycl_op) = &cycle[2] else {
      return Err(Error::custom(
        "the third value in cycle information is supposed to be a string",
      ));
    };

    result.push(CycleInfo {
      addr: cycle_addr,
      val: cycle_val as u8,
      operation: cycl_op.clone(),
    });
  }

  Ok(result)
}
