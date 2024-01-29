use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

use crate::song_property::{property, Property};

pub fn song(input: &str) -> IResult<&str, Vec<Property>> {
    preceded(
        preceded(tag("[Song]"), multispace0),
        delimited(
            preceded(tag("{"), multispace0),
            separated_list1(multispace1, property),
            preceded(multispace1, tag("}")),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

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
