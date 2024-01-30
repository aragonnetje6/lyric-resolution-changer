use std::fmt::Display;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, multispace1},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::track_event::TrackEvent;

#[derive(Debug)]
pub struct Track<'a> {
    name: &'a str,
    events: Vec<TrackEvent<'a>>,
}

impl<'a> Track<'a> {
    pub fn new(name: &'a str, events: Vec<TrackEvent<'a>>) -> Self {
        Self { name, events }
    }

    pub fn multiply(&mut self, factor: u32) {
        for item in &mut self.events {
            item.multiply(factor);
        }
    }
    pub fn parse(input: &str) -> IResult<&str, Track> {
        map(
            tuple((
                terminated(delimited(tag("["), alpha1, tag("]")), multispace0),
                delimited(
                    preceded(tag("{"), multispace0),
                    separated_list1(multispace1, TrackEvent::parse),
                    preceded(multispace0, tag("}")),
                ),
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
