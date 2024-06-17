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

#![allow(dead_code)]
// TODO: Remove this line after implementing the code

#[derive(Debug, PartialEq, Clone)]
pub enum Processor {
    SingleCycle,
    Pipelined,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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
    InstrSingle(InstructionSingleCycle), // Reserved keywords to be used only for instructions - mentioned in the ISA
    InstrPipe(InstructionPipelined), // Reserved keywords to be used only for instructions - mentioned in the ISA
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

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    IdentifierSingle(InstructionSingleCycle), // a symbol(opcode),
    IdentifierPipelined(InstructionPipelined), // a symbol(opcode), // only one of these will be used
    Label(String),
    Number(i32),
    Register(Register),
    Comment(String),
    EOF,
    Error(String), // for unknown tokens
    Comma,
}

// #[derive(Debug, PartialEq)]
// pub enum TokenPipelined {
//     Identifier(InstructionPipelined), // a symbol(opcode),
//     Label(String),
//     Number(i32),
//     Register(Register),
//     Comment(String),
//     EOF,
//     Error(String), // for unknown tokens
// }

#[derive(Debug, PartialEq)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            // println!("{ch}");
            self.position += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.next_char() {
            if (ch.is_whitespace() || ch == '\n') {
                self.next_char();
                self.position -= 1;
                // println!("This is the problem");
            } else {
                self.position -= 1;
                //println!("This is the problem {:?}", self.peek_char());

                break;
            }
        }
    }

    fn read_number(&mut self, first_digit: char) -> i32 {
        let mut number = first_digit.to_digit(10).unwrap() as i32;
        while let Some(ch) = self.peek_char() {
            if ch.is_digit(10) {
                number = number * 10 + ch.to_digit(10).unwrap() as i32;
                self.next_char();
            } else {
                break;
            }
        }
        number
    }

    fn read_identifier(&mut self, first_char: char) -> String {
        let mut identifier = String::new();
        identifier.push(first_char);

        while let Some(ch) = self.peek_char() {
            if ch.is_alphanumeric() || ch == '_' {
                // println!("{ch}");
                identifier.push(ch);
                self.next_char();
            } else {
                if ch.is_whitespace() {
                    let position = self.position;
                    self.skip_whitespace();
                    if let Some(ch) = self.peek_char() {
                        if ch == ':' {
                            identifier.push(ch);
                            self.next_char();
                            //self.next_char();
                            // println!("This is the problem");
                            break;
                        }
                    } else {
                        //self.position = position + 3;
                        break;
                    }
                } else if ch == ':' {
                    identifier.push(ch);
                    self.next_char();
                    break;
                }
                self.position -= 1;
                // println!("ThIS is the problem");
                break;
            }
        }

        identifier
    }

    fn peek_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    pub fn next_token(&mut self, processor: Processor) -> Token {
        self.skip_whitespace();
        match self.next_char() {
            Some(ch) => {
                if ch.is_digit(10) {
                    let number = self.read_number(ch);
                    Token::Number(number)
                } else if ch.is_alphabetic() {
                    let identifier = self.read_identifier(ch);
                    if identifier.ends_with(':') {
                        Token::Label(identifier)
                    } else {
                        match identifier.to_uppercase().as_str() {
                            "R1" => Token::Register(Register::R1),
                            "R2" => Token::Register(Register::R2),
                            "R3" => Token::Register(Register::R3),
                            "R4" => Token::Register(Register::R4),
                            "R5" => Token::Register(Register::R5),
                            "R6" => Token::Register(Register::R6),
                            "R7" => Token::Register(Register::R7),
                            "R0" => Token::Register(Register::R0),

                            _ => {
                                if matches!(processor, Processor::SingleCycle) {
                                    match identifier.to_uppercase().as_str() {
                                        "ADD" => {
                                            Token::IdentifierSingle(InstructionSingleCycle::ADD)
                                        }
                                        "SUB" => {
                                            Token::IdentifierSingle(InstructionSingleCycle::SUB)
                                        }
                                        "MUL" => {
                                            Token::IdentifierSingle(InstructionSingleCycle::MUL)
                                        }
                                        "ADI" => {
                                            Token::IdentifierSingle(InstructionSingleCycle::ADI)
                                        }
                                        "AND" => {
                                            Token::IdentifierSingle(InstructionSingleCycle::AND)
                                        }
                                        "ORA" => {
                                            Token::IdentifierSingle(InstructionSingleCycle::ORA)
                                        }
                                        "IMP" => {
                                            Token::IdentifierSingle(InstructionSingleCycle::IMP)
                                        }
                                        "LHI" => {
                                            Token::IdentifierSingle(InstructionSingleCycle::LHI)
                                        }
                                        "LLI" => {
                                            Token::IdentifierSingle(InstructionSingleCycle::LLI)
                                        }
                                        "LW" => Token::IdentifierSingle(InstructionSingleCycle::LW),
                                        "SW" => Token::IdentifierSingle(InstructionSingleCycle::SW),
                                        "BEQ" => {
                                            Token::IdentifierSingle(InstructionSingleCycle::BEQ)
                                        }
                                        "JAL" => {
                                            Token::IdentifierSingle(InstructionSingleCycle::JAL)
                                        }
                                        "JLR" => {
                                            Token::IdentifierSingle(InstructionSingleCycle::JLR)
                                        }
                                        _ => Token::Error(format!(
                                            "Unknown identifier: {}",
                                            identifier
                                        )),
                                    }
                                } else if matches!(processor, Processor::Pipelined) {
                                    match identifier.to_uppercase().as_str() {
                                        "ADA" => {
                                            Token::IdentifierPipelined(InstructionPipelined::ADA)
                                        }
                                        "ADC" => {
                                            Token::IdentifierPipelined(InstructionPipelined::ADC)
                                        }
                                        "ADZ" => {
                                            Token::IdentifierPipelined(InstructionPipelined::ADZ)
                                        }
                                        "AWC" => {
                                            Token::IdentifierPipelined(InstructionPipelined::AWC)
                                        }
                                        "ACA" => {
                                            Token::IdentifierPipelined(InstructionPipelined::ACA)
                                        }
                                        "ACC" => {
                                            Token::IdentifierPipelined(InstructionPipelined::ACC)
                                        }
                                        "ACZ" => {
                                            Token::IdentifierPipelined(InstructionPipelined::ACZ)
                                        }
                                        "ACW" => {
                                            Token::IdentifierPipelined(InstructionPipelined::ACW)
                                        }
                                        "ADI" => {
                                            Token::IdentifierPipelined(InstructionPipelined::ADI)
                                        }
                                        "NDU" => {
                                            Token::IdentifierPipelined(InstructionPipelined::NDU)
                                        }
                                        "NDC" => {
                                            Token::IdentifierPipelined(InstructionPipelined::NDC)
                                        }
                                        "NDZ" => {
                                            Token::IdentifierPipelined(InstructionPipelined::NDZ)
                                        }
                                        "NCU" => {
                                            Token::IdentifierPipelined(InstructionPipelined::NCU)
                                        }
                                        "NCC" => {
                                            Token::IdentifierPipelined(InstructionPipelined::NCC)
                                        }
                                        "NCZ" => {
                                            Token::IdentifierPipelined(InstructionPipelined::NCZ)
                                        }
                                        "LLI" => {
                                            Token::IdentifierPipelined(InstructionPipelined::LLI)
                                        }
                                        "LW" => {
                                            Token::IdentifierPipelined(InstructionPipelined::LW)
                                        }
                                        "SW" => {
                                            Token::IdentifierPipelined(InstructionPipelined::SW)
                                        }
                                        "LM" => {
                                            Token::IdentifierPipelined(InstructionPipelined::LM)
                                        }
                                        "SM" => {
                                            Token::IdentifierPipelined(InstructionPipelined::SM)
                                        }
                                        "BEQ" => {
                                            Token::IdentifierPipelined(InstructionPipelined::BEQ)
                                        }
                                        "BLT" => {
                                            Token::IdentifierPipelined(InstructionPipelined::BLT)
                                        }
                                        "BLE" => {
                                            Token::IdentifierPipelined(InstructionPipelined::BLE)
                                        }
                                        "JAL" => {
                                            Token::IdentifierPipelined(InstructionPipelined::JAL)
                                        }
                                        "JLR" => {
                                            Token::IdentifierPipelined(InstructionPipelined::JLR)
                                        }
                                        "JRI" => {
                                            Token::IdentifierPipelined(InstructionPipelined::JRI)
                                        }
                                        _ => Token::Error(format!(
                                            "Unknown identifier: {}",
                                            identifier
                                        )),
                                    }
                                } else {
                                    Token::Error(format!("Unknown processor: {:?}", processor))
                                }
                            }
                        }
                    }
                } else if ch == '/' && self.peek_char() == Some('/') {
                    self.position += 1; // skip the second '/'
                    let index = self.input[self.position..].iter().position(|&x| x == 'R');
                    if let Some(_index) = index {
                        let comment: String = self.input[self.position..self.position + _index]
                            .iter()
                            .collect();
                        self.position += _index;
                        Token::Comment(comment)
                    } else {
                        let comment: String = self.input[self.position..].iter().collect();
                        self.position += self.input.len();
                        Token::Comment(comment)
                    }
                    // let comment: String = self.input[self.position...index].iter().collect();

                    //self.position += self.input.len();

                    // Token::Comment(comment)
                } else if ch == ';' {
                    let comment: String = self.input[self.position..].iter().collect();
                    let index = self.input[self.position..].iter().position(|&x| x == '\n');
                    if let Some(_index) = index {
                        let comment: String = self.input[self.position..self.position + _index]
                            .iter()
                            .collect();
                        self.position += _index;
                        Token::Comment(comment)
                    } else {
                        let comment: String = self.input[self.position..].iter().collect();
                        self.position += self.input.len();
                        Token::Comment(comment)
                    }
                } else if ch == ',' {
                    self.position += 1;
                    Token::Comma
                } else {
                    Token::Error(format!("Unknown token: {}", ch))
                }
            }
            None => Token::EOF,
        }
    }
}

pub struct TokenStream {
    pub tokens: Vec<Token>,
    pub position: usize,
}

impl TokenStream {
    pub fn new() -> Self {
        TokenStream {
            tokens: Vec::new(),
            position: 0,
        }
    }

    pub fn from(tokens: Vec<Token>) -> Self {
        TokenStream {
            tokens,
            position: 0,
        }
    }

    pub fn add(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn next(&mut self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.position = 0;
    }

    pub fn peek(&self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            Some(&self.tokens[self.position])
        } else {
            None
        }
    }
}
