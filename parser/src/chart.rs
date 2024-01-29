use std::fmt::Display;

use nom::{
    bytes::complete::take_until,
    character::complete::{multispace0, multispace1},
    multi::separated_list1,
    IResult,
};

use crate::{
    global_event::{global_events, GlobalEvent},
    song::{song, Song},
    song_property::Property,
    sync_track::sync_track,
    sync_track_event::SyncTrackEvent,
    track::{track, Track},
};

#[derive(Debug)]
pub struct Chart<'a> {
    song: Song<'a>,
    synctrack: Vec<SyncTrackEvent>,
    global_events: Vec<GlobalEvent<'a>>,
    tracks: Vec<Track<'a>>,
}

impl<'a> Chart<'a> {
    pub fn new(
        song: Song<'a>,
        synctrack: Vec<SyncTrackEvent>,
        global_events: Vec<GlobalEvent<'a>>,
        tracks: Vec<Track<'a>>,
    ) -> Self {
        Self {
            song,
            synctrack,
            global_events,
            tracks,
        }
    }

    pub fn multiply(&mut self, factor: u32) {
        self.song.multiply(factor);
        for item in &mut self.synctrack {
            item.multiply(factor);
        }
        for item in &mut self.global_events {
            item.multiply(factor);
        }
        for item in &mut self.tracks {
            item.multiply(factor);
        }
    }
}

impl<'a> Display for Chart<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Song]
{{
{}}}
[SyncTrack]
{{
{}}}
[Events]
{{
{}}}
{}",
            self.song,
            self.synctrack
                .iter()
                .map(SyncTrackEvent::to_string)
                .collect::<String>(),
            self.global_events
                .iter()
                .map(GlobalEvent::to_string)
                .collect::<String>(),
            self.tracks.iter().map(Track::to_string).collect::<String>()
        )
    }
}

pub fn chart(input: &str) -> IResult<&str, Chart> {
    let (input, _) = take_until("[")(input)?;
    let (input, song) = song(input)?;
    let (input, _) = multispace0(input)?;
    let (input, synctrack) = sync_track(input)?;
    let (input, _) = multispace0(input)?;
    let (input, global_events) = global_events(input)?;
    let (input, _) = multispace0(input)?;
    let (input, tracks) = separated_list1(multispace1, track)(input)?;
    Ok((input, Chart::new(song, synctrack, global_events, tracks)))
}
