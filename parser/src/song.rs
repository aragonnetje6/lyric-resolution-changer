use std::fmt::Display;

use nom::{bytes::complete::tag, combinator::map_res, multi::many1, sequence::preceded, IResult};

use crate::{
    components::{curlied, spaced},
    song_property::SongProperty,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Song<'a> {
    resolution: u32,
    properties: Vec<SongProperty<'a>>,
}

impl<'a> Song<'a> {
    #[must_use]
    pub(crate) fn new(resolution: u32, properties: Vec<SongProperty<'a>>) -> Self {
        Self {
            resolution,
            properties,
        }
    }

    pub(crate) fn multiply(&mut self, factor: u32) {
        self.resolution *= factor;
    }

    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, Song> {
        map_res(
            preceded(
                spaced(tag("[Song]")),
                curlied(many1(spaced(SongProperty::parse))),
            ),
            Song::try_from,
        )(input)
    }
}

impl<'a> TryFrom<Vec<SongProperty<'a>>> for Song<'a> {
    type Error = &'static str;

    fn try_from(value: Vec<SongProperty<'a>>) -> Result<Self, Self::Error> {
        let resolution_entry = value
            .iter()
            .find(|x| x.name() == "Resolution")
            .ok_or("resolution not found")?;
        let resolution = resolution_entry
            .value()
            .parse::<u32>()
            .map_err(|_| "invalid resolution")?;
        let properties = value
            .into_iter()
            .filter(|x| x.name() != "Resolution")
            .collect();
        Ok(Self::new(resolution, properties))
    }
}

impl<'a> Display for Song<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let resolution_string = self.resolution.to_string();
        write!(
            f,
            "{}{}",
            SongProperty::new("Resolution", &resolution_string),
            self.properties
                .iter()
                .map(SongProperty::to_string)
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
