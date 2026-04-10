use std::fs::OpenOptions;
use std::path::PathBuf;

use crate::cpu::CPU;
use crate::cpu::processor_status::ProcessorStatus;

use crate::memory::Generic64kMem;
use serde::Deserialize;

#[test]
fn nmos6502_tests() {
  for i in 0..=255 {
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

        let Cycle::Number(cycle_addr) = cycle[0] else {
          panic!("the first value in cycle information is supposed to be a number");
        };

        let Cycle::Number(cycle_val) = cycle[1] else {
          panic!("the second value in cycle information is supposed to be a number");
        };

        let Cycle::Text(cycl_op) = &cycle[2] else {
          panic!("the third value in cycle information is supposed to be a string");
        };

        let addr = match last_op {
          crate::memory::Operation::Read(addr) => {
            assert_eq!(
              cycl_op, "read",
              "mismatched memory operation during cycle {idx}"
            );
            addr
          }
          crate::memory::Operation::Write(addr) => {
            assert_eq!(
              cycl_op, "write",
              "mismatched memory operation during cycle {idx}"
            );
            addr
          }
        };
        spec_assert!(
          addr,
          cycle_addr,
          "mismatched address access during cycle {idx}"
        );
        let val = memory.data[addr as usize];
        spec_assert!(val, cycle_val as u8, "mismatched value during cycle {idx}");
      }

      spec_assert!(
        uut.program_counter,
        spec.final_status.pc,
        "program counter mismatch"
      );
      assert_eq!(uut.accumulator, spec.final_status.a, "accumulator mismatch");
      assert_eq!(
        uut.index_register_x, spec.final_status.x,
        "index register x mismatch"
      );
      assert_eq!(
        uut.index_register_y, spec.final_status.y,
        "index register y mismatch"
      );
      assert_eq!(
        uut.stack_pointer, spec.final_status.s,
        "stack pointer mismatch"
      );
      assert_eq!(
        format!("{}", uut.processor_status),
        format!(
          "{}",
          std::convert::Into::<ProcessorStatus>::into(spec.final_status.p)
        ),
        "processor status mismatch"
      );

      for [addr, expected_val] in spec.final_status.ram {
        assert_eq!(
          memory[addr], expected_val as u8,
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
  pub cycles: Vec<[Cycle; 3]>,
}
