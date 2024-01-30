use std::fmt::Display;

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

use crate::song_property::Property;

#[derive(Debug)]
pub struct Song<'a> {
    resolution: u32,
    properties: Vec<Property<'a>>,
}

impl<'a> Song<'a> {
    pub fn new(resolution: u32, properties: Vec<Property<'a>>) -> Self {
        Self {
            resolution,
            properties,
        }
    }

    pub fn multiply(&mut self, factor: u32) {
        self.resolution *= factor;
    }

    pub fn parse(input: &str) -> IResult<&str, Song> {
        map_res(
            preceded(
                preceded(tag("[Song]"), multispace0),
                delimited(
                    preceded(tag("{"), multispace0),
                    separated_list1(multispace1, Property::parse),
                    preceded(multispace1, tag("}")),
                ),
            ),
            Song::try_from,
        )(input)
    }
}

impl<'a> TryFrom<Vec<Property<'a>>> for Song<'a> {
    type Error = &'static str;

    fn try_from(value: Vec<Property<'a>>) -> Result<Self, Self::Error> {
        let resolution_entry = value
            .iter()
            .find(|x| x.name == "Resolution")
            .ok_or("resolution not found")?;
        let resolution = resolution_entry
            .value
            .parse::<u32>()
            .map_err(|_| "invalid resolution")?;
        let properties = value
            .into_iter()
            .filter(|x| x.name != "Resolution")
            .collect();
        Ok(Self {
            resolution,
            properties,
        })
    }
}

impl<'a> Display for Song<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            Property::new("resolution", self.resolution.to_string()),
            self.properties
                .iter()
                .map(Property::to_string)
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_song() {
        Song::parse(
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
