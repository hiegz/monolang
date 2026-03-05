use crate::Number;
use crate::UnaryExpression;

#[derive(Debug)]
pub enum Expression {
    NumericLiteral(Number),
    Unary(UnaryExpression),
}
