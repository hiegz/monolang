use crate::Literal;
use crate::UnaryExpression;

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Unary(UnaryExpression),
}
