use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{multispace0, multispace1},
    combinator::{cut, map},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug)]
pub enum GlobalEvent<'a> {
    PhraseStart { time: u32 },
    PhraseEnd { time: u32 },
    Section { time: u32, name: &'a str },
    Lyric { time: u32, text: &'a str },
    Other { time: u32, value: &'a str },
}

impl<'a> GlobalEvent<'a> {
    pub fn multiply(&mut self, factor: u32) {
        match self {
            GlobalEvent::PhraseStart { time }
            | GlobalEvent::PhraseEnd { time }
            | GlobalEvent::Section { time, .. }
            | GlobalEvent::Lyric { time, .. }
            | GlobalEvent::Other { time, .. } => *time *= factor,
        }
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

pub fn global_events(input: &str) -> IResult<&str, Vec<GlobalEvent>> {
    preceded(
        preceded(tag("[Events]"), multispace0),
        delimited(
            preceded(tag("{"), multispace0),
            separated_list1(multispace1, global_event),
            preceded(multispace0, tag("}")),
        ),
    )(input)
}

pub fn global_event(input: &str) -> IResult<&str, GlobalEvent> {
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_global_event() {
        global_event("4224 = E \"section Intro\"").unwrap();
        global_event("38496 = E \"phrase_start\"").unwrap();
        global_event("38592 = E \"lyric I\"").unwrap();
        global_event("40512 = E \"phrase_end\"").unwrap();
    }

    #[test]
    fn test_global_events() {
        global_events(include_str!("test_data/test_events.txt")).unwrap();
    }
}
