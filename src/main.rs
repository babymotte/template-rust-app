use miette::Result;
use std::{io, time::Duration};
use {{crate_name | snake_case}}::{config::Config, webserver::start_webserver, worterbuch::start_worterbuch};
use tokio_graceful_shutdown::{SubsystemBuilder, SubsystemHandle, Toplevel};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::load().await?;

    info!("Starting {} instance '{}' …", config.app.name, config.app.instance.name);

    Toplevel::new(|s| async move {
        s.start(SubsystemBuilder::new("{{crate_name | kebab_case}}", |s| {
            run(s, config)
        }));
    })
    .catch_signals()
    .handle_shutdown_requests(Duration::from_secs(1))
    .await?;

    Ok(())
}

async fn run(subsys: SubsystemHandle, config: Config) -> Result<()> {
    let wb = start_worterbuch(&subsys, config.clone()).await?;

    start_webserver(&subsys, config, wb);

    subsys.on_shutdown_requested().await;

    Ok(())
}
