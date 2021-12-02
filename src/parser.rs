use crate::ast;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    multi::{fold_many0, separated_list0},
    sequence::{pair, preceded, terminated},
    IResult,
};

mod helper_combinators;
mod raw_res;

/// assignment <- identifier "=" expression ";";
fn assignment(input: &str) -> IResult<&str, ast::Expression> {
    let (input, name) = raw_res::identifier(input)?;
    let (input, _) = helper_combinators::ws(tag("="))(input)?;
    let (input, ast_expression) = expression(input)?;
    let (input, _) = preceded(multispace1, tag(";"))(input)?;

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
                tag("<"),
                tag(">"),
                tag("<="),
                tag(">="),
                tag("=="),
                tag("!="),
            ))),
            additive,
        ),
        || left_operand.clone(),
        |acc, (operator, right_operand)| match operator {
            "<" => ast::less_than(acc, right_operand),
            ">" => ast::greater_than(acc, right_operand),
            "<=" => ast::less_or_equal(acc, right_operand),
            ">=" => ast::greater_or_equal(acc, right_operand),
            "==" => ast::equal_equal(acc, right_operand),
            "!=" => ast::not_equal(acc, right_operand),
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
    let (input, name) = terminated(raw_res::identifier, multispace1)(input)?;

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
