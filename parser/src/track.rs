use std::fmt::Display;

use nom::{
    character::complete::{alpha1, multispace1},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::{
    components::{curlied, spaced, squared},
    track_event::TrackEvent,
};

#[derive(Debug)]
pub struct Track<'a> {
    name: &'a str,
    events: Vec<TrackEvent<'a>>,
}

impl<'a> Track<'a> {
    #[must_use]
    pub(crate) fn new(name: &'a str, events: Vec<TrackEvent<'a>>) -> Self {
        Self { name, events }
    }

    pub(crate) fn multiply(&mut self, factor: u32) {
        for item in &mut self.events {
            item.multiply(factor);
        }
    }

    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, Track> {
        map(
            tuple((
                spaced(squared(alpha1)),
                curlied(spaced(separated_list1(multispace1, TrackEvent::parse))),
            )),
            |(name, events)| Track::new(name, events),
        )(input)
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_track() {
        Track::parse(include_str!("test_data/test_track.txt")).unwrap();
    }
}
