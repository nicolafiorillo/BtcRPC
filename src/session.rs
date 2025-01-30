use crate::config::{Config, Port};

#[derive(Debug, Default)]
pub struct Session {
    port: Port,
    closed: bool,
}

impl Session {
    pub fn new(config: &Config) -> Self {
        let port = config.network.port();

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
