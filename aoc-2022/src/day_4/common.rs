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
