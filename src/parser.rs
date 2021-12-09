use crate::ast;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::opt,
    multi::{fold_many0, many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};

mod helper_combinators;
mod raw_res;

/// program <- top_level_definition*;
fn program(input: &str) -> IResult<&str, ast::Program> {
    let (input, definitions) = many0(top_level_definition)(input)?;

    Ok((input, ast::Program { definitions }))
}

/// top_level_definition <-
///     function_definition \
///     global_variable_definition;
fn top_level_definition(input: &str) -> IResult<&str, ast::TopLevel> {
    alt((function_definition, global_variable_definition))(input)
}

/// function_definition <-
///     "define" identifier
///     "(" (identifier ("," identifier)*)? ")"
///     block_expression
fn function_definition(input: &str) -> IResult<&str, ast::TopLevel> {
    let (input, _) = tag("define")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, name) = raw_res::identifier(input)?;
    let (input, _) = multispace0(input)?;

    let (input, args) = helper_combinators::parentheses(separated_list0(
        delimited(multispace0, tag(","), multispace0),
        raw_res::identifier,
    ))(input)?;
    let (input, _) = multispace0(input)?;

    let (input, body) = block_expression(input)?;

    Ok((input, ast::define_function(name, &args, body)))
}

/// global_variable_definition <-
///     "global" identifier "=" expression;
fn global_variable_definition(input: &str) -> IResult<&str, ast::TopLevel> {
    let (input, _) = tag("global")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = raw_res::identifier(input)?;
    let (input, _) = helper_combinators::ws(tag("="))(input)?;
    let (input, expression) = expression(input)?;

    Ok((input, ast::difine_global_variable(name, expression)))
}

/// line <-
///     println \
///     if_expression \
///     while_expression \
///     block_expression
///     assignment \
///     expression_line;
fn line(input: &str) -> IResult<&str, ast::Expression> {
    // *terminated multispace0* is important!
    terminated(
        alt((
            println,
            if_expression,
            while_expression,
            block_expression,
            assignment,
            expression_line,
        )),
        multispace0,
    )(input)
}

/// println <- "println" "(" expression ")";
fn println(input: &str) -> IResult<&str, ast::Expression> {
    let (input, _) = terminated(tag("println"), multispace0)(input)?;
    let (input, expression) = helper_combinators::parentheses(expression)(input)?;

    Ok((input, ast::ast_println(expression)))
}

/// if_expression <-
///     "if" "(" expression ")" line
///     ("else" line)?;
fn if_expression(input: &str) -> IResult<&str, ast::Expression> {
    let (input, _) = terminated(tag("if"), multispace0)(input)?;
    let (input, condition) = helper_combinators::parentheses(expression)(input)?;
    let (input, then_clause) = preceded(multispace0, line)(input)?;

    let (input, else_clause) = opt(preceded(helper_combinators::ws(tag("else")), line))(input)?;

    Ok((input, ast::ast_if(condition, then_clause, else_clause)))
}

/// while_expression <-
///     "while" "(" expression ")" line;
fn while_expression(input: &str) -> IResult<&str, ast::Expression> {
    let (input, _) = terminated(tag("while"), multispace0)(input)?;
    let (input, condition) = helper_combinators::parentheses(expression)(input)?;
    let (input, body) = preceded(multispace0, line)(input)?;

    Ok((input, ast::ast_while(condition, body)))
}

/// block_expression <- "{" line* "}";
fn block_expression(input: &str) -> IResult<&str, ast::Expression> {
    let (input, elements) = helper_combinators::curly_brackets(many0(line))(input)?;

    Ok((input, ast::block(elements)))
}

/// assignment <- identifier "=" expression ";";
fn assignment(input: &str) -> IResult<&str, ast::Expression> {
    let (input, name) = raw_res::identifier(input)?;
    let (input, _) = helper_combinators::ws(tag("="))(input)?;
    let (input, ast_expression) = terminated(expression, tag(";"))(input)?;

    Ok((input, ast::assignment(name, ast_expression)))
}

/// expression_line <- expression ";";
fn expression_line(input: &str) -> IResult<&str, ast::Expression> {
    terminated(expression, tag(";"))(input)
}

/// expression <- comparative;
fn expression(input: &str) -> IResult<&str, ast::Expression> {
    comparative(input)
}

/// comparative <- additive (
///     ("<" / ">" / "<=" / ">=" / "==" / "!=") additive
/// )*;
fn comparative(input: &str) -> IResult<&str, ast::Expression> {
    let (input, left_operand) = additive(input)?;

    let result = fold_many0(
        pair(
            helper_combinators::ws(alt((
                tag("<="),
                tag(">="),
                tag("=="),
                tag("!="),
                tag("<"),
                tag(">"),
            ))),
            additive,
        ),
        || left_operand.clone(),
        |acc, (operator, right_operand)| match operator {
            "<=" => ast::less_or_equal(acc, right_operand),
            ">=" => ast::greater_or_equal(acc, right_operand),
            "==" => ast::equal_equal(acc, right_operand),
            "!=" => ast::not_equal(acc, right_operand),
            "<" => ast::less_than(acc, right_operand),
            ">" => ast::greater_than(acc, right_operand),
            _ => unreachable!(),
        },
    )(input);

    result
}

/// additive <- multitive (
///     ("+" / "-") multitive
/// )*;
fn additive(input: &str) -> IResult<&str, ast::Expression> {
    let (input, left_operand) = multitive(input)?;

    let result = fold_many0(
        pair(helper_combinators::ws(alt((tag("+"), tag("-")))), multitive),
        || left_operand.clone(),
        |acc, (operator, right_operand)| match operator {
            "+" => ast::add(acc, right_operand),
            "-" => ast::subtract(acc, right_operand),
            _ => unreachable!(),
        },
    )(input);

    result
}

/// multitive <- primary (
///     ("*" / "/") primary
/// )*;
fn multitive(input: &str) -> IResult<&str, ast::Expression> {
    let (input, left_operand) = primary(input)?;

    let result = fold_many0(
        pair(helper_combinators::ws(alt((tag("*"), tag("/")))), primary),
        || left_operand.clone(),
        |acc, (operator, right_operand)| match operator {
            "*" => ast::multiply(acc, right_operand),
            "/" => ast::divide(acc, right_operand),
            _ => unreachable!(),
        },
    )(input);

    result
}

/// primary <- "(" expression ")"
///     / integer
///     / function_call
///     / identifier
fn primary(input: &str) -> IResult<&str, ast::Expression> {
    alt((
        helper_combinators::parentheses(expression),
        integer,
        function_call,
        identifier,
    ))(input)
}

/// integer <- i64
fn integer(input: &str) -> IResult<&str, ast::Expression> {
    let (input, value) = raw_res::integer(input)?;

    Ok((input, ast::integer(value)))
}

/// function_call <- identifier "("
///     (expression ("," expression)*)?
/// ")"
fn function_call(input: &str) -> IResult<&str, ast::Expression> {
    let (input, name) = terminated(raw_res::identifier, multispace0)(input)?;

    let (input, args) = helper_combinators::parentheses(separated_list0(
        helper_combinators::ws(tag(",")),
        expression,
    ))(input)?;

    Ok((input, ast::call(name, args)))
}

/// identifier <- (alpha / "_")+ (
///     alphanumeric / "_"
/// )*;
fn identifier(input: &str) -> IResult<&str, ast::Expression> {
    let (input, name) = raw_res::identifier(input)?;

    Ok((input, ast::identifier(name)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Interpreter;

    #[test]
    fn interger_test() {
        let mut interpreter = Interpreter::new();

        let input = "42";
        let (_, expression) = integer(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 42);
    }

    #[test]
    fn multitive_left_operand_only() {
        let mut interpreter = Interpreter::new();

        let input = "42";
        let (_, expression) = multitive(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 42);
    }

    #[test]
    fn multitive_multiply() {
        let mut interpreter = Interpreter::new();

        let input = "21 * 2";
        let (_, expression) = multitive(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 42);
    }

    #[test]
    fn multitive_divide() {
        let mut interpreter = Interpreter::new();

        let input = "42 / 2";
        let (_, expression) = multitive(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 21);
    }

    #[test]
    fn multitive_second_right_operand_followed() {
        let mut interpreter = Interpreter::new();

        let input = "2 * 3 * (4 + 5)";
        let (_, expression) = multitive(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 54);
    }

    #[test]
    fn additive_left_operand_only() {
        let mut interpreter = Interpreter::new();

        let input = "42";
        let (_, expression) = additive(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 42);
    }

    #[test]
    fn additive_add() {
        let mut interpreter = Interpreter::new();

        let input = "2 + 2";
        let (_, expression) = additive(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 4);
    }

    #[test]
    fn additive_subtract() {
        let mut interpreter = Interpreter::new();

        let input = "42 - 2";
        let (_, expression) = additive(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 40);
    }

    #[test]
    fn additive_second_right_operand_followed() {
        let mut interpreter = Interpreter::new();

        let input = "2 + 3 + (4 - 5)";
        let (_, expression) = additive(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 4);
    }

    #[test]
    fn comparative_left_operand_only() {
        let mut interpreter = Interpreter::new();

        let input = "42";
        let (_, expression) = comparative(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 42);
    }

    #[test]
    fn comparative_less_than() {
        let mut interpreter = Interpreter::new();

        let input = "42 < 53";
        let (_, expression) = comparative(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 1);
    }

    #[test]
    fn comparative_greater_than() {
        let mut interpreter = Interpreter::new();

        let input = "42 > 53";
        let (_, expression) = comparative(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 0);
    }

    #[test]
    fn comparative_less_or_equal() {
        let mut interpreter = Interpreter::new();

        let input = "42 <= 42";
        let (_, expression) = comparative(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 1);
    }

    #[test]
    fn comparative_greater_or_equal() {
        let mut interpreter = Interpreter::new();

        let input = "42 >= 42";
        let (_, expression) = comparative(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 1);
    }

    #[test]
    fn comparative_equal_equal() {
        let mut interpreter = Interpreter::new();

        let input = "42 == 42";
        let (_, expression) = comparative(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 1);
    }

    #[test]
    fn comparative_not_equal() {
        let mut interpreter = Interpreter::new();

        let input = "42 != 42";
        let (_, expression) = comparative(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 0);
    }

    #[test]
    fn comparative_second_right_operand_followed() {
        let mut interpreter = Interpreter::new();

        let input = "(1 < 2) == (3 < 4)";
        let (_, expression) = comparative(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 1);
    }

    #[test]
    fn expression_line_test() {
        let mut interpreter = Interpreter::new();

        let input = "2 + 2;";
        let (_, expression) = expression_line(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 4);
    }

    #[test]
    fn assign_and_identify() {
        let mut interpreter = Interpreter::new();

        let input = "answer = 42;";
        let (_, expression) = assignment(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 42);

        let input = "answer";

        let (_, expression) = identifier(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 42);
    }

    #[test]
    fn block_expression_test() {
        let mut interpreter = Interpreter::new();

        let input = "{
            answer = 42;

            2 + 2 == 4;

            answer;
        }";

        let (_, expression) = block_expression(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 42);
    }

    #[test]
    fn while_expression_test() {
        let mut interpreter = Interpreter::new();

        let input = "answer = 0;";
        let (_, expression) = assignment(input).unwrap();
        let _ = interpreter.interpret(&expression).unwrap();

        let input = "i = 1;";
        let (_, expression) = assignment(input).unwrap();
        let _ = interpreter.interpret(&expression).unwrap();

        let input = "while (i <= 5) {
            answer = answer + i;
            i = i + 1;
        }";

        let (_, expression) = while_expression(input).unwrap();
        let _ = interpreter.interpret(&expression).unwrap();

        let input = "answer;";
        let (_, expression) = expression_line(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 15);
    }

    #[test]
    fn if_expression_only_then_clause() {
        let mut interpreter = Interpreter::new();

        let input = "if (2 + 2 == 4) {
            42;
        }";

        let (_, expression) = if_expression(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 42);
    }

    #[test]
    fn if_expression_else_clause() {
        let mut interpreter = Interpreter::new();

        let input = "if (2 + 2 != 4) {
            42;
        } else {
            21;
        }";

        let (_, expression) = if_expression(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 21);
    }

    #[test]
    fn println_test() {
        let mut interpreter = Interpreter::new();

        let input = "println(42)";
        let (_, expression) = println(input).unwrap();
        let value = interpreter.interpret(&expression).unwrap();

        assert_eq!(value, 1);
    }
}
