use std::fmt::Display;

use nom::{
    bytes::complete::take_until,
    character::complete::{multispace0, multispace1},
    combinator::all_consuming,
    multi::separated_list1,
    IResult,
};

use crate::{events::Events, song::Song, sync_track::SyncTrack, track::Track};

#[derive(Debug, PartialEq, Eq)]
pub struct Chart<'a> {
    song: Song<'a>,
    synctrack: SyncTrack,
    global_events: Events<'a>,
    tracks: Vec<Track<'a>>,
}

impl<'a> Chart<'a> {
    #[must_use]
    pub(crate) fn new(
        song: Song<'a>,
        synctrack: SyncTrack,
        global_events: Events<'a>,
        tracks: Vec<Track<'a>>,
    ) -> Self {
        Self {
            song,
            synctrack,
            global_events,
            tracks,
        }
    }

    /// Multiply all timestamps and durations by the given factor. If two events have a 1-tick difference, this difference is preserved.
    pub fn multiply(&mut self, factor: u32) {
        self.song.multiply(factor);
        self.synctrack.multiply(factor);
        self.global_events.multiply(factor);
        for item in &mut self.tracks {
            item.multiply(factor);
        }
    }

    /// Parse the .chart
    ///
    /// # Errors
    ///
    /// This function will return an error if the given string does not
    /// represent a valid .chart file.
    pub fn parse(input: &str) -> IResult<&str, Chart> {
        let (input, _) = take_until("[")(input)?;
        let (input, song) = Song::parse(input)?;
        let (input, _) = multispace0(input)?;
        let (input, synctrack) = SyncTrack::parse(input)?;
        let (input, _) = multispace0(input)?;
        let (input, global_events) = Events::parse(input)?;
        let (input, _) = multispace0(input)?;
        let (input, tracks) = separated_list1(multispace1, Track::parse)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, Chart::new(song, synctrack, global_events, tracks)))
    }
}

impl<'a> Display for Chart<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{}",
            self.song,
            self.synctrack,
            self.global_events,
            self.tracks.iter().map(Track::to_string).collect::<String>()
        )
    }
}
