#[derive(Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone)]
pub enum Expression {
    BinaryExpression {
        operator: Operator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    IntegerLiteral {
        value: i64,
    },
    Identifier {
        name: String,
    },
    Assignment {
        name: String,
        expression: Box<Expression>,
    },
}

pub fn add(lhs: &Expression, rhs: &Expression) -> Expression {
    Expression::BinaryExpression {
        operator: Operator::Add,
        lhs: Box::new(lhs.clone()),
        rhs: Box::new(rhs.clone()),
    }
}

pub fn subtract(lhs: &Expression, rhs: &Expression) -> Expression {
    Expression::BinaryExpression {
        operator: Operator::Subtract,
        lhs: Box::new(lhs.clone()),
        rhs: Box::new(rhs.clone()),
    }
}

pub fn multiply(lhs: &Expression, rhs: &Expression) -> Expression {
    Expression::BinaryExpression {
        operator: Operator::Multiply,
        lhs: Box::new(lhs.clone()),
        rhs: Box::new(rhs.clone()),
    }
}

pub fn divide(lhs: &Expression, rhs: &Expression) -> Expression {
    Expression::BinaryExpression {
        operator: Operator::Divide,
        lhs: Box::new(lhs.clone()),
        rhs: Box::new(rhs.clone()),
    }
}

pub fn integer(value: i64) -> Expression {
    Expression::IntegerLiteral { value }
}

pub fn identifier(name: &str) -> Expression {
    Expression::Identifier { name: name.into() }
}

pub fn assignment(name: &str, expression: &Expression) -> Expression {
    Expression::Assignment {
        name: name.into(),
        expression: Box::new(expression.clone()),
    }
}
