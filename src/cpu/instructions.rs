#![allow(dead_code)]

use paste::paste;
use phf::phf_map;

use crate::consts::Byte;
use crate::cpu::CPU;
use crate::cpu::tasks::Tasks;

use self::arithmetic::*;
use self::branches::*;
use self::inc_and_decrements::*;
use self::jumps_and_calls::*;
use self::load_and_store_ops::*;
use self::logical::*;
use self::register_transfers::*;
use self::shifts::*;
use self::stack_operations::*;
use self::status_flag_changes::*;
use self::system_functions::*;

type OpcodeHandler = fn(&mut CPU) -> Box<dyn Tasks>;

pub struct Instruction {
  pub handler: OpcodeHandler,
  pub name: &'static str,
}

macro_rules! instructions {
  ( $($opcode:literal => ($handler:ident, $display:literal)),+ ) => {
    paste! {
      $(
        pub const [<$handler:upper>]: Byte = $opcode;
      )+
    }

    // this is required becasue phf currently cannot evalute consts as keys
    pub static INSTRUCTIONS: phf::Map<Byte, Instruction> = phf_map!{
      $(
        $opcode => Instruction { handler: $handler, name: $display }
      ),+
    };
  };
}

instructions! {
  // u8 suffix is required for phf
  0x69u8 => (adc_im, "ADC"),
  0x65u8 => (adc_zp, "ADC"),
  0x75u8 => (adc_zpx, "ADC"),
  0x6Du8 => (adc_a, "ADC"),
  0x7Du8 => (adc_ax, "ADC"),
  0x79u8 => (adc_ay, "ADC"),
  0x61u8 => (adc_inx, "ADC"),
  0x71u8 => (adc_iny, "ADC"),
  0x49u8 => (and_im, "AND"),
  0x45u8 => (and_zp, "AND"),
  0x55u8 => (and_zpx, "AND"),
  0x4Du8 => (and_a, "AND"),
  0x5Du8 => (and_ax, "AND"),
  0x59u8 => (and_ay, "AND"),
  0x41u8 => (and_inx, "AND"),
  0x51u8 => (and_iny, "AND"),
  0x0Au8 => (asl_acc, "ASL"),
  0x06u8 => (asl_zp, "ASL"),
  0x16u8 => (asl_zpx, "ASL"),
  0x0Eu8 => (asl_a, "ASL"),
  0x1Eu8 => (asl_ax, "ASL"),
  0x90u8 => (bcc, "BCC"),
  0xB0u8 => (bcs, "BCS"),
  0xF0u8 => (beq, "BEQ"),
  0x24u8 => (bit_zp, "BIT"),
  0x2Cu8 => (bit_a, "BIT"),
  0x30u8 => (bmi, "BMI"),
  0xD0u8 => (bne, "BNE"),
  0x10u8 => (bpl, "BPL"),
  0x00u8 => (brk, "BRK"),
  0x50u8 => (bvc, "BVC"),
  0x70u8 => (bvs, "BVS"),
  0x18u8 => (clc, "CLC"),
  0xD8u8 => (cld, "CLD"),
  0x58u8 => (cli, "CLI"),
  0xB8u8 => (clv, "CLV"),
  0xC9u8 => (cmp_im, "CMP"),
  0xC5u8 => (cmp_zp, "CMP"),
  0xD5u8 => (cmp_zpx, "CMP"),
  0xCDu8 => (cmp_a, "CMP"),
  0xDDu8 => (cmp_ax, "CMP"),
  0xD9u8 => (cmp_ay, "CMP"),
  0xC1u8 => (cmp_inx, "CMP"),
  0xD1u8 => (cmp_iny, "CMP"),
  0xE0u8 => (cpx_im, "CPX"),
  0xE4u8 => (cpx_zp, "CPX"),
  0xECu8 => (cpx_a, "CPX"),
  0xC0u8 => (cpy_im, "CPY"),
  0xC4u8 => (cpy_zp, "CPY"),
  0xCCu8 => (cpy_a, "CPY"),
  0xCEu8 => (dec_a, "DEC"),
  0xDEu8 => (dec_ax, "DEC"),
  0xC6u8 => (dec_zp, "DEC"),
  0xD6u8 => (dec_zpx, "DEC"),
  0xCAu8 => (dex_im, "DEX"),
  0x88u8 => (dey_im, "DEY"),
  0x29u8 => (eor_im, "EOR"),
  0x25u8 => (eor_zp, "EOR"),
  0x35u8 => (eor_zpx, "EOR"),
  0x2Du8 => (eor_a, "EOR"),
  0x3Du8 => (eor_ax, "EOR"),
  0x39u8 => (eor_ay, "EOR"),
  0x21u8 => (eor_inx, "EOR"),
  0x31u8 => (eor_iny, "EOR"),
  0xE6u8 => (inc_zp, "INC"),
  0xF6u8 => (inc_zpx, "INC"),
  0xEEu8 => (inc_a, "INC"),
  0xFEu8 => (inc_ax, "INC"),
  0xE8u8 => (inx_im, "INX"),
  0xC8u8 => (iny_im, "INY"),
  0x4Cu8 => (jmp_a, "JMP"),
  0x6Cu8 => (jmp_in, "JMP"),
  0x20u8 => (jsr_a, "JSR"),
  0xA9u8 => (lda_im, "LDA"),
  0xA5u8 => (lda_zp, "LDA"),
  0xB5u8 => (lda_zpx, "LDA"),
  0xADu8 => (lda_a, "LDA"),
  0xBDu8 => (lda_ax, "LDA"),
  0xB9u8 => (lda_ay, "LDA"),
  0xA1u8 => (lda_inx, "LDA"),
  0xB1u8 => (lda_iny, "LDA"),
  0xA0u8 => (ldy_im, "LDY"),
  0xA4u8 => (ldy_zp, "LDY"),
  0xB4u8 => (ldy_zpx, "LDY"),
  0xACu8 => (ldy_a, "LDY"),
  0xBCu8 => (ldy_ax, "LDY"),
  0xA2u8 => (ldx_im, "LDX"),
  0xA6u8 => (ldx_zp, "LDX"),
  0xB6u8 => (ldx_zpy, "LDX"),
  0xAEu8 => (ldx_a, "LDX"),
  0xBEu8 => (ldx_ay, "LDX"),
  0x4Au8 => (lsr_acc, "LSR"),
  0x46u8 => (lsr_zp, "LSR"),
  0x56u8 => (lsr_zpx, "LSR"),
  0x4Eu8 => (lsr_a, "LSR"),
  0x5Eu8 => (lsr_ax, "LSR"),
  0xEAu8 => (nop, "NOP"),
  0x09u8 => (ora_im, "ORA"),
  0x05u8 => (ora_zp, "ORA"),
  0x15u8 => (ora_zpx, "ORA"),
  0x0Du8 => (ora_a, "ORA"),
  0x1Du8 => (ora_ax, "ORA"),
  0x19u8 => (ora_ay, "ORA"),
  0x01u8 => (ora_inx, "ORA"),
  0x11u8 => (ora_iny, "ORA"),
  0x48u8 => (pha, "PHA"),
  0x08u8 => (php, "PHP"),
  0x68u8 => (pla, "PLA"),
  0x28u8 => (plp, "PLP"),
  0x2Au8 => (rol_acc, "ROL"),
  0x26u8 => (rol_zp, "ROL"),
  0x36u8 => (rol_zpx, "ROL"),
  0x2Eu8 => (rol_a, "ROL"),
  0x3Eu8 => (rol_ax, "ROL"),
  0x6Au8 => (ror_acc, "ROR"),
  0x66u8 => (ror_zp, "ROR"),
  0x76u8 => (ror_zpx, "ROR"),
  0x6Eu8 => (ror_a, "ROR"),
  0x7Eu8 => (ror_ax, "ROR"),
  0x40u8 => (rti, "RTI"),
  0x60u8 => (rts, "RTS"),
  0x85u8 => (sta_zp, "STA"),
  0x95u8 => (sta_zpx, "STA"),
  0x8Du8 => (sta_a, "STA"),
  0x9Du8 => (sta_ax, "STA"),
  0x99u8 => (sta_ay, "STA"),
  0x81u8 => (sta_inx, "STA"),
  0x91u8 => (sta_iny, "STA"),
  0x86u8 => (stx_zp, "STX"),
  0x96u8 => (stx_zpy, "STX"),
  0x8Eu8 => (stx_a, "STX"),
  0x84u8 => (sty_zp, "STY"),
  0x94u8 => (sty_zpx, "STY"),
  0x8Cu8 => (sty_a, "STY"),
  0x38u8 => (sec, "SEC"),
  0xF8u8 => (sed, "SED"),
  0x78u8 => (sei, "SEI"),
  0xE9u8 => (sbc_im, "SBC"),
  0xE5u8 => (sbc_zp, "SBC"),
  0xF5u8 => (sbc_zpx, "SBC"),
  0xEDu8 => (sbc_a, "SBC"),
  0xFDu8 => (sbc_ax, "SBC"),
  0xF9u8 => (sbc_ay, "SBC"),
  0xE1u8 => (sbc_inx, "SBC"),
  0xF1u8 => (sbc_iny, "SBC"),
  0xAAu8 => (tax, "TAX"),
  0xA8u8 => (tay, "TAY"),
  0xBAu8 => (tsx, "TSX"),
  0x8Au8 => (txa, "TXA"),
  0x9Au8 => (txs, "TXS"),
  0x98u8 => (tya, "TYA")
}

mod arithmetic;
mod branches;
mod inc_and_decrements;
mod jumps_and_calls;
mod load_and_store_ops;
mod logical;
mod register_transfers;
mod shifts;
mod stack_operations;
mod status_flag_changes;
mod system_functions;
