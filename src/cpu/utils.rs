use crate::{
  cpu::{CPU, debugger::Debugger},
  memory::Memory,
};

pub fn execute_next_instruction(
  cpu: &mut CPU,
  memory: &mut dyn Memory,
  mut debugger: Option<&mut Debugger>,
) {
  loop {
    cpu.tick(memory);

    if let Some(dbg) = debugger.as_mut() {
      dbg.probe(cpu, memory);
    }

    if cpu.current_instruction.is_none() {
      break;
    }
  }
}

pub fn execute_until_break(
  cpu: &mut CPU,
  memory: &mut dyn Memory,
  debugger: &mut Debugger,
) -> usize {
  while !cpu.processor_status.get_break_flag() {
    execute_next_instruction(cpu, memory, Some(debugger));
    let Some(inst) = debugger.get_last_instruction() else {
      continue;
    };

    if inst.opcode == 0x00 {
      break;
    }
  }

  cpu.cycle
}
