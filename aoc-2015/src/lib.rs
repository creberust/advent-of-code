use common::*;

mod day_1;
mod day_2;

pub fn event() -> Event {
    Event::new(
        Year::from(2015),
        [day_1::puzzle(), day_2::puzzle()].into_iter(),
    )
}
