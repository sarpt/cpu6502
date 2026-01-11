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
      dbg.probe(cpu);
    }

    if cpu.current_instruction.is_none() {
      break;
    }
  }
}

pub fn execute_until_break(
  cpu: &mut CPU,
  memory: &mut dyn Memory,
  mut debugger: Option<&mut Debugger>,
) -> usize {
  while !cpu.processor_status.get_break_flag() {
    match debugger {
      Some(ref mut dbg) => execute_next_instruction(cpu, memory, Some(dbg)),
      None => execute_next_instruction(cpu, memory, None),
    }
  }

  cpu.cycle
}
