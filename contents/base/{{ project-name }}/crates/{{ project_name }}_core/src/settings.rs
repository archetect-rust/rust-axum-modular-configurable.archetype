use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoreSettings {
}

impl CoreSettings {
    pub fn new() -> CoreSettings {
        CoreSettings {
        }
    }
}

impl Default for CoreSettings {
    fn default() -> Self {
        CoreSettings {
        }
    }
}
