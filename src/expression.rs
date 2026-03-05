use crate::Operator;

#[derive(Debug)]
pub enum Expression {
    Signed8(i8),
    Signed16(i16),
    Signed32(i32),
    Signed64(i64),
    Unsigned8(u8),
    Unsigned16(u16),
    Unsigned32(u32),
    Unsigned64(u64),

    Unary(Box<Expression>, Operator),
}

struct UnaryExpression;
#[rustfmt::skip]
impl   UnaryExpression {
    pub fn new(operand: Expression, operator: Operator) -> Expression {
        Expression::Unary(Box::new(operand), operator)
    }
}
