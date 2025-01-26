pub type Byte = u8;
pub type Word = u16;

pub const STACK_PAGE_HI: Word = 0x0100;

pub const BRK_INTERRUPT_VECTOR: Word = 0xFFFE;
pub const RESET_VECTOR: Word = 0xFFFC;
