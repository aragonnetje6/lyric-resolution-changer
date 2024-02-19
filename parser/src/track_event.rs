use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace1},
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub enum TrackEvent<'a> {
    Note { time: u32, fret: u32, sustain: u32 },
    Special { time: u32, kind: u32, content: u32 },
    Event { time: u32, value: &'a str },
}

impl<'a> TrackEvent<'a> {
    pub(crate) fn multiply(&mut self, factor: u32) {
        match self {
            TrackEvent::Note { time, sustain, .. } => {
                *time *= factor;
                *sustain *= factor;
            }
            TrackEvent::Special { time, content, .. } => {
                *time *= factor;
                *content *= factor;
            }
            TrackEvent::Event { time, .. } => *time *= factor,
        }
    }

    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, TrackEvent<'_>> {
        let (input, time) = nom::character::complete::u32(input)?;
        let (input, _) = tag(" = ")(input)?;
        let (input, result) = alt((
            map(
                preceded(
                    tag("N "),
                    separated_pair(
                        nom::character::complete::u32,
                        multispace1,
                        nom::character::complete::u32,
                    ),
                ),
                |(fret, sustain)| TrackEvent::Note {
                    time,
                    fret,
                    sustain,
                },
            ),
            map(preceded(tag("E "), alpha1), |value| TrackEvent::Event {
                time,
                value,
            }),
            map(
                preceded(
                    tag("S "),
                    separated_pair(
                        nom::character::complete::u32,
                        multispace1,
                        nom::character::complete::u32,
                    ),
                ),
                |(kind, content)| TrackEvent::Special {
                    time,
                    kind,
                    content,
                },
            ),
        ))(input)?;
        Ok((input, result))
    }
}

impl<'a> Display for TrackEvent<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrackEvent::Note {
                time,
                fret,
                sustain,
            } => writeln!(f, "  {time} = N {fret} {sustain}"),
            TrackEvent::Special {
                time,
                kind,
                content,
            } => writeln!(f, "  {time} = S {kind} {content}"),
            TrackEvent::Event { time, value } => writeln!(f, "  {time} = E {value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_track_event() {
        TrackEvent::parse("183936 = N 4 3072").unwrap();
    }
}
