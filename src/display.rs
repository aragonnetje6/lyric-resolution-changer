use std::fmt::Display;

#[derive(Debug)]
pub struct Chart<'a> {
    pub song: Vec<Property<'a>>,
    pub synctrack: Vec<SyncTrackEvent>,
    pub global_events: Vec<GlobalEvent<'a>>,
    pub tracks: Vec<Track<'a>>,
}

impl<'a> Chart<'a> {
    pub fn new(
        song: Vec<Property<'a>>,
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

    pub fn multiply_res(&mut self, factor: u32) {
        self.song
            .iter_mut()
            .find(|x| x.name == "Resolution")
            .map(|x| {
                x.value = (x.value.parse::<u32>().unwrap() * factor).to_string();
            })
            .unwrap();
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
            self.song
                .iter()
                .map(Property::to_string)
                .collect::<String>(),
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

#[derive(Debug)]
pub struct Property<'a> {
    name: &'a str,
    value: String,
}

impl<'a> Display for Property<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  {} = {}", self.name, self.value)
    }
}

impl<'a> Property<'a> {
    pub fn new(name: &'a str, value: String) -> Self {
        Self { name, value }
    }
}

#[derive(Debug)]
pub enum SyncTrackEvent {
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
    fn multiply(&mut self, factor: u32) {
        match self {
            SyncTrackEvent::Bpm { time, .. }
            | SyncTrackEvent::TimeSignature { time, .. }
            | SyncTrackEvent::Anchor { time, .. } => *time *= factor,
        }
    }
}

#[derive(Debug)]
pub enum GlobalEvent<'a> {
    PhraseStart { time: u32 },
    PhraseEnd { time: u32 },
    Section { time: u32, name: &'a str },
    Lyric { time: u32, text: &'a str },
}

impl<'a> GlobalEvent<'a> {
    fn multiply(&mut self, factor: u32) {
        match self {
            GlobalEvent::PhraseStart { time }
            | GlobalEvent::PhraseEnd { time }
            | GlobalEvent::Section { time, .. }
            | GlobalEvent::Lyric { time, .. } => *time *= factor,
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
        }
    }
}

#[derive(Debug)]
pub struct Track<'a> {
    name: &'a str,
    events: Vec<TrackEvent<'a>>,
}

impl<'a> Track<'a> {
    pub fn new(name: &'a str, events: Vec<TrackEvent<'a>>) -> Self {
        Self { name, events }
    }

    fn multiply(&mut self, factor: u32) {
        for item in &mut self.events {
            item.multiply(factor);
        }
    }
}

impl<'a> Display for Track<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]\n{{\n{}}}\n",
            self.name,
            self.events
                .iter()
                .map(TrackEvent::to_string)
                .collect::<String>()
        )
    }
}

#[derive(Debug)]
pub enum TrackEvent<'a> {
    Note { time: u32, fret: u32, sustain: u32 },
    Special { time: u32, kind: u32, content: u32 },
    Event { time: u32, value: &'a str },
}

impl<'a> TrackEvent<'a> {
    fn multiply(&mut self, factor: u32) {
        match self {
            TrackEvent::Note { time, sustain, .. } => {
                *time *= factor;
                *sustain *= factor;
            }
            TrackEvent::Special { time, .. } | TrackEvent::Event { time, .. } => *time *= factor,
        }
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
