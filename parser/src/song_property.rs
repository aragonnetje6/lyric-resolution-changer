use std::fmt::Display;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, not_line_ending},
    IResult,
};

#[derive(Debug)]
pub struct Property<'a> {
    pub name: &'a str,
    pub value: String,
}

impl<'a> Display for Property<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  {} = {}", self.name, self.value)
    }
}

impl<'a> Property<'a> {
    pub fn new(name: &'a str, value: String) -> Self {
        Self { name, value }
    }

    pub fn parse(input: &str) -> IResult<&str, Property> {
        let (input, name) = alphanumeric1(input)?;
        let (input, _) = tag(" = ")(input)?;
        let (input, value) = not_line_ending(input)?;
        Ok((input, Property::new(name, value.to_string())))
    }
}
