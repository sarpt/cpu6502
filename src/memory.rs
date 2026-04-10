use crate::consts::Word;

use super::consts::Byte;
use std::{
  cell::Cell,
  ops::{Index, IndexMut, Range},
};

const MAX_MEMORY_KB: usize = 64 * 1024;

pub trait Memory: IndexMut<Word, Output = Byte> + Index<Word, Output = Byte> {}

#[derive(Copy, Clone)]
pub enum Operation {
  Read(Word),
  Write(Word),
}

pub struct Generic64kMem {
  last_op: Cell<Option<Operation>>,
  pub data: Vec<Byte>,
}

impl Default for Generic64kMem {
  fn default() -> Self {
    Self::new()
  }
}

impl Generic64kMem {
  pub fn new() -> Self {
    Generic64kMem {
      last_op: Cell::new(None),
      data: vec![0; MAX_MEMORY_KB],
    }
  }

  pub fn store(&mut self, payload: &[(Word, Byte)]) {
    for (address, value) in payload {
      let idx: usize = (*address).into();
      self.data[idx] = *value;
    }
  }
  pub fn insert(&mut self, addr: Word, payload: &[Byte]) {
    let mut tgt_addr = addr as usize;
    for value in payload {
      self.data[tgt_addr] = *value;
      tgt_addr += 1;
    }
  }

  pub fn get_last_operation(&self) -> Option<Operation> {
    self.last_op.get()
  }
}

impl Memory for Generic64kMem {}

impl Index<Word> for Generic64kMem {
  type Output = Byte;

  fn index(&self, idx: Word) -> &Self::Output {
    let mem_address: usize = idx.into();
    self.last_op.set(Some(Operation::Read(idx)));
    &self.data[mem_address]
  }
}

impl Index<Range<Word>> for Generic64kMem {
  type Output = [Byte];

  fn index(&self, idx: Range<Word>) -> &Self::Output {
    let start: usize = idx.start.into();
    let end: usize = idx.end.into();
    &self.data[start..end]
  }
}

impl IndexMut<Word> for Generic64kMem {
  fn index_mut(&mut self, idx: Word) -> &mut Self::Output {
    let mem_address: usize = idx.into();
    self.last_op.set(Some(Operation::Write(idx)));
    &mut self.data[mem_address]
  }
}

impl From<&[(Word, Byte)]> for Generic64kMem {
  fn from(value: &[(Word, Byte)]) -> Self {
    let mut res = Generic64kMem::new();
    res.store(value);

    res
  }
}
