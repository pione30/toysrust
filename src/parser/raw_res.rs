use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, i64},
    combinator::recognize,
    multi::many0,
    sequence::pair,
    IResult,
};

/// integer <- i64
pub fn integer(input: &str) -> IResult<&str, i64> {
    i64(input)
}

/// identifier <- (alpha / "_")+ (
///     alphanumeric / "_"
/// )*;
pub fn identifier(input: &str) -> IResult<&str, &str> {
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

    #[test]
    fn i64_test() {
        assert_eq!(integer("42"), Ok(("", 42)));
    }
}
