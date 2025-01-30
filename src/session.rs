#[derive(Debug, Default)]
pub struct Session {
    closed: bool,
}

impl Session {
    pub fn new() -> Self {
        Self { closed: false }
    }

    pub fn close(&mut self) {
        self.closed = true;
    }

    pub fn closed(&self) -> bool {
        self.closed
    }
}
