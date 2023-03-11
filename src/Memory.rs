#[derive(Debug)]
pub struct Memory {
    i: i64,
}

impl Memory {
    pub fn new() -> Memory {
        Memory { i: 0 }
    }

    pub fn add(&mut self) {
        self.i += 1;
    }
}