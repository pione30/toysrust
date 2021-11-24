use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::recognize,
    multi::many0,
    sequence::pair,
    IResult,
};

/// identifier <- (alpha / "_")+ (
///     alphanumeric / "_"
/// )*;
fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn identifier_test() {
        assert_eq!(identifier("foo"), Ok(("", "foo")));
        assert_eq!(identifier("_foo"), Ok(("", "_foo")));
        assert_eq!(identifier("foo42_hello"), Ok(("", "foo42_hello")));
        assert_eq!(
            identifier("42foo"),
            Err(nom::Err::Error(nom::error::Error::new(
                "42foo",
                nom::error::ErrorKind::Tag
            )))
        );
    }
}
