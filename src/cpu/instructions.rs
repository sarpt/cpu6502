#![allow(dead_code)]

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
  ( $($name:ident => ($opcode:literal, $handler:ident)),+ ) => {
    $(
      pub const $name: Byte = $opcode;
    )+

    // this is required becasue phf currently cannot evalute consts as keys
    pub static INSTRUCTIONS: phf::Map<Byte, Instruction> = phf_map!{
      $(
        $opcode => Instruction { handler: $handler, name: stringify!($name) }
      ),+
    };
  };
}

instructions! {
  // u8 suffix is required for phf
  ADC_IM => (0x69u8, adc_im),
  ADC_ZP => (0x65u8, adc_zp),
  ADC_ZPX => (0x75u8, adc_zpx),
  ADC_A => (0x6Du8, adc_a),
  ADC_AX => (0x7Du8, adc_ax),
  ADC_AY => (0x79u8, adc_ay),
  ADC_INX => (0x61u8, adc_inx),
  ADC_INY => (0x71u8, adc_iny),
  ND_IM => (0x49u8, and_im),
  AND_ZP => (0x45u8, and_zp),
  AND_ZPX => (0x55u8, and_zpx),
  AND_A => (0x4Du8, and_a),
  AND_AX => (0x5Du8, and_ax),
  AND_AY => (0x59u8, and_ay),
  AND_INX => (0x41u8, and_inx),
  AND_INY => (0x51u8, and_iny),
  ASL_ACC => (0x0Au8, asl_acc),
  ASL_ZP => (0x06u8, asl_zp),
  ASL_ZPX => (0x16u8, asl_zpx),
  ASL_A => (0x0Eu8, asl_a),
  ASL_AX => (0x1Eu8, asl_ax),
  BCC => (0x90u8, bcc),
  BCS => (0xB0u8, bcs),
  BEQ => (0xF0u8, beq),
  BIT_ZP => (0x24u8, bit_zp),
  BIT_A => (0x2Cu8, bit_a),
  BMI => (0x30u8, bmi),
  BNE => (0xD0u8, bne),
  BPL => (0x10u8, bpl),
  BRK => (0x00u8, brk),
  BVC => (0x50u8, bvc),
  BVS => (0x70u8, bvs),
  CLC => (0x18u8, clc),
  CLD => (0xD8u8, cld),
  CLI => (0x58u8, cli),
  CLV => (0xB8u8, clv),
  CMP_IM => (0xC9u8, cmp_im),
  CMP_ZP => (0xC5u8, cmp_zp),
  CMP_ZPX => (0xD5u8, cmp_zpx),
  CMP_A => (0xCDu8, cmp_a),
  CMP_AX => (0xDDu8, cmp_ax),
  CMP_AY => (0xD9u8, cmp_ay),
  CMP_INX => (0xC1u8, cmp_inx),
  CMP_INY => (0xD1u8, cmp_iny),
  CPX_IM => (0xE0u8, cpx_im),
  CPX_ZP => (0xE4u8, cpx_zp),
  CPX_A => (0xECu8, cpx_a),
  CPY_IM => (0xC0u8, cpy_im),
  CPY_ZP => (0xC4u8, cpy_zp),
  CPY_A => (0xCCu8, cpy_a),
  DEC_A => (0xCEu8, dec_a),
  DEC_AX => (0xDEu8, dec_ax),
  DEC_ZP => (0xC6u8, dec_zp),
  DEC_ZPX => (0xD6u8, dec_zpx),
  DEX_IM => (0xCAu8, dex_im),
  DEY_IM => (0x88u8, dey_im),
  EOR_IM => (0x29u8, eor_im),
  EOR_ZP => (0x25u8, eor_zp),
  EOR_ZPX => (0x35u8, eor_zpx),
  EOR_A => (0x2Du8, eor_a),
  EOR_AX => (0x3Du8, eor_ax),
  EOR_AY => (0x39u8, eor_ay),
  EOR_INX => (0x21u8, eor_inx),
  EOR_INY => (0x31u8, eor_iny),
  INC_ZP => (0xE6u8, inc_zp),
  INC_ZPX => (0xF6u8, inc_zpx),
  INC_A => (0xEEu8, inc_a),
  INC_AX => (0xFEu8, inc_ax),
  INX_IM => (0xE8u8, inx_im),
  INY_IM => (0xC8u8, iny_im),
  JMP_A => (0x4Cu8, jmp_a),
  JMP_IN => (0x6Cu8, jmp_in),
  JSR_A => (0x20u8, jsr_a),
  LDA_IM => (0xA9u8, lda_im),
  LDA_ZP => (0xA5u8, lda_zp),
  LDA_ZPX => (0xB5u8, lda_zpx),
  LDA_A => (0xADu8, lda_a),
  LDA_AX => (0xBDu8, lda_ax),
  LDA_AY => (0xB9u8, lda_ay),
  LDA_INX => (0xA1u8, lda_inx),
  LDA_INY => (0xB1u8, lda_iny),
  LDY_IM => (0xA0u8, ldy_im),
  LDY_ZP => (0xA4u8, ldy_zp),
  LDY_ZPX => (0xB4u8, ldy_zpx),
  LDY_A => (0xACu8, ldy_a),
  LDY_AX => (0xBCu8, ldy_ax),
  LDX_IM => (0xA2u8, ldx_im),
  LDX_ZP => (0xA6u8, ldx_zp),
  LDX_ZPY => (0xB6u8, ldx_zpy),
  LDX_A => (0xAEu8, ldx_a),
  LDX_AY => (0xBEu8, ldx_ay),
  LSR_ACC => (0x4Au8, lsr_acc),
  LSR_ZP => (0x46u8, lsr_zp),
  LSR_ZPX => (0x56u8, lsr_zpx),
  LSR_A => (0x4Eu8, lsr_a),
  LSR_AX => (0x5Eu8, lsr_ax),
  NOP => (0xEAu8, nop),
  ORA_IM => (0x09u8, ora_im),
  ORA_ZP => (0x05u8, ora_zp),
  ORA_ZPX => (0x15u8, ora_zpx),
  ORA_A => (0x0Du8, ora_a),
  ORA_AX => (0x1Du8, ora_ax),
  ORA_AY => (0x19u8, ora_ay),
  ORA_INX => (0x01u8, ora_inx),
  ORA_INY => (0x11u8, ora_iny),
  PHA => (0x48u8, pha),
  PHP => (0x08u8, php),
  PLA => (0x68u8, pla),
  PLP => (0x28u8, plp),
  ROL_ACC => (0x2Au8, rol_acc),
  ROL_ZP => (0x26u8, rol_zp),
  ROL_ZPX => (0x36u8, rol_zpx),
  ROL_A => (0x2Eu8, rol_a),
  ROL_AX => (0x3Eu8, rol_ax),
  ROR_ACC => (0x6Au8, ror_acc),
  ROR_ZP => (0x66u8, ror_zp),
  ROR_ZPX => (0x76u8, ror_zpx),
  ROR_A => (0x6Eu8, ror_a),
  ROR_AX => (0x7Eu8, ror_ax),
  RTI => (0x40u8, rti),
  RTS => (0x60u8, rts),
  STA_ZP => (0x85u8, sta_zp),
  STA_ZPX => (0x95u8, sta_zpx),
  STA_A => (0x8Du8, sta_a),
  STA_AX => (0x9Du8, sta_ax),
  STA_AY => (0x99u8, sta_ay),
  STA_INX => (0x81u8, sta_inx),
  STA_INY => (0x91u8, sta_iny),
  STX_ZP => (0x86u8, stx_zp),
  STX_ZPY => (0x96u8, stx_zpy),
  STX_A => (0x8Eu8, stx_a),
  STY_ZP => (0x84u8, sty_zp),
  STY_ZPX => (0x94u8, sty_zpx),
  STY_A => (0x8Cu8, sty_a),
  SEC => (0x38u8, sec),
  SED => (0xF8u8, sed),
  SEI => (0x78u8, sei),
  SBC_IM => (0xE9u8, sbc_im),
  SBC_ZP => (0xE5u8, sbc_zp),
  SBC_ZPX => (0xF5u8, sbc_zpx),
  SBC_A => (0xEDu8, sbc_a),
  SBC_AX => (0xFDu8, sbc_ax),
  SBC_AY => (0xF9u8, sbc_ay),
  SBC_INX => (0xE1u8, sbc_inx),
  SBC_INY => (0xF1u8, sbc_iny),
  TAX => (0xAAu8, tax),
  TAY => (0xA8u8, tay),
  TSX => (0xBAu8, tsx),
  TXA => (0x8Au8, txa),
  TXS => (0x9Au8, txs),
  TYA => (0x98u8, tya)
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
