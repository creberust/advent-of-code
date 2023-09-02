//! Event for the Advent of Code 2022.

use common::{Event, Year};

mod day_1;
mod day_2;
mod day_3;
mod day_4;

pub fn event() -> Event {
    Event::new(
        Year::from(2022),
        [
            day_1::puzzle(),
            day_2::puzzle(),
            day_3::puzzle(),
            day_4::puzzle(),
        ]
        .into_iter(),
    )
}
