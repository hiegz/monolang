mod char_reader;
mod encoded;
mod utf8;

mod token;
pub(crate) use token::*;
mod expression;
pub(crate) use expression::*;
mod unary_expression;
pub(crate) use unary_expression::*;
mod operator;
pub(crate) use operator::*;
mod literal;
pub(crate) use literal::*;
mod numeric_literal;
pub(crate) use numeric_literal::*;
mod numeric_type;
pub(crate) use numeric_type::*;
mod parser;
pub(crate) use parser::*;

use logos::Logos;

fn main() {
    let mut lexer = Token::lexer("let mebe firstName lastName");
    while let (Some(Ok(tok)), span, slice) = (lexer.next(), lexer.span(), lexer.slice()) {
        println!("{:?} {:?} {:?}", tok, span, slice);
    }
}
