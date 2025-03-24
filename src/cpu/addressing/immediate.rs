use crate::cpu::tasks::Tasks;

pub struct ImmediateAddressingTasks {
    done: bool,
}

impl ImmediateAddressingTasks {
    pub fn new() -> Self {
        return ImmediateAddressingTasks { done: false };
    }
}

impl Tasks for ImmediateAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> bool {
        if self.done {
            return self.done;
        }

        let addr = cpu.program_counter;
        cpu.set_address_output(addr);
        cpu.increment_program_counter();
        self.done = true;

        return self.done;
    }
}
