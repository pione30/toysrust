//! Useful helper combinators.
//! Ref. https://docs.rs/nom/7.1.0/nom/recipes/index.html

use nom::{bytes::complete::tag, character::complete::multispace0, sequence::delimited, IResult};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading "(" and
/// trailing ")", returning the output of `inner`.
pub fn parentheses<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(tag("("), ws(inner), tag(")"))
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading "{" and
/// trailing "}", returning the output of `inner`.
pub fn curly_brackets<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(tag("{"), ws(inner), tag("}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::bytes::complete::tag;

    #[test]
    fn ws_mulitspace0_test() {
        let input = "hello";
        let hello = tag::<&str, &str, nom::error::Error<&str>>("hello");

        assert_eq!(ws(hello)(input), Ok(("", "hello")));
    }

    #[test]
    fn ws_mulitspace1_test() {
        let input = "
            hello   world";

        let hello = tag::<&str, &str, nom::error::Error<&str>>("hello");

        assert_eq!(ws(hello)(input), Ok(("world", "hello")));
    }

    #[test]
    fn parantheses_multispace0_test() {
        let input = "(hello)";

        let hello = tag::<&str, &str, nom::error::Error<&str>>("hello");

        assert_eq!(parentheses(hello)(input), Ok(("", "hello")));
    }

    #[test]
    fn parantheses_multispace1_test() {
        let input = "(
            hello
        )";

        let hello = tag::<&str, &str, nom::error::Error<&str>>("hello");

        assert_eq!(parentheses(hello)(input), Ok(("", "hello")));
    }

    #[test]
    fn curly_brackets_multispaces0_test() {
        let input = "{hello}";

        let hello = tag::<&str, &str, nom::error::Error<&str>>("hello");

        assert_eq!(curly_brackets(hello)(input), Ok(("", "hello")));
    }

    #[test]
    fn curly_brackets_multispace1_test() {
        let input = "{
            hello
        }";

        let hello = tag::<&str, &str, nom::error::Error<&str>>("hello");

        assert_eq!(curly_brackets(hello)(input), Ok(("", "hello")));
    }
}
