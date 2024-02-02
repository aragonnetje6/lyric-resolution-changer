use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace1, space1},
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SyncTrackEvent {
    Bpm {
        time: u32,
        value: u32,
    },
    TimeSignature {
        time: u32,
        value1: u32,
        value2: Option<u32>,
    },
    Anchor {
        time: u32,
        value: u32,
    },
}

impl SyncTrackEvent {
    pub(crate) fn multiply(&mut self, factor: u32) {
        match self {
            SyncTrackEvent::Bpm { time, .. }
            | SyncTrackEvent::TimeSignature { time, .. }
            | SyncTrackEvent::Anchor { time, .. } => *time *= factor,
        }
    }

    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, SyncTrackEvent> {
        let (input, time) = nom::character::complete::u32(input)?;
        let (input, _) = tag(" = ")(input)?;
        let (input, result) = alt((
            map(
                tuple((
                    tag("TS"),
                    preceded(multispace1, nom::character::complete::u32),
                    opt(preceded(space1, nom::character::complete::u32)),
                )),
                |(_, value1, value2)| SyncTrackEvent::TimeSignature {
                    time,
                    value1,
                    value2,
                },
            ),
            map(
                tuple((
                    tag("B"),
                    preceded(multispace1, nom::character::complete::u32),
                )),
                |(_, value)| SyncTrackEvent::Bpm { time, value },
            ),
            map(
                tuple((
                    tag("A"),
                    preceded(multispace1, nom::character::complete::u32),
                )),
                |(_, value)| SyncTrackEvent::Anchor { time, value },
            ),
        ))(input)?;
        Ok((input, result))
    }
}

impl Display for SyncTrackEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncTrackEvent::Bpm { time, value } => writeln!(f, "  {time} = B {value}"),
            SyncTrackEvent::TimeSignature {
                time,
                value1,
                value2: Some(value2),
            } => writeln!(f, "  {time} = TS {value1} {value2}"),
            SyncTrackEvent::TimeSignature {
                time,
                value1,
                value2: None,
            } => writeln!(f, "  {time} = TS {value1}"),
            SyncTrackEvent::Anchor { time, value } => writeln!(f, "  {time} = A {value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_sync_track_event() {
        SyncTrackEvent::parse("0 = TS 6").unwrap();
        SyncTrackEvent::parse("0 = B 152525").unwrap();
    }
}
