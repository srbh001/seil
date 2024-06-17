// A lexer for the Assembly Language for the IITB RISC-V Processor.
//
// This lexer supports:
// - 24 instructions for the Pipelined Architecture
// - 14 instructions for the Single Cycle Architecture
//
// The ISA is developed by Prof. Virendra Singh, IIT Bombay.
// For more information, see <https://www.ee.iitb.ac.in/~viren/>.
// The ISA is based on the RISC-V ISA and has been modified to suit the needs of the EE309 and EE224 courses at IIT Bombay.
//
// Author: Saurabh <saurabhkumarnomeas@gmail.com>

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i32),
}

#[derive(Debug, PartialEq)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Instr(Instruction), // Reserved keywords to be used only for instructions - mentioned in the ISA
    Reg(Register),
    Operand,
    Label,
    Immediate6,
    Immediate9,
    // Address, - Not relevant for this ISA
    Comment,
    Unknown,
    Symbol,
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum InstructionSingleCycle {
    ADD, // 0x00
    SUB, //0x02
    MUL, //0x03
    ADI, // 0x01
    AND, // 0x04
    ORA, // 0x05
    IMP, // 0x06
    LHI, // 0x08
    LLI, // 0x09
    LW,  // 0x0A
    SW,  // 0x0B
    BEQ, // 0x0C
    JAL, // 0x0D
    JLR, // 0x0F
}

pub enum InstructionPipelined {
    ADA, //00_01 RA RB RC 0 00
    ADC, //00_01 RA RB RC 0 10
    ADZ, //00_01 RA RB RC 0 01
    AWC, //00_01 RA RB RC 0 11
    ACA, //00_01 RA RB RC 1 00
    ACC, //00_01 RA RB RC 1 10
    ACZ, //00_01 RA RB RC 1 01
    ACW, //00_01 RA RB RC 1 11
    ADI, //00 RA RB IMM6
    NDU, //00_10 RA RB RC 0 00
    NDC, //00_10 RA RB RC 0 10
    NDZ, //00_10 RA RB RC 0 01
    NCU, //00_10 RA RB RC 1 00
    NCC, //00_10 RA RB RC 1 10
    NCZ, //00_10 RA RB RC 1 01
    LLI, //00_11 RA IMM9
    LW,  //01_00 RA RB IMM6
    SW,  //01_01 RA RB IMM6
    LM,  //01_10 RA 0 + 8 bits corresponding to R0 to R7
    SM,  //01_11 RA 0 + 8 bits corresponding to R0 to R7
    BEQ, //10_00 RA RB IMM6
    BLT, //10_01 RA RB IMM6
    BLE, //10_10 RA RB IMM6
    JAL, //11_00 RA IMM9
    JLR, //11_01 RA 0 0000
    JRI, //11_11 RA 0 0000
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}
