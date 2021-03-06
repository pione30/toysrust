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
    PrintLn {
        expression: Box<Expression>,
    },
}

pub fn add(lhs: Expression, rhs: Expression) -> Expression {
    Expression::Binary {
        operator: Operator::Add,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

pub fn subtract(lhs: Expression, rhs: Expression) -> Expression {
    Expression::Binary {
        operator: Operator::Subtract,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

pub fn multiply(lhs: Expression, rhs: Expression) -> Expression {
    Expression::Binary {
        operator: Operator::Multiply,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

pub fn divide(lhs: Expression, rhs: Expression) -> Expression {
    Expression::Binary {
        operator: Operator::Divide,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

pub fn less_than(lhs: Expression, rhs: Expression) -> Expression {
    Expression::Binary {
        operator: Operator::LessThan,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

pub fn less_or_equal(lhs: Expression, rhs: Expression) -> Expression {
    Expression::Binary {
        operator: Operator::LessOrEqual,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

pub fn greater_than(lhs: Expression, rhs: Expression) -> Expression {
    Expression::Binary {
        operator: Operator::GreaterThan,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

pub fn greater_or_equal(lhs: Expression, rhs: Expression) -> Expression {
    Expression::Binary {
        operator: Operator::GreaterOrEqual,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

pub fn equal_equal(lhs: Expression, rhs: Expression) -> Expression {
    Expression::Binary {
        operator: Operator::EqualEqual,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

pub fn not_equal(lhs: Expression, rhs: Expression) -> Expression {
    Expression::Binary {
        operator: Operator::NotEqual,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

pub fn integer(value: i64) -> Expression {
    Expression::IntegerLiteral { value }
}

pub fn identifier(name: &str) -> Expression {
    Expression::Identifier { name: name.into() }
}

pub fn assignment(name: &str, expression: Expression) -> Expression {
    Expression::Assignment {
        name: name.into(),
        expression: Box::new(expression),
    }
}

pub fn block(elements: Vec<Expression>) -> Expression {
    Expression::Block { elements }
}

pub fn ast_while(condition: Expression, body: Expression) -> Expression {
    Expression::While {
        condition: Box::new(condition),
        body: Box::new(body),
    }
}

pub fn ast_if(
    condition: Expression,
    then_clause: Expression,
    else_clause: Option<Expression>,
) -> Expression {
    Expression::If {
        condition: Box::new(condition),
        then_clause: Box::new(then_clause),
        else_clause: else_clause.map(Box::new),
    }
}

pub fn call(name: &str, args: Vec<Expression>) -> Expression {
    Expression::FunctionCall {
        name: name.to_string(),
        args,
    }
}

pub fn ast_println(expression: Expression) -> Expression {
    Expression::PrintLn {
        expression: Box::new(expression),
    }
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

pub fn define_function(name: &str, args: &[&str], body: Expression) -> TopLevel {
    TopLevel::FunctionDefinition(Function {
        name: name.to_string(),
        args: args.iter().map(|arg| arg.to_string()).collect(),
        body,
    })
}

pub fn difine_global_variable(name: &str, expression: Expression) -> TopLevel {
    TopLevel::GlobalVariableDefinition {
        name: name.to_string(),
        expression,
    }
}

pub struct Program {
    pub definitions: Vec<TopLevel>,
}
