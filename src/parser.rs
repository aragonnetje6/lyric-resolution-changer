use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_until1},
    character::complete::{alpha1, alphanumeric1, multispace1, space1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::display::{Chart, GlobalEvent, Property, SyncTrackEvent, Track, TrackEvent};

pub fn chart(input: &str) -> IResult<&str, Chart> {
    let (input, _) = take_until("[")(input)?;
    let (input, song) = song(input)?;
    let (input, _) = multispace1(input)?;
    let (input, synctrack) = sync_track(input)?;
    let (input, _) = multispace1(input)?;
    let (input, global_events) = global_events(input)?;
    let (input, _) = multispace1(input)?;
    let (input, tracks) = separated_list1(multispace1, track)(input)?;
    Ok((input, Chart::new(song, synctrack, global_events, tracks)))
}

fn song(input: &str) -> IResult<&str, Vec<Property>> {
    preceded(
        preceded(tag("[Song]"), multispace1),
        delimited(
            preceded(tag("{"), multispace1),
            separated_list1(multispace1, preceded(multispace1, property)),
            preceded(multispace1, tag("}")),
        ),
    )(input)
}

fn property(input: &str) -> IResult<&str, Property> {
    let (input, name) = alphanumeric1(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, value) = take_until1("\n")(input)?;
    Ok((input, Property::new(name, value.to_string())))
}

fn sync_track(input: &str) -> IResult<&str, Vec<SyncTrackEvent>> {
    preceded(
        preceded(tag("[SyncTrack]"), multispace1),
        delimited(
            preceded(tag("{"), multispace1),
            separated_list1(multispace1, preceded(multispace1, sync_track_event)),
            preceded(multispace1, tag("}")),
        ),
    )(input)
}

fn sync_track_event(input: &str) -> IResult<&str, SyncTrackEvent> {
    let (input, time) = nom::character::complete::u32(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, result) = alt((
        map(
            tuple((
                tag("TS"),
                preceded(space1, nom::character::complete::u32),
                opt(preceded(space1, nom::character::complete::u32)),
            )),
            |(_, value1, value2)| SyncTrackEvent::TimeSignature {
                time,
                value1,
                value2,
            },
        ),
        map(
            tuple((tag("B"), preceded(space1, nom::character::complete::u32))),
            |(_, value)| SyncTrackEvent::Bpm { time, value },
        ),
        map(
            tuple((tag("A"), preceded(space1, nom::character::complete::u32))),
            |(_, value)| SyncTrackEvent::Anchor { time, value },
        ),
    ))(input)?;
    Ok((input, result))
}

fn global_events(input: &str) -> IResult<&str, Vec<GlobalEvent>> {
    preceded(
        preceded(tag("[Events]"), multispace1),
        delimited(
            preceded(tag("{"), multispace1),
            separated_list1(multispace1, preceded(multispace1, global_event)),
            preceded(multispace1, tag("}")),
        ),
    )(input)
}

fn global_event(input: &str) -> IResult<&str, GlobalEvent> {
    let (input, time) = nom::character::complete::u32(input)?;
    let (input, _) = tag(" = E ")(input)?;
    let (input, result) = delimited(
        tag("\""),
        alt((
            map(tag("phrase_start"), |_| GlobalEvent::PhraseStart { time }),
            map(tag("phrase_end"), |_| GlobalEvent::PhraseEnd { time }),
            map(preceded(tag("section "), take_until("\"")), |name| {
                GlobalEvent::Section { time, name }
            }),
            map(preceded(tag("lyric "), take_until("\"")), |text| {
                GlobalEvent::Lyric { time, text }
            }),
        )),
        tag("\""),
    )(input)?;
    Ok((input, result))
}

fn track(input: &str) -> IResult<&str, Track> {
    map(
        tuple((
            terminated(delimited(tag("["), alpha1, tag("]")), multispace1),
            delimited(
                preceded(tag("{"), multispace1),
                separated_list1(multispace1, preceded(multispace1, track_event)),
                preceded(multispace1, tag("}")),
            ),
        )),
        |(name, events)| Track::new(name, events),
    )(input)
}

fn track_event(input: &str) -> IResult<&str, TrackEvent<'_>> {
    let (input, time) = nom::character::complete::u32(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, result) = alt((
        map(
            preceded(
                tag("N "),
                separated_pair(
                    nom::character::complete::u32,
                    space1,
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
                    space1,
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_track_event() {
        track_event("183936 = N 4 3072").unwrap();
    }

    #[test]
    fn test_track() {
        track(include_str!("test_data/test_track.txt")).unwrap();
    }

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

    #[test]
    fn test_sync_track_event() {
        sync_track_event("0 = TS 6").unwrap();
        sync_track_event("0 = B 152525").unwrap();
    }

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

    #[test]
    fn test_song() {
        song(
            r#"[Song]
{
  Name = "Second Sight"
  Artist = "Adagio"
  Charter = "Peddy"
  Album = "Sanctus Ignis"
  Year = ", 2001"
  Offset = 0
  Resolution = 192
  Player2 = bass
  Difficulty = 0
  PreviewStart = 0
  PreviewEnd = 0
  Genre = "Neoclassical Metal"
  MediaType = "cd"
  MusicStream = "song.ogg"
}"#,
        )
        .unwrap();
    }
}
