use std::fmt::Display;

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

use crate::sync_track_event::SyncTrackEvent;

#[derive(Debug)]
pub(crate) struct SyncTrack {
    events: Vec<SyncTrackEvent>,
}

impl SyncTrack {
    pub(crate) fn new(events: Vec<SyncTrackEvent>) -> Self {
        Self { events }
    }

    pub(crate) fn parse(input: &str) -> IResult<&str, Self> {
        map(
            preceded(
                preceded(tag("[SyncTrack]"), multispace0),
                delimited(
                    preceded(tag("{"), multispace0),
                    separated_list1(multispace1, SyncTrackEvent::parse),
                    preceded(multispace0, tag("}")),
                ),
            ),
            Self::new,
        )(input)
    }

    pub(crate) fn multiply(&mut self, factor: u32) {
        for event in &mut self.events {
            event.multiply(factor);
        }
    }
}

impl Display for SyncTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.events.iter().map(SyncTrackEvent::to_string).collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_sync_track() {
        SyncTrack::parse(
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
