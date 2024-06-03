use serde::{Deserialize, Serialize};

const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 8080;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    host: String,
    port: u16,
}

impl ServerSettings {
    pub fn host(&self) -> &str {
        self.host.as_str()
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn set_port(&mut self, port: u16) -> &mut ServerSettings {
        self.port = port;
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
}

impl Default for ServerSettings {
    fn default() -> Self {
        ServerSettings {
            host: String::from(DEFAULT_HOST),
            port: DEFAULT_PORT,
        }
    }
}
