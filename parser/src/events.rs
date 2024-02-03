use std::fmt::Display;

use nom::{bytes::complete::tag, combinator::map, multi::many1, sequence::preceded, IResult};

use crate::{
    components::{curlied, spaced},
    global_event::GlobalEvent,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Events<'a> {
    events: Vec<GlobalEvent<'a>>,
}

impl<'a> Events<'a> {
    #[must_use]
    pub(crate) fn new(events: Vec<GlobalEvent<'a>>) -> Self {
        Self { events }
    }

    pub(crate) fn multiply(&mut self, factor: u32) {
        if let Some((head, tail)) = self.events.split_first_mut() {
            let mut prev_time = head.time();
            head.multiply(factor);
            for item in tail {
                let time = item.time();
                if time == prev_time + 1 {
                    *item.time_mut() = prev_time * factor + 1;
                } else {
                    item.multiply(factor);
                }
                prev_time = time;
            }
        }
    }

    #[inline]
    pub(crate) fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            preceded(
                spaced(tag("[Events]")),
                curlied(many1(spaced(GlobalEvent::parse))),
            ),
            Self::new,
        )(input)
    }
}

impl<'a> Display for Events<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Events]\n{{\n{}}}\n",
            self.events
                .iter()
                .map(GlobalEvent::to_string)
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_global_events() {
        Events::parse(include_str!("test_data/test_events.txt")).unwrap();
    }
}
