use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::{cut, map},
    multi::many1,
    sequence::{delimited, preceded},
    IResult,
};

use crate::components::{curlied, spaced};

#[derive(Debug)]
pub enum GlobalEvent<'a> {
    PhraseStart { time: u32 },
    PhraseEnd { time: u32 },
    Section { time: u32, name: &'a str },
    Lyric { time: u32, text: &'a str },
    Other { time: u32, value: &'a str },
}

impl<'a> GlobalEvent<'a> {
    pub(crate) fn multiply(&mut self, factor: u32) {
        match self {
            GlobalEvent::PhraseStart { time }
            | GlobalEvent::PhraseEnd { time }
            | GlobalEvent::Section { time, .. }
            | GlobalEvent::Lyric { time, .. }
            | GlobalEvent::Other { time, .. } => *time *= factor,
        }
    }

    #[inline]
    fn parse(input: &str) -> IResult<&str, GlobalEvent> {
        let (input, time) = nom::character::complete::u32(input)?;
        let (input, _) = tag(" = E ")(input)?;
        let (input, result) = delimited(
            tag("\""),
            alt((
                map(tag("phrase_start"), |_| GlobalEvent::PhraseStart { time }),
                map(tag("phrase_end"), |_| GlobalEvent::PhraseEnd { time }),
                map(preceded(tag("section "), cut(take_until("\""))), |name| {
                    GlobalEvent::Section { time, name }
                }),
                map(preceded(tag("lyric "), cut(take_until("\""))), |text| {
                    GlobalEvent::Lyric { time, text }
                }),
                map(take_until("\""), |value| GlobalEvent::Other { time, value }),
            )),
            tag("\""),
        )(input)?;
        Ok((input, result))
    }

    #[inline]
    pub(crate) fn parse_section(input: &str) -> IResult<&str, Vec<GlobalEvent>> {
        preceded(
            spaced(tag("[Events]")),
            curlied(many1(spaced(GlobalEvent::parse))),
        )(input)
    }
}

impl<'a> Display for GlobalEvent<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlobalEvent::PhraseStart { time } => writeln!(f, "  {time} = E \"phrase_start\""),
            GlobalEvent::PhraseEnd { time } => writeln!(f, "  {time} = E \"phrase_end\""),
            GlobalEvent::Section { time, name } => writeln!(f, "  {time} = E \"section {name}\""),
            GlobalEvent::Lyric { time, text } => writeln!(f, "  {time} = E \"lyric {text}\""),
            GlobalEvent::Other { time, value } => write!(f, "  {time} = E \"{value}\""),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_global_event() {
        GlobalEvent::parse("4224 = E \"section Intro\"").unwrap();
        GlobalEvent::parse("38496 = E \"phrase_start\"").unwrap();
        GlobalEvent::parse("38592 = E \"lyric I\"").unwrap();
        GlobalEvent::parse("40512 = E \"phrase_end\"").unwrap();
    }

    #[test]
    fn test_global_events() {
        GlobalEvent::parse_section(include_str!("test_data/test_events.txt")).unwrap();
    }
}
