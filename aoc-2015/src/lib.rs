use common::*;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

pub fn event() -> Event {
    Event::new(
        Year::from(2015),
        [
            day_1::puzzle(),
            day_2::puzzle(),
            day_3::puzzle(),
            day_4::puzzle(),
        ]
        .into_iter(),
    )
}
