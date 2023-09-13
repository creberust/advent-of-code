use common::*;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

pub fn event() -> Event {
    Event::new(
        Year::from(2015),
        [
            day_1::puzzle(),
            day_2::puzzle(),
            day_3::puzzle(),
            day_4::puzzle(),
            day_5::puzzle(),
            day_6::puzzle(),
            day_7::puzzle(),
        ]
        .into_iter(),
    )
}
