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
            ast::Expression::Binary { operator, lhs, rhs } => {
                let lhs = self.interpret(lhs);
                let rhs = self.interpret(rhs);

                match operator {
                    ast::Operator::Add => lhs + rhs,
                    ast::Operator::Subtract => lhs - rhs,
                    ast::Operator::Multiply => lhs * rhs,
                    ast::Operator::Divide => lhs / rhs,
                    ast::Operator::LessThan => {
                        if lhs < rhs {
                            1
                        } else {
                            0
                        }
                    }
                    ast::Operator::LessOrEqual => {
                        if lhs <= rhs {
                            1
                        } else {
                            0
                        }
                    }
                    ast::Operator::GreaterThan => {
                        if lhs > rhs {
                            1
                        } else {
                            0
                        }
                    }
                    ast::Operator::GreaterOrEqual => {
                        if lhs >= rhs {
                            1
                        } else {
                            0
                        }
                    }
                    ast::Operator::EqualEqual => {
                        if lhs == rhs {
                            1
                        } else {
                            0
                        }
                    }
                    ast::Operator::NotEqual => {
                        if lhs != rhs {
                            1
                        } else {
                            0
                        }
                    }
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
            ast::Expression::Block { elements } => elements
                .iter()
                .fold(0, |_, element| self.interpret(element)),
            ast::Expression::While { condition, body } => {
                loop {
                    let condition = self.interpret(condition);
                    if condition != 0 {
                        self.interpret(body);
                    } else {
                        break;
                    }
                }

                1
            }
            ast::Expression::If {
                condition,
                then_clause,
                else_clause,
            } => {
                let condition = self.interpret(condition);
                if condition != 0 {
                    self.interpret(then_clause)
                } else {
                    else_clause
                        .as_ref()
                        .map(|expression| self.interpret(expression))
                        .unwrap_or(1)
                }
            }
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

    #[test]
    fn if_then() {
        let mut interpreter = Interpreter::new();

        let condition = ast::binary(&ast::Operator::LessThan, &ast::integer(2), &ast::integer(4));

        let expression = ast::ast_if(&condition, &ast::integer(42), &None);
        assert_eq!(interpreter.interpret(&expression), 42);
    }

    #[test]
    fn if_then_else() {
        let mut interpreter = Interpreter::new();

        let condition = ast::binary(
            &ast::Operator::GreaterThan,
            &ast::integer(2),
            &ast::integer(4),
        );

        let expression = ast::ast_if(&condition, &ast::integer(42), &Some(ast::integer(53)));
        assert_eq!(interpreter.interpret(&expression), 53);
    }

    #[test]
    fn block() {
        let mut interpreter = Interpreter::new();

        let elements = [ast::integer(1), ast::integer(2), ast::integer(3)];

        let expression = ast::block(&elements);
        assert_eq!(interpreter.interpret(&expression), 3);
    }
}
