use crate::consts::{Byte, Word};

#[derive(Default)]
pub struct Address {
    val: Option<Word>,
}

impl Address {
    pub fn new() -> Self {
        return Address { val: None };
    }

    pub fn value(&self) -> Option<Word> {
        return self.val;
    }

    pub fn set<T: Into<Word>>(&mut self, val: T) {
        self.val = Some(val.into());
    }

    pub fn set_lo(&mut self, lo: Byte) {
        let hi: Byte = match self.val {
            Some(val) => val.to_le_bytes()[1],
            None => 0,
        };

        self.val = Some(Word::from_le_bytes([lo, hi]));
    }

    pub fn set_hi(&mut self, hi: Byte) {
        let lo: Byte = match self.val {
            Some(val) => val.to_le_bytes()[0],
            None => 0,
        };

        self.val = Some(Word::from_le_bytes([lo, hi]));
    }
}
