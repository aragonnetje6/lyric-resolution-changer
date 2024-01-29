use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

use crate::sync_track_event::{sync_track_event, SyncTrackEvent};

pub fn sync_track(input: &str) -> IResult<&str, Vec<SyncTrackEvent>> {
    preceded(
        preceded(tag("[SyncTrack]"), multispace0),
        delimited(
            preceded(tag("{"), multispace0),
            separated_list1(multispace1, sync_track_event),
            preceded(multispace0, tag("}")),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_sync_track() {
        sync_track(
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
