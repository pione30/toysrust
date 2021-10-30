use crate::ast;

pub fn interpret(expression: &ast::Expression) -> i64 {
    match expression {
        ast::Expression::BinaryExpression { operator, lhs, rhs } => {
            let lhs = interpret(lhs);
            let rhs = interpret(rhs);

            match operator {
                ast::Operator::Add => lhs + rhs,
                ast::Operator::Subtract => lhs - rhs,
                ast::Operator::Multiply => lhs * rhs,
                ast::Operator::Divide => lhs / rhs,
            }
        }
        ast::Expression::IntegerLiteral { value } => *value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10_plus_20_is_30() {
        let expression = ast::add(&ast::integer(10), &ast::integer(20));
        assert_eq!(interpret(&expression), 30);
    }

    #[test]
    fn test_30_minus_20_is_10() {
        let expression = ast::subtract(&ast::integer(30), &ast::integer(20));
        assert_eq!(interpret(&expression), 10);
    }

    #[test]
    fn test_10_multiplies_20_is_200() {
        let expression = ast::multiply(&ast::integer(10), &ast::integer(20));
        assert_eq!(interpret(&expression), 200);
    }

    #[test]
    fn test_200_divided_by_20_is_10() {
        let expression = ast::divide(&ast::integer(200), &ast::integer(20));
        assert_eq!(interpret(&expression), 10);
    }

    #[test]
    fn test_42_is_42() {
        let expression = ast::integer(42);
        assert_eq!(interpret(&expression), 42);
    }
}
