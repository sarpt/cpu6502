use crate::cpu::{Registers, Tasks, CPU};

pub struct TransferRegistersTasks {
    src: Registers,
    tgt: Registers,
    done: bool,
}

impl TransferRegistersTasks {
    pub fn new(src: Registers, tgt: Registers) -> Self {
        TransferRegistersTasks {
            src,
            tgt,
            done: false,
        }
    }
}

impl Tasks for TransferRegistersTasks {
    fn done(&self) -> bool {
        self.done
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        if self.done() {
            panic!("tick mustn't be called when done")
        }

        cpu.transfer_registers(self.src, self.tgt);

        self.done = true;
        self.done
    }
}
