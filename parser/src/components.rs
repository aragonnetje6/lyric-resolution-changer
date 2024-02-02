use nom::{
    character::complete::{self, multispace0},
    error::ParseError,
    sequence::delimited,
    IResult, Parser,
};

#[inline]
pub(crate) fn curlied<'a, O, E: ParseError<&'a str>, F>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(complete::char('{'), inner, complete::char('}'))
}

#[inline]
pub(crate) fn squared<'a, O, E: ParseError<&'a str>, F>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(complete::char('['), inner, complete::char(']'))
}

#[inline]
pub(crate) fn quoted<'a, O, E: ParseError<&'a str>, F>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(complete::char('"'), inner, complete::char('"'))
}

#[inline]
pub(crate) fn spaced<'a, O, E: ParseError<&'a str>, F>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
