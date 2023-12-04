//! Event for the Advent of Code 2023.

mod day_1;

use common::{Event, Year};

pub fn event() -> Event {
    Event::new(Year::from(2023), [day_1::puzzle()].into_iter())
}
