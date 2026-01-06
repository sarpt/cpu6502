use cpu6502::cpu::CPU;
use cpu6502::cpu::utils::execute_until_break;
use cpu6502::memory::Generic64kMem;
use std::cell::RefCell;
use std::str;

const TO_LOWER_PROCEDURE: &[(u16, u8)] = &[
    (0x0080, 0x00), // SRC ADDR
    (0x0081, 0x04),
    (0x0082, 0x00), // DST ADDR
    (0x0083, 0x05),
    (0x0600, 0xA0), // TOLOWER   LDY #00 ;setup SRC & DST character index to 0
    (0x0601, 0x00),
    (0x0602, 0xB1), // LOOP      LDA (SRC),Y ;fetch SRC character at Y to A
    (0x0603, 0x80),
    (0x0604, 0xF0), //           BEQ DONE ;end of string character in A is 0
    (0x0605, 0x11),
    (0x0606, 0xC9), //           CMP #'A' ;compare character in accumulator with "A"
    (0x0607, 0x41),
    (0x0608, 0x90), //           BCC SKIP ;if check above smaller then go to SKIP
    (0x0609, 0x06),
    (0x060A, 0xC9), //           CMP #'Z' + 1 ;compare character in accumulator with "Z" + 1
    (0x060B, 0x5B),
    (0x060C, 0xB0), //           BCS SKIP ;if check above bigger than Z go to SKIP
    (0x060D, 0x02),
    (0x060E, 0x09), //           ORA #%00100000 ;convert character in accumulator to lower case
    (0x060F, 0x20),
    (0x0610, 0x91), // SKIP      STA (DST),Y ;store character in accumulator to DST offset by index Y
    (0x0611, 0x82),
    (0x0612, 0xC8), //           INY ;increment character index
    (0x0613, 0xD0), //           BNE LOOP ;process next character if Y not wrapped around
    (0x0614, 0xED),
    (0x0615, 0x38), //           SEC ;string too long error
    (0x0616, 0x60), //           RTS ;function return
    (0x0617, 0x91), // DONE      STA (DST),Y ;store 0 as a string terminator (after jump at 0x0604)
    (0x0618, 0x82), //
    (0x0619, 0x18), //           CLC ;mark no error
    (0x061A, 0x60), //           RTS ;function return
];

const BOOTSTRAP: &[(u16, u8)] = &[
    (0xFFFC, 0x00), // jump to $0600
    (0xFFFD, 0x06),
];

#[test]
fn should_change_word_to_lower_case() {
    let src_string: &[(u16, u8)] = &[
        (0x0400, 0x53), // Some Message
        (0x0401, 0x6F),
        (0x0402, 0x6D),
        (0x0403, 0x65),
        (0x0404, 0x20),
        (0x0405, 0x4D),
        (0x0406, 0x65),
        (0x0407, 0x73),
        (0x0408, 0x73),
        (0x0409, 0x61),
        (0x040A, 0x67),
        (0x040B, 0x65),
    ];
    let program: &[(u16, u8)] = &[src_string, TO_LOWER_PROCEDURE, BOOTSTRAP].concat();
    let mut memory = Generic64kMem::from(program);

    let mut cpu = CPU::new_nmos();
    cpu.reset(&memory);
    execute_until_break(&mut cpu, &mut memory, None);

    assert_eq!(str::from_utf8(&memory[0x0500..0x050C]), Ok("some message"));
    assert_eq!(cpu.get_processor_status() & 0b00000001, 0);
}

#[test]
fn should_report_string_too_long() {
    let program: &[(u16, u8)] = &[TO_LOWER_PROCEDURE, BOOTSTRAP].concat();
    let mut memory = Generic64kMem::from(program);
    memory.insert(0x0400, &[0x53; 256]);

    let mut cpu = CPU::new_nmos();
    cpu.reset(&memory);
    execute_until_break(&mut cpu, &mut memory, None);

    assert_eq!(str::from_utf8(&memory[0x0500..0x050C]), Ok("ssssssssssss"));
    assert_eq!(cpu.get_processor_status() & 0b00000001, 1);
}

#[test]
fn should_handle_empty_string() {
    let program: &[(u16, u8)] = &[TO_LOWER_PROCEDURE, BOOTSTRAP].concat();
    let mut memory = Generic64kMem::from(program);

    let mut cpu = CPU::new_nmos();
    cpu.reset(&memory);
    execute_until_break(&mut cpu, &mut memory, None);

    assert_eq!(memory[0x0500], 0);
    assert_eq!(cpu.get_processor_status() & 0b00000001, 0);
}
