use crate::Expression;
use crate::Operator;

#[derive(Debug)]
pub struct UnaryExpression {
    operand: Box<Expression>,
    operator: Operator,
}

impl UnaryExpression {
    pub fn new(operand: Expression, operator: Operator) -> Self {
        let operand = Box::new(operand);
        return Self { operand, operator };
    }

    pub fn operand(&self) -> &Expression {
        &self.operand
    }

    pub fn operator(&self) -> &Operator {
        &self.operator
    }
}
