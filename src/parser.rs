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
    InstructionPipelined, InstructionSingleCycle, Lexer, Processor, Register, Token, TokenStream,
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

pub fn parse(tokens: Vec<Token>, processor: Processor) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut token_stream = TokenStream {
        tokens,
        position: 0,
    };

    while token_stream.position < token_stream.tokens.len() {
        let instruction = parse_instruction(&token_stream, processor.clone());
        instructions.push(instruction);
    }

    instructions
}

pub fn parse_instruction(token_stream: &TokenStream, processor: Processor) -> Instruction {
    Instruction {
        instruction: String::from(""),
        rd: Register::R1,
        rs1: Register::R1,
        rs2: Register::R1,
        imm: 0,
        line_number: 0,
        processor,
    }
}
