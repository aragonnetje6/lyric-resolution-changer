use std::fmt::Display;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, not_line_ending},
    combinator::map,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
pub struct SongProperty<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

impl<'a> Display for SongProperty<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  {} = {}", self.name, self.value)
    }
}

impl<'a> SongProperty<'a> {
    #[must_use]
    pub fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }

    pub fn parse(input: &str) -> IResult<&str, SongProperty> {
        map(
            separated_pair(alphanumeric1, tag(" = "), not_line_ending),
            |(name, value)| SongProperty::new(name, value),
        )(input)
    }
}
