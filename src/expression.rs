use crate::Operator;

#[derive(Debug)]
pub enum Expression {
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
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
