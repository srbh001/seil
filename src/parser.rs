// A parser for the Assembly Language for the IITB RISC-V Processor.
//
// This parser supports:
// - 24 instructions for the Pipelined Architecture
// - 14 instructions for the Single Cycle Architecture
//
// The ISA was developed by Prof. Virendra Singh, IIT Bombay.
// For more information, see <https://www.ee.iitb.ac.in/~viren/>.
// The ISA is based on the RISC-V ISA and has been modified to suit the needs of the EE309 and EE224 courses at IIT Bombay.
//
// Author: Saurabh <saurabhkumarnomeas@gmail.com>
//

use crate::lexer::{
    Lexer, Processor, Register, Token, TokenStream, INSTRUCTION_PIPELINED, INSTRUCTION_SINGLE_CYCLE,
};

pub struct Instruction {
    pub instruction: String,
    pub rd: Register,
    pub rs1: Register,
    pub rs2: Register,
    pub imm: i32,
    pub line_number: i32,
    pub processor: Processor, //use this to determine the type of instruction
}

impl Instruction {
    pub fn new(
        instruction: String,
        rd: Register,
        rs1: Register,
        rs2: Register,
        imm: i32,
        line_number: i32,
        processor: Processor,
    ) -> Instruction {
        Instruction {
            instruction,
            rd,
            rs1,
            rs2,
            imm,
            line_number,
            processor,
        }
    }
}
