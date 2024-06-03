use anyhow::{Context, Result};
use tonic::transport::Channel;
use tracing::info;

use crate::settings::CoreSettings;

#[derive(Clone, Debug)]
pub struct {{ ProjectName }}Core {
}

impl {{ ProjectName }}Core {
    pub fn builder() -> Builder {
        Builder::new()
    }
}

pub struct Builder {
    settings: CoreSettings,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            settings: Default::default(),
        }
    }

    pub fn with_settings(mut self, settings: &CoreSettings) -> Self {
        self.settings = settings.clone();
        self
    }

    pub async fn build(self) -> Result<{{ ProjectName }}Core> {
        Ok({{ ProjectName }}Core {})
    }
}