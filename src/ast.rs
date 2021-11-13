#[derive(Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
    EqualEqual,
    NotEqual,
}

#[derive(Clone)]
pub enum Expression {
    Binary {
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
    Block {
        elements: Vec<Expression>,
    },
    While {
        condition: Box<Expression>,
        body: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        then_clause: Box<Expression>,
        else_clause: Option<Box<Expression>>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
}

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub body: Expression,
}

pub enum TopLevel {
    FunctionDefinition(Function),
    GlobalVariableDefinition {
        name: String,
        expression: Expression,
    },
}

pub struct Program {
    pub definitions: Vec<TopLevel>,
}

pub fn add(lhs: &Expression, rhs: &Expression) -> Expression {
    Expression::Binary {
        operator: Operator::Add,
        lhs: Box::new(lhs.to_owned()),
        rhs: Box::new(rhs.to_owned()),
    }
}

pub fn subtract(lhs: &Expression, rhs: &Expression) -> Expression {
    Expression::Binary {
        operator: Operator::Subtract,
        lhs: Box::new(lhs.to_owned()),
        rhs: Box::new(rhs.to_owned()),
    }
}

pub fn multiply(lhs: &Expression, rhs: &Expression) -> Expression {
    Expression::Binary {
        operator: Operator::Multiply,
        lhs: Box::new(lhs.to_owned()),
        rhs: Box::new(rhs.to_owned()),
    }
}

pub fn divide(lhs: &Expression, rhs: &Expression) -> Expression {
    Expression::Binary {
        operator: Operator::Divide,
        lhs: Box::new(lhs.to_owned()),
        rhs: Box::new(rhs.to_owned()),
    }
}

pub fn binary(operator: &Operator, lhs: &Expression, rhs: &Expression) -> Expression {
    Expression::Binary {
        operator: operator.to_owned(),
        lhs: Box::new(lhs.to_owned()),
        rhs: Box::new(rhs.to_owned()),
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
        expression: Box::new(expression.to_owned()),
    }
}

pub fn block(elements: &[Expression]) -> Expression {
    Expression::Block {
        elements: elements.to_vec(),
    }
}

pub fn ast_while(condition: &Expression, body: &Expression) -> Expression {
    Expression::While {
        condition: Box::new(condition.to_owned()),
        body: Box::new(body.to_owned()),
    }
}

pub fn ast_if(
    condition: &Expression,
    then_clause: &Expression,
    else_clause: &Option<Expression>,
) -> Expression {
    Expression::If {
        condition: Box::new(condition.to_owned()),
        then_clause: Box::new(then_clause.to_owned()),
        else_clause: else_clause
            .as_ref()
            .map(|expression| Box::new(expression.to_owned())),
    }
}
