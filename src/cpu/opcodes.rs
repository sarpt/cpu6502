use crate::consts::Byte;

pub const ADC_IM: Byte = 0x69;
pub const ADC_ZP: Byte = 0x65;
pub const ADC_ZPX: Byte = 0x75;
pub const ADC_A: Byte = 0x6D;
pub const ADC_AX: Byte = 0x7D;
pub const ADC_AY: Byte = 0x79;
pub const ADC_INX: Byte = 0x61;
pub const ADC_INY: Byte = 0x71;
pub const AND_IM: Byte = 0x49;
pub const AND_ZP: Byte = 0x45;
pub const AND_ZPX: Byte = 0x55;
pub const AND_A: Byte = 0x4D;
pub const AND_AX: Byte = 0x5D;
pub const AND_AY: Byte = 0x59;
pub const AND_INX: Byte = 0x41;
pub const AND_INY: Byte = 0x51;
pub const ASL_ACC: Byte = 0x0A;
pub const ASL_ZP: Byte = 0x06;
pub const ASL_ZPX: Byte = 0x16;
pub const ASL_A: Byte = 0x0E;
pub const ASL_AX: Byte = 0x1E;
pub const BCC: Byte = 0x90;
pub const BCS: Byte = 0xB0;
pub const BEQ: Byte = 0xF0;
pub const BIT_ZP: Byte = 0x24;
pub const BIT_A: Byte = 0x2C;
pub const BMI: Byte = 0x30;
pub const BNE: Byte = 0xD0;
pub const BPL: Byte = 0x10;
pub const BRK: Byte = 0x00;
pub const BVC: Byte = 0x50;
pub const BVS: Byte = 0x70;
pub const CLC: Byte = 0x18;
pub const CLD: Byte = 0xD8;
pub const CLI: Byte = 0x58;
pub const CLV: Byte = 0xB8;
pub const CMP_IM: Byte = 0xC9;
pub const CMP_ZP: Byte = 0xC5;
pub const CMP_ZPX: Byte = 0xD5;
pub const CMP_A: Byte = 0xCD;
pub const CMP_AX: Byte = 0xDD;
pub const CMP_AY: Byte = 0xD9;
pub const CMP_INX: Byte = 0xC1;
pub const CMP_INY: Byte = 0xD1;
pub const CPX_IM: Byte = 0xE0;
pub const CPX_ZP: Byte = 0xE4;
pub const CPX_A: Byte = 0xEC;
pub const CPY_IM: Byte = 0xC0;
pub const CPY_ZP: Byte = 0xC4;
pub const CPY_A: Byte = 0xCC;
pub const DEC_A: Byte = 0xCE;
pub const DEC_AX: Byte = 0xDE;
pub const DEC_ZP: Byte = 0xC6;
pub const DEC_ZPX: Byte = 0xD6;
pub const DEX_IM: Byte = 0xCA;
pub const DEY_IM: Byte = 0x88;
pub const EOR_IM: Byte = 0x29;
pub const EOR_ZP: Byte = 0x25;
pub const EOR_ZPX: Byte = 0x35;
pub const EOR_A: Byte = 0x2D;
pub const EOR_AX: Byte = 0x3D;
pub const EOR_AY: Byte = 0x39;
pub const EOR_INX: Byte = 0x21;
pub const EOR_INY: Byte = 0x31;
pub const INC_ZP: Byte = 0xE6;
pub const INC_ZPX: Byte = 0xF6;
pub const INC_A: Byte = 0xEE;
pub const INC_AX: Byte = 0xFE;
pub const INX_IM: Byte = 0xE8;
pub const INY_IM: Byte = 0xC8;
pub const JMP_A: Byte = 0x4C;
pub const JMP_IN: Byte = 0x6C;
pub const JSR_A: Byte = 0x20;
pub const LDA_IM: Byte = 0xA9;
pub const LDA_ZP: Byte = 0xA5;
pub const LDA_ZPX: Byte = 0xB5;
pub const LDA_A: Byte = 0xAD;
pub const LDA_AX: Byte = 0xBD;
pub const LDA_AY: Byte = 0xB9;
pub const LDA_INX: Byte = 0xA1;
pub const LDA_INY: Byte = 0xB1;
pub const LDY_IM: Byte = 0xA0;
pub const LDY_ZP: Byte = 0xA4;
pub const LDY_ZPX: Byte = 0xB4;
pub const LDY_A: Byte = 0xAC;
pub const LDY_AX: Byte = 0xBC;
pub const LDX_IM: Byte = 0xA2;
pub const LDX_ZP: Byte = 0xA6;
pub const LDX_ZPY: Byte = 0xB6;
pub const LDX_A: Byte = 0xAE;
pub const LDX_AY: Byte = 0xBE;
pub const LSR_ACC: Byte = 0x4A;
pub const LSR_ZP: Byte = 0x46;
pub const LSR_ZPX: Byte = 0x56;
pub const LSR_A: Byte = 0x4E;
pub const LSR_AX: Byte = 0x5E;
pub const NOP: Byte = 0xEA;
pub const ORA_IM: Byte = 0x09;
pub const ORA_ZP: Byte = 0x05;
pub const ORA_ZPX: Byte = 0x15;
pub const ORA_A: Byte = 0x0D;
pub const ORA_AX: Byte = 0x1D;
pub const ORA_AY: Byte = 0x19;
pub const ORA_INX: Byte = 0x01;
pub const ORA_INY: Byte = 0x11;
pub const PHA: Byte = 0x48;
pub const PHP: Byte = 0x08;
pub const PLA: Byte = 0x68;
pub const PLP: Byte = 0x28;
pub const ROL_ACC: Byte = 0x2A;
pub const ROL_ZP: Byte = 0x26;
pub const ROL_ZPX: Byte = 0x36;
pub const ROL_A: Byte = 0x2E;
pub const ROL_AX: Byte = 0x3E;
pub const ROR_ACC: Byte = 0x6A;
pub const ROR_ZP: Byte = 0x66;
pub const ROR_ZPX: Byte = 0x76;
pub const ROR_A: Byte = 0x6E;
pub const ROR_AX: Byte = 0x7E;
pub const RTI: Byte = 0x40;
pub const RTS: Byte = 0x60;
pub const STA_ZP: Byte = 0x85;
pub const STA_ZPX: Byte = 0x95;
pub const STA_A: Byte = 0x8D;
pub const STA_AX: Byte = 0x9D;
pub const STA_AY: Byte = 0x99;
pub const STA_INX: Byte = 0x81;
pub const STA_INY: Byte = 0x91;
pub const STX_ZP: Byte = 0x86;
pub const STX_ZPY: Byte = 0x96;
pub const STX_A: Byte = 0x8E;
pub const STY_ZP: Byte = 0x84;
pub const STY_ZPX: Byte = 0x94;
pub const STY_A: Byte = 0x8C;
pub const SEC: Byte = 0x38;
pub const SED: Byte = 0xF8;
pub const SEI: Byte = 0x78;
pub const SBC_IM: Byte = 0xE9;
pub const SBC_ZP: Byte = 0xE5;
pub const SBC_ZPX: Byte = 0xF5;
pub const SBC_A: Byte = 0xED;
pub const SBC_AX: Byte = 0xFD;
pub const SBC_AY: Byte = 0xF9;
pub const SBC_INX: Byte = 0xE1;
pub const SBC_INY: Byte = 0xF1;
pub const TAX: Byte = 0xAA;
pub const TAY: Byte = 0xA8;
pub const TSX: Byte = 0xBA;
pub const TXA: Byte = 0x8A;
pub const TXS: Byte = 0x9A;
pub const TYA: Byte = 0x98;
