pub struct Sections {
    start: u32,
    end: u32,
}

impl Sections {
    pub fn new(start: u32, end: u32) -> Self {
        Sections { start, end }
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    pub fn end(&self) -> u32 {
        self.end
    }
}

pub fn full_overlaping(a: &Sections, b: &Sections) -> bool {
    (a.start() <= b.start() && a.end() >= b.end()) || (b.start() <= a.start() && b.end() >= a.end())
}

pub fn not_fully_overlaping(a: &Sections, b: &Sections) -> bool {
    a.end() < b.start() || b.end() < a.start()
}
