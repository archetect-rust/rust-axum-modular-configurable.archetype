use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;
use {{ project_name }}_core::{{ ProjectName }}Core;
use crate::routes::router;
use crate::settings::ServerSettings;

pub struct {{ ProjectName }}Server {
    core: {{ ProjectName }}Core,
    port: u16,
    listener: TcpListener,
}

pub struct Builder {
    settings: ServerSettings,
    core: {{ ProjectName }}Core,
}

impl Builder {
    pub fn new(core: {{ ProjectName }}Core) -> Builder {
        Builder {
            settings: ServerSettings::default(),
            core,
        }
    }

    pub fn with_settings(mut self, settings: &ServerSettings) -> Builder {
        self.settings = settings.clone();
        self
    }

    pub fn with_random_port(mut self) -> Builder {
        self.settings.set_port(0);
        self
    }

    pub async fn build(self) -> Result<{{ ProjectName }}Server> {
        let listener = TcpListener::bind((self.settings.host(), self.settings.port())).await?;
        let addr = listener.local_addr()?;

        Ok({{ ProjectName }}Server {
            core: self.core,
            port: addr.port(),
            listener,
        })
    }

}


impl {{ ProjectName }}Server {
    pub fn builder(core: {{ ProjectName }}Core) -> Builder {
        Builder::new(core)
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn serve(self) -> Result<()> {
        let listener = self.listener;
        tracing::info!("{{ ProjectName }} starting on http://{}", listener.local_addr()?);
        axum::serve(listener, router()).await?;
        Ok(())
    }
}
