use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, space1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug)]
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

impl SyncTrackEvent {
    pub(crate) fn multiply(&mut self, factor: u32) {
        match self {
            SyncTrackEvent::Bpm { time, .. }
            | SyncTrackEvent::TimeSignature { time, .. }
            | SyncTrackEvent::Anchor { time, .. } => *time *= factor,
        }
    }

    fn parse(input: &str) -> IResult<&str, SyncTrackEvent> {
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

    pub(crate) fn parse_section(input: &str) -> IResult<&str, Vec<SyncTrackEvent>> {
        preceded(
            preceded(tag("[SyncTrack]"), multispace0),
            delimited(
                preceded(tag("{"), multispace0),
                separated_list1(multispace1, SyncTrackEvent::parse),
                preceded(multispace0, tag("}")),
            ),
        )(input)
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

    #[test]
    fn test_sync_track() {
        SyncTrackEvent::parse_section(
            "[SyncTrack]
{
  0 = TS 6
  0 = B 152525
  1152 = TS 4
  4224 = B 160187
  10368 = B 160000
  154752 = B 158662
  156288 = B 180000
  168576 = B 160000
  173184 = B 160866
  174720 = B 160000
}",
        )
        .unwrap();
    }
}
