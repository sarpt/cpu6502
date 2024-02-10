use std::collections::HashMap;

use crate::consts::Byte;

use self::arithmetic::*;
use self::branches::*;
use self::inc_and_decrements::*;
use self::jumps_and_calls::*;
use self::load_and_store_ops::*;
use self::logical::*;
use self::stack_operations::*;
use self::status_flag_changes::*;
use self::system_functions::*;

use super::OpcodeHandler;
use crate::cpu::opcodes::*;

pub fn get_instructions() -> HashMap<Byte, OpcodeHandler> {
    return HashMap::from([
        (LDA_IM, lda_im as OpcodeHandler),
        (LDA_ZP, lda_zp),
        (LDA_ZPX, lda_zpx),
        (LDA_A, lda_a),
        (LDA_AX, lda_ax),
        (LDA_AY, lda_ay),
        (LDA_INX, lda_inx),
        (LDA_INY, lda_iny),
        (LDY_IM, ldy_im),
        (LDY_ZP, ldy_zp),
        (LDY_ZPX, ldy_zpx),
        (LDY_A, ldy_a),
        (LDY_AX, ldy_ax),
        (LDX_IM, ldx_im),
        (LDX_ZP, ldx_zp),
        (LDX_ZPY, ldx_zpy),
        (LDX_A, ldx_a),
        (LDX_AY, ldx_ay),
        (JMP_A, jmp_a),
        (JMP_IN, jmp_in),
        (JSR_A, jsr_a),
        (RTS, rts),
        (BCC, bcc),
        (BCS, bcs),
        (BEQ, beq),
        (BNE, bne),
        (CMP_IM, cmp_im),
        (CMP_ZP, cmp_zp),
        (CMP_ZPX, cmp_zpx),
        (CMP_A, cmp_a),
        (CMP_AX, cmp_ax),
        (CMP_AY, cmp_ay),
        (CMP_INX, cmp_inx),
        (CMP_INY, cmp_iny),
        (CPX_IM, cpx_im),
        (CPX_ZP, cpx_zp),
        (CPX_A, cpx_a),
        (CPY_IM, cpy_im),
        (CPY_ZP, cpy_zp),
        (CPY_A, cpy_a),
        (INC_ZP, inc_zp),
        (INC_ZPX, inc_zpx),
        (INC_A, inc_a),
        (INC_AX, inc_ax),
        (INX_IM, inx_im),
        (INY_IM, iny_im),
        (DEC_ZP, dec_zp),
        (DEC_ZPX, dec_zpx),
        (DEC_A, dec_a),
        (DEC_AX, dec_ax),
        (DEX_IM, dex_im),
        (DEY_IM, dey_im),
        (STA_ZP, sta_zp),
        (STA_ZPX, sta_zpx),
        (STA_A, sta_a),
        (STA_AX, sta_ax),
        (STA_AY, sta_ay),
        (STA_INX, sta_inx),
        (STA_INY, sta_iny),
        (STX_ZP, stx_zp),
        (STX_ZPY, stx_zpy),
        (STX_A, stx_a),
        (STY_ZP, sty_zp),
        (STY_ZPX, sty_zpx),
        (STY_A, sty_a),
        (ORA_IM, ora_im),
        (ORA_ZP, ora_zp),
        (ORA_ZPX, ora_zpx),
        (ORA_A, ora_a),
        (ORA_AX, ora_ax),
        (ORA_AY, ora_ay),
        (ORA_INX, ora_inx),
        (ORA_INY, ora_iny),
        (NOP, nop),
        (CLC, clc),
        (CLD, cld),
        (CLI, cli),
        (CLV, clv),
        (SEC, sec),
        (SED, sed),
        (SEI, sei),
        (BRK, brk),
        (BIT_A, bit_a),
        (BIT_ZP, bit_zp),
        (PHA, pha),
        (PHP, php),
        (PLA, pla),
        (PLP, plp),
        (TSX, tsx),
        (TXS, txs),
    ]);
}

mod arithmetic;
mod branches;
mod inc_and_decrements;
mod jumps_and_calls;
mod load_and_store_ops;
mod logical;
mod stack_operations;
mod status_flag_changes;
mod system_functions;
