mod lexer;
mod parser;

use crate::lexer::{Lexer, Processor, Token, TokenStream};
use crate::parser::Parser;

fn main() {
    let sample =
        "ADA R1 , R2 , R3 \n Main: AWC R1 , R2 , R3 \n MAIN3:NCC R1 , R2 , R4  // comments Version2";
    //let sample = "    \n \n ;comments \n";
    let mut token_stream = TokenStream::new();
    let mut lexer = Lexer::new(sample);
    // let mut tokens = Vec::new();

    // loop {
    //     let token = lexer.next_token(Processor::Pipelined);
    //     tokens.push(token.clone());
    //     token_stream.add(token.clone());
    //     //println!("      {:?}", token);
    //     if token == Token::EOF {
    //         break;
    //     }
    // }
    //
    let mut parser = Parser::new(sample);
    println!("[PARSING RESULT] Errors: {:?}", parser.parse().err());
    println!(
        "\n
        Instructions:           {:?} \n
        Labels:                 {:?}\n
        TokenStream:            {:?}",
        parser.instructions, parser.labels, parser.token_stream
    );
}
