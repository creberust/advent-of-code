use common::*;

struct Event2022 {}

impl Event2022 {
    fn new() -> Self {
        Self {}
    }
}

impl Event for Event2022 {
    fn solve(&self, _: Day, _: Part) {
        unimplemented!()
    }
}

pub fn event() -> Box<dyn Event> {
    Box::new(Event2022::new())
}
