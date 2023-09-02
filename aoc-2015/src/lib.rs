use common::*;

mod day_1;

pub fn event() -> Event {
    Event::new(Year::from(2015), [day_1::puzzle()].into_iter())
}
