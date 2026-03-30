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
      let mut uut = CPU::new_nmos();
      let mut memory = Generic64kMem::new();

      uut.processor_status = spec.initial_status.p.into();

      uut.reset(&memory);

      uut.accumulator = spec.initial_status.a;
      uut.index_register_x = spec.initial_status.x;
      uut.index_register_y = spec.initial_status.y;
      uut.stack_pointer = spec.initial_status.s;
      uut.program_counter = spec.initial_status.pc;

      for [addr, val] in spec.initial_status.ram {
        memory[addr] = val as u8;
      }

      for _ in spec.cycles {
        uut.tick(&mut memory);
      }

      assert_eq!(
        uut.program_counter, spec.final_status.pc,
        "program counter for test \"{}\" in file \"{filename}\"",
        spec.name
      );
      assert_eq!(
        uut.accumulator, spec.final_status.a,
        "accumulator for test \"{}\" in file \"{filename}\"",
        spec.name
      );
      assert_eq!(
        uut.index_register_x, spec.final_status.x,
        "index register x for test \"{}\" in file \"{filename}\"",
        spec.name
      );
      assert_eq!(
        uut.index_register_y, spec.final_status.y,
        "index register y for test \"{}\" in file \"{filename}\"",
        spec.name
      );
      assert_eq!(
        uut.stack_pointer, spec.final_status.s,
        "stack pointer for test \"{}\" in file \"{filename}\"",
        spec.name
      );
      assert_eq!(
        format!("{}", uut.processor_status),
        format!(
          "{}",
          std::convert::Into::<ProcessorStatus>::into(spec.final_status.p)
        ),
        "processor status for test \"{}\" in file \"{filename}\"",
        spec.name
      );
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
