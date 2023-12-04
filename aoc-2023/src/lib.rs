//! Event for the Advent of Code 2023.

mod day_1;
mod day_2;

use common::{Event, Year};

pub fn event() -> Event {
    Event::new(
        Year::from(2023),
        [day_1::puzzle(), day_2::puzzle()].into_iter(),
    )
}
