mod lexer;
mod parser;

use crate::lexer::{Lexer, Processor, Token, TokenStream};
use crate::parser::{parse, Instruction};

fn main() {
    println!("Hello, world!");
    let sample = "; comments like this are ignored\n ADI R1 , R2 , R3 \n Main: ADC R1 , R2 , R3 \n ADI R1 , R2 , 10";
    let mut token_stream = TokenStream::new();
    let mut lexer = Lexer::new(sample);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token(Processor::Pipelined);
        tokens.push(token.clone());
        println!("{:?}", token);
        if token == Token::EOF {
            break;
        }
    }
}
