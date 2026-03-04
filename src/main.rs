mod char_reader;
mod encoded;
mod utf8;

mod token;
pub(crate) use token::*;

use logos::Logos;

fn main() {
    let mut lexer = Token::lexer("let mebe firstName lastName");
    while let (Some(Ok(tok)), span, slice) = (lexer.next(), lexer.span(), lexer.slice()) {
        println!("{:?} {:?} {:?}", tok, span, slice);
    }
}
