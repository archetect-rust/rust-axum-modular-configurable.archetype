use anyhow::Result;

use {{ project_name }}_core::{{ ProjectName }}Core;
use {{ project_name }}_server::{{ ProjectName }}Server;

mod cli;
mod settings;
mod traces;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = cli::arg_matches();
    let settings = settings::Settings::new(&args)?;
    traces::init(settings.tracing())?;

    match args.subcommand() {
        Some(("config", args)) => match args.subcommand() {
            Some(("defaults", _)) => settings::Settings::default().print()?,
            Some(("merged", _)) => settings.print()?,
            Some(("generate", _)) => settings.generate()?,
            _ => unreachable!(),
        },
        Some((_command, _args)) => {
            unreachable!()
        }
        None => {
            tracing::info!("Initializing...");
            let core = {{ ProjectName }}Core::builder()
                .with_settings(settings.core())
                .build()
                .await?;
            let server = {{ ProjectName }}Server::builder(core)
                .with_settings(settings.server())
                .build()
                .await?;

            tokio::select! {
                result = server.serve() => {
                  return result;
                },
                _ = tokio::signal::ctrl_c() => {
                    return Ok(());
                },
            }
        }
    }

    Ok(())
}
