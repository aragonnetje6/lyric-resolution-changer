use std::fmt::Display;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, not_line_ending},
    combinator::map,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SongProperty<'a> {
    name: &'a str,
    value: &'a str,
}

impl<'a> Display for SongProperty<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  {} = {}", self.name, self.value)
    }
}

impl<'a> SongProperty<'a> {
    #[must_use]
    pub(crate) fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }

    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, SongProperty> {
        map(
            separated_pair(alphanumeric1, tag(" = "), not_line_ending),
            |(name, value)| SongProperty::new(name, value),
        )(input)
    }

    pub(crate) fn name(&self) -> &'a str {
        self.name
    }

    pub(crate) fn value(&self) -> &'a str {
        self.value
    }
}
