mod lexer;
mod parser;

use crate::lexer::{Lexer, Processor, Token, TokenStream};
// use crate::parser::Instruction;

fn main() {
    println!("Hello, world!");
    let sample = "; comments like this are ignored\n ADI R1 , R2 , R3 \nMain: ADC R1 , R2 , R3 \nADI R1 , R2 , 10 // comments Version2";
    //let sample = "    \n \n ;comments \n";
    let mut token_stream = TokenStream::new();
    let mut lexer = Lexer::new(sample);
    let mut tokens = Vec::new();
    println!("Tokens Parsed :");

    loop {
        let token = lexer.next_token(Processor::Pipelined);
        tokens.push(token.clone());
        token_stream.add(token.clone());
        //println!("      {:?}", token);
        if token == Token::EOF {
            break;
        }
    }
    println!("{:?}", token_stream);
}
