use cpu6502::machine;

fn main() {
    let program: &[(u16, u8)] = &[
        (0xFFFC, 0x34), // JMP $1234
        (0xFFFD, 0x12),
        (0x1234, 0xB5), // LDA $AB,X
        (0x1235, 0xAB),
        (0x00AB, 0x42),
        (0x1236, 0x20), // JSR $0300
        (0x1237, 0x00),
        (0x1238, 0x03),
        (0x0300, 0xA9), // LDA #FF
        (0x0301, 0xFF),
    ];
    let mut machine = machine::Machine::new();
    let cycles = 14;
    machine.execute_cycles(program, cycles);
}
