use std::collections::HashMap;

use crate::ast;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("This program doesn't have main() function")]
    MainNotPresent,
    #[error("Zero is an invalid denominator")]
    ZeroDivision,
    #[error("Variable {0} is not present in this environment")]
    VariableNotPresent(String),
    #[error("`else_clause` should not be None when the `if` condition is not met")]
    ElseClauseNoneUnderIfConditionNotMet,
    #[error("An argument for `{0}` is not provided")]
    NotEnoughArguments(String),
    #[error("Function {0} is not found")]
    FunctionNotFound(String),
}

#[derive(Clone)]
pub struct Interpreter {
    variable_environment: HashMap<String, i64>,
    function_environment: HashMap<String, ast::Function>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variable_environment: HashMap::new(),
            function_environment: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, expression: &ast::Expression) -> Result<i64, InterpreterError> {
        let value = match expression {
            ast::Expression::Binary { operator, lhs, rhs } => {
                let lhs = self.interpret(lhs)?;
                let rhs = self.interpret(rhs)?;

                match operator {
                    ast::Operator::Add => lhs + rhs,
                    ast::Operator::Subtract => lhs - rhs,
                    ast::Operator::Multiply => lhs * rhs,
                    ast::Operator::Divide => {
                        if rhs == 0 {
                            return Err(InterpreterError::ZeroDivision);
                        }

                        lhs / rhs
                    }
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
                .variable_environment
                .get(name)
                .ok_or_else(|| InterpreterError::VariableNotPresent(name.clone()))?,
            ast::Expression::Assignment { name, expression } => {
                let value = self.interpret(expression)?;
                self.variable_environment.insert(name.clone(), value);
                value
            }
            ast::Expression::Block { elements } => {
                let mut value = 0;
                for element in elements {
                    value = self.interpret(element)?;
                }
                value
            }
            ast::Expression::While { condition, body } => {
                loop {
                    let condition = self.interpret(condition)?;
                    if condition != 0 {
                        self.interpret(body)?;
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
                let condition = self.interpret(condition)?;
                if condition != 0 {
                    self.interpret(then_clause)?
                } else {
                    let expression = else_clause
                        .as_ref()
                        .ok_or(InterpreterError::ElseClauseNoneUnderIfConditionNotMet)?;

                    self.interpret(expression)?
                }
            }
            ast::Expression::FunctionCall { name, args } => {
                let backup_environments = self.clone();

                let definition = backup_environments
                    .function_environment
                    .get(name)
                    .ok_or_else(|| InterpreterError::FunctionNotFound(name.clone()))?;

                let mut args_iter = args.iter();

                // 関数呼び出し先では呼び出し元のローカル変数が見えないようにする
                for formal_param_name in &definition.args {
                    let actual_expression = args_iter.next().ok_or_else(|| {
                        InterpreterError::NotEnoughArguments(formal_param_name.clone())
                    })?;

                    let actual_value = self.interpret(actual_expression)?;

                    self.variable_environment
                        .insert(formal_param_name.clone(), actual_value);
                }
                let value = self.interpret(&definition.body)?;

                // 呼び出し先から返ったら変数環境も元に戻す
                self.variable_environment = backup_environments.variable_environment;

                value
            }
        };

        Ok(value)
    }

    pub fn call_main(&mut self, program: ast::Program) -> Result<i64, InterpreterError> {
        for top_level in program.definitions {
            match top_level {
                ast::TopLevel::FunctionDefinition(function) => {
                    self.function_environment
                        .insert(function.name.clone(), function);
                }
                ast::TopLevel::GlobalVariableDefinition { name, expression } => {
                    let value = self.interpret(&expression)?;
                    self.variable_environment.insert(name, value);
                }
            }
        }

        let main = self
            .function_environment
            .get("main")
            .cloned()
            .ok_or(InterpreterError::MainNotPresent)?;

        self.interpret(&main.body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10_plus_20_is_30() {
        let mut interpreter = Interpreter::new();
        let expression = ast::add(&ast::integer(10), &ast::integer(20));
        assert_eq!(interpreter.interpret(&expression).unwrap(), 30);
    }

    #[test]
    fn test_30_minus_20_is_10() {
        let mut interpreter = Interpreter::new();
        let expression = ast::subtract(&ast::integer(30), &ast::integer(20));
        assert_eq!(interpreter.interpret(&expression).unwrap(), 10);
    }

    #[test]
    fn test_10_multiplies_20_is_200() {
        let mut interpreter = Interpreter::new();
        let expression = ast::multiply(&ast::integer(10), &ast::integer(20));
        assert_eq!(interpreter.interpret(&expression).unwrap(), 200);
    }

    #[test]
    fn test_200_divided_by_20_is_10() {
        let mut interpreter = Interpreter::new();
        let expression = ast::divide(&ast::integer(200), &ast::integer(20));
        assert_eq!(interpreter.interpret(&expression).unwrap(), 10);
    }

    #[test]
    fn test_200_divided_by_0_is_error() {
        let mut interpreter = Interpreter::new();
        let expression = ast::divide(&ast::integer(200), &ast::integer(0));

        match interpreter.interpret(&expression).unwrap_err() {
            InterpreterError::ZeroDivision => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_42_is_42() {
        let mut interpreter = Interpreter::new();
        let expression = ast::integer(42);
        assert_eq!(interpreter.interpret(&expression).unwrap(), 42);
    }

    #[test]
    fn assign_and_identify() {
        let mut interpreter = Interpreter::new();

        let assignment = ast::assignment("foo", &ast::integer(42));
        assert_eq!(interpreter.interpret(&assignment).unwrap(), 42);

        let identifier = ast::identifier("foo");
        assert_eq!(interpreter.interpret(&identifier).unwrap(), 42);
    }

    #[test]
    fn if_then() {
        let mut interpreter = Interpreter::new();

        let condition = ast::binary(&ast::Operator::LessThan, &ast::integer(2), &ast::integer(4));

        let expression = ast::ast_if(&condition, &ast::integer(42), &None);
        assert_eq!(interpreter.interpret(&expression).unwrap(), 42);
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
        assert_eq!(interpreter.interpret(&expression).unwrap(), 53);
    }

    #[test]
    fn block() {
        let mut interpreter = Interpreter::new();

        let elements = [ast::integer(1), ast::integer(2), ast::integer(3)];

        let expression = ast::block(&elements);
        assert_eq!(interpreter.interpret(&expression).unwrap(), 3);
    }
}
