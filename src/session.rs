use crate::config::Port;

#[derive(Debug, Default)]
pub struct Session {
    port: Port,
    closed: bool,
}

impl Session {
    pub fn new(port: Port) -> Self {
        Self {
            port,
            closed: false,
        }
    }

    pub fn close(&mut self) {
        self.closed = true;
    }

    pub fn closed(&self) -> bool {
        self.closed
    }
}
