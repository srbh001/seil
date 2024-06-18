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

use crate::lexer::{
    Lexer, Processor, Register, Token, TokenStream, INSTRUCTION_PIPELINED, INSTRUCTION_SINGLE_CYCLE,
};

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: String,
    pub reg_a: Register,
    pub reg_b: Option<Register>, // optional
    pub reg_c: Option<Register>,
    pub imm: i32,
    pub line_number: usize,
    pub processor: Processor, //use this to determine the type of instruction
}

impl Instruction {
    pub fn new(
        opcode: String,
        reg_a: Register,
        reg_b: Option<Register>,
        reg_c: Option<Register>,
        imm: i32,
        line_number: usize,
        processor: Processor,
    ) -> Instruction {
        Instruction {
            opcode,
            reg_a,
            reg_b,
            reg_c,
            imm,
            line_number,
            processor,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parser {
    pub token_stream: TokenStream,
    pub lexer: Lexer,                        // contains the original sample
    pub instructions: Vec<Vec<Instruction>>, // the final program - contains labels separated instructions
    pub labels: Vec<String>,
    // contains the labels
    // Example:
    // MAIN: ADI R1, R2, 10 // I1
    //       ADC R1, R2, R3 // I2
    //       ADI R1, R2, 10 // I3
    //       RET
    // NEXT: ADI R1, R2, 10 // I4
    //       ADC R1, R2, R3 // I5
    //       ADI R1, R2, 10 // I6
    //       RET
    // The above program will be stored as:
    // instructions = [[I1, I2, I3], [I4, I5, I6]]
    // labels = [MAIN, NEXT]
}

#[derive(Debug, Clone)]
pub struct ParserError {
    pub message: String,
    pub line_number: usize,
    pub column_number: usize,
}

impl Parser {
    pub fn new(sample: &str) -> Parser {
        let mut token_stream = TokenStream::new();
        let mut lexer = Lexer::new(sample);
        let mut instructions = Vec::new();
        let mut labels = Vec::new();

        loop {
            let token = lexer.next_token(Processor::Pipelined);
            token_stream.add(token.clone());
            if let Token::Label(label) = token {
                labels.push(label);
            } else if token == Token::EOF {
                break;
            }
        }
        Parser {
            token_stream,
            lexer,
            instructions,
            labels,
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction, label_count: usize) {
        while self.instructions.len() <= label_count {
            self.instructions.push(Vec::new());
        }
        self.instructions[label_count].push(instruction);

        // if self.instructions.len() <= 0 {
        //     self.instructions.push(Vec::new());
        // } else {
        //     let last = self.instructions.len() - 1;
        //     self.instructions[last].push(instruction);
        // }
    }

    pub fn parse(&mut self) -> Result<(Self), ParserError> {
        // fields for instruction

        //let  mut instructioin:Instruction = Instruction::new(opcode, reg_a, reg_b, reg_c, imm, line_number, processor);

        // fields for parser
        let mut processor = Processor::Pipelined;
        let mut line = 0;
        let mut position = 0;
        let mut line_number = 0;

        // Data for the instruction
        let opcodes_with_three_register_pipelined = vec![
            "ADA", //00_01 RA RB RC 0 00
            "ADC", //00_01 RA RB RC 0 10
            "ADZ", //00_01 RA RB RC 0 01
            "AWC", //00_01 RA RB RC 0 11
            "ACA", //00_01 RA RB RC 1 00
            "ACC", //00_01 RA RB RC 1 10
            "ACZ", //00_01 RA RB RC 1 01
            "ACW", //00_01 RA RB RC 1 11
            "NDU", //00_10 RA RB RC 0 00
            "NDC", //00_10 RA RB RC 0 10
            "NDZ", //00_10 RA RB RC 0 01
            "NCU", //00_10 RA RB RC 1 00
            "NCC", //00_10 RA RB RC 1 10
            "NCZ", //00_10 RA RB RC 1 01
        ];

        let opcodes_with_two_register_pipelined = vec![
            "ADI", //00 RA RB IMM6
            "LLI", //00_11 RA IMM9
            "LW",  //01_00 RA RB IMM6
            "SW",  //01_01 RA RB IMM6
            "BEQ", //10_00 RA RB IMM6
            "BLT", //10_01 RA RB IMM6
            "BLE", //10_10 RA RB IMM6
        ];

        let mut opcodes_with_single_register_pipelined = vec![
            "LLI", //00_11 RA IMM9
            "LM",  //01_10 RA 0 + 8 bits corresponding to R0 to R7
            "SM",  //01_11 RA 0 + 8 bits corresponding to R0 to R7
            "JAL", //11_00 RA IMM9
            "JLR", //11_01 RA 0 0000
            "JRI", //11_11 RA 0 0000
        ];

        let mut instructions_to_add = Vec::new();
        let mut label_count = 0;

        for (line_number, token_by_lines) in self.token_stream.tokens_by_line.iter().enumerate() {
            let mut token_position: usize = 0;

            for (position, token) in token_by_lines.iter().enumerate() {
                token_position = position;

                match token {
                    Token::Label(_) => {
                        if position != 0 {
                            return Err(ParserError {
                                message: "Label must be at the beginning of the line".to_string(),
                                line_number: line_number + 1,
                                column_number: position + 1,
                            });
                        }
                        label_count += 1;
                    }
                    Token::Opcode(opcode) => {
                        let mut reg_a = Register::R0;
                        let mut reg_b = Register::R0;
                        let mut reg_c = Register::R0;
                        let mut imm = 0;

                        if opcodes_with_three_register_pipelined.contains(&opcode.as_str()) {
                            if let Some(Token::Register(reg)) =
                                token_by_lines.get(token_position + 1)
                            {
                                reg_a = *reg;
                            } else {
                                return Err(ParserError {
                                    message: "Expected register".to_string(),
                                    line_number: line_number + 1,
                                    column_number: position + 2,
                                });
                            }

                            if let Some(Token::Comma) = token_by_lines.get(token_position + 2) {
                                // Expected comma, continue
                            } else {
                                return Err(ParserError {
                                    message: "Expected comma".to_string(),
                                    line_number: line_number + 1,
                                    column_number: position + 3,
                                });
                            }

                            if let Some(Token::Register(reg)) =
                                token_by_lines.get(token_position + 3)
                            {
                                reg_b = *reg;
                            } else {
                                return Err(ParserError {
                                    message: "Expected register".to_string(),
                                    line_number: line_number + 1,
                                    column_number: position + 4,
                                });
                            }

                            if let Some(Token::Comma) = token_by_lines.get(token_position + 4) {
                                // Expected comma, continue
                            } else {
                                return Err(ParserError {
                                    message: "Expected comma".to_string(),
                                    line_number: line_number + 1,
                                    column_number: position + 5,
                                });
                            }

                            if let Some(Token::Register(reg)) =
                                token_by_lines.get(token_position + 5)
                            {
                                reg_c = *reg;
                            } else {
                                return Err(ParserError {
                                    message: "Expected register".to_string(),
                                    line_number: line_number + 1,
                                    column_number: position + 6,
                                });
                            }

                            let instruction = Instruction::new(
                                opcode.clone(),
                                reg_a,
                                Some(reg_b),
                                Some(reg_c),
                                imm,
                                line_number + 1,
                                Processor::Pipelined,
                            );
                            instructions_to_add.push((instruction, label_count));
                        } else if opcodes_with_two_register_pipelined.contains(&opcode.as_str()) {
                            if let Some(Token::Register(reg)) =
                                token_by_lines.get(token_position + 1)
                            {
                                reg_a = *reg;
                            } else {
                                return Err(ParserError {
                                    message: "Expected register".to_string(),
                                    line_number: line_number + 1,
                                    column_number: position + 2,
                                });
                            }

                            if let Some(Token::Comma) = token_by_lines.get(token_position + 2) {
                                // Expected comma, continue
                            } else {
                                return Err(ParserError {
                                    message: "Expected comma".to_string(),
                                    line_number: line_number + 1,
                                    column_number: position + 3,
                                });
                            }

                            if let Some(Token::Register(reg)) =
                                token_by_lines.get(token_position + 3)
                            {
                                reg_b = *reg;
                            } else {
                                return Err(ParserError {
                                    message: "Expected register".to_string(),
                                    line_number: line_number + 1,
                                    column_number: position + 4,
                                });
                            }

                            if let Some(Token::Number(num)) = token_by_lines.get(token_position + 4)
                            {
                                imm = *num;
                            } else {
                                return Err(ParserError {
                                    message: "Expected immediate".to_string(),
                                    line_number: line_number + 1,
                                    column_number: position + 5,
                                });
                            }

                            let instruction = Instruction::new(
                                opcode.clone(),
                                reg_a,
                                Some(reg_b),
                                None,
                                imm,
                                line_number + 1,
                                Processor::Pipelined,
                            );
                            instructions_to_add.push((instruction, label_count));
                        } else if opcodes_with_single_register_pipelined.contains(&opcode.as_str())
                        {
                            if let Some(Token::Register(reg)) =
                                token_by_lines.get(token_position + 1)
                            {
                                reg_a = *reg;
                            } else {
                                return Err(ParserError {
                                    message: "Expected register".to_string(),
                                    line_number: line_number + 1,
                                    column_number: position + 2,
                                });
                            }

                            if let Some(Token::Number(num)) = token_by_lines.get(token_position + 2)
                            {
                                imm = *num;
                            } else {
                                return Err(ParserError {
                                    message: "Expected immediate".to_string(),
                                    line_number: line_number + 1,
                                    column_number: position + 3,
                                });
                            }

                            let instruction = Instruction::new(
                                opcode.clone(),
                                reg_a,
                                None,
                                None,
                                imm,
                                line_number + 1,
                                Processor::Pipelined,
                            );
                            instructions_to_add.push((instruction, label_count));
                        } else {
                            return Err(ParserError {
                                message: format!("Invalid opcode: {}", opcode),
                                line_number: line_number + 1,
                                column_number: position + 1,
                            });
                        }
                    }
                    Token::EOF => break,
                    _ => {
                        continue;
                    }
                }
            }
        }

        // Add instructions after processing all tokens
        println!("HERE INSTRUCTIONS TO ADD : {:?} \n\n", instructions_to_add);
        for (instruction, label_count) in instructions_to_add {
            self.add_instruction(instruction, label_count);
        }

        Ok(self.clone())
    }
}
