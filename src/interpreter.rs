use std::collections::HashMap;

use crate::ast;

pub struct Interpreter {
    environment: HashMap<String, i64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, expression: &ast::Expression) -> i64 {
        match expression {
            ast::Expression::BinaryExpression { operator, lhs, rhs } => {
                let lhs = self.interpret(lhs);
                let rhs = self.interpret(rhs);

                match operator {
                    ast::Operator::Add => lhs + rhs,
                    ast::Operator::Subtract => lhs - rhs,
                    ast::Operator::Multiply => lhs * rhs,
                    ast::Operator::Divide => lhs / rhs,
                }
            }
            ast::Expression::IntegerLiteral { value } => *value,
            ast::Expression::Identifier { name } => *self
                .environment
                .get(name)
                .unwrap_or_else(|| panic!("variable {} to exist in the environment", name)),
            ast::Expression::Assignment { name, expression } => {
                let value = self.interpret(expression);
                self.environment.insert(name.clone(), value);
                value
            }
            ast::Expression::BlockExpression { elements } => unimplemented!(),
            ast::Expression::WhileExpression { condition, body } => unimplemented!(),
            ast::Expression::IfExpression {
                condition,
                then_clause,
                else_clause,
            } => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10_plus_20_is_30() {
        let mut interpreter = Interpreter::new();
        let expression = ast::add(&ast::integer(10), &ast::integer(20));
        assert_eq!(interpreter.interpret(&expression), 30);
    }

    #[test]
    fn test_30_minus_20_is_10() {
        let mut interpreter = Interpreter::new();
        let expression = ast::subtract(&ast::integer(30), &ast::integer(20));
        assert_eq!(interpreter.interpret(&expression), 10);
    }

    #[test]
    fn test_10_multiplies_20_is_200() {
        let mut interpreter = Interpreter::new();
        let expression = ast::multiply(&ast::integer(10), &ast::integer(20));
        assert_eq!(interpreter.interpret(&expression), 200);
    }

    #[test]
    fn test_200_divided_by_20_is_10() {
        let mut interpreter = Interpreter::new();
        let expression = ast::divide(&ast::integer(200), &ast::integer(20));
        assert_eq!(interpreter.interpret(&expression), 10);
    }

    #[test]
    fn test_42_is_42() {
        let mut interpreter = Interpreter::new();
        let expression = ast::integer(42);
        assert_eq!(interpreter.interpret(&expression), 42);
    }

    #[test]
    fn assign_and_identify() {
        let mut interpreter = Interpreter::new();

        let assignment = ast::assignment("foo", &ast::integer(42));
        assert_eq!(interpreter.interpret(&assignment), 42);

        let identifier = ast::identifier("foo");
        assert_eq!(interpreter.interpret(&identifier), 42);
    }
}
