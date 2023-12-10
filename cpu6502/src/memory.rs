use crate::consts::Word;

use super::consts::Byte;
use std::ops::{Index, IndexMut};

const MAX_MEMORY_KB: usize = 64 * 1024;

pub trait Memory: IndexMut<Word, Output = Byte>
    + Index<Word, Output = Byte> {}

pub struct VecMemory {
    pub data: Vec<Byte>,
}

impl VecMemory {
    pub fn new() -> Self {
        return VecMemory { data: vec![0; MAX_MEMORY_KB] };
    }

    pub fn store(&mut self, payload: &[(Word,Byte)]) {
        for (address, value) in payload {
            let idx: usize = (*address).into();
            self.data[idx] = *value;
        }
    }
}

impl Memory for VecMemory {}

impl Index<Word> for VecMemory {
    type Output = Byte;

    fn index(&self, idx: Word) -> &Self::Output {
        let mem_address: usize = idx.into();
        return &self.data[mem_address];
    }
}

impl IndexMut<Word> for VecMemory {
    fn index_mut(&mut self, idx: Word) -> &mut Self::Output {
        let mem_address: usize = idx.into();
        return &mut self.data[mem_address];
    }
}

impl From<&[(Word,Byte)]> for VecMemory {
    fn from(value: &[(Word,Byte)]) -> Self {
        let mut res = VecMemory::new();
        res.store(value);

        return res;
    }
}

