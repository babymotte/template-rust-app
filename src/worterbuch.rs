use crate::{config::Config, error::{{crate_name | upper_camel_case}}Result};
use tokio::select;
use tokio_graceful_shutdown::{SubsystemBuilder, SubsystemHandle};
use tracing::info;
use worterbuch_client::{KeyValuePair, OnDisconnect, Worterbuch, topic};

pub async fn start_worterbuch(
    subsys: &SubsystemHandle,
    config: Config,
) -> Result<worterbuch_client::Worterbuch, miette::Error> {
    info!("Starting worterbuch client subsystem");
    let (wb, on_disconnect, _) = worterbuch_client::connect_with_default_config().await?;
    let wbc = wb.clone();
    subsys.start(SubsystemBuilder::new("worterbuch", |subsys| {
        worterbuch(subsys, wbc, on_disconnect, config)
    }));
    Ok(wb)
}

async fn worterbuch(
    subsys: SubsystemHandle,
    wb: Worterbuch,
    on_disconnect: OnDisconnect,
    config: Config,
) -> {{crate_name | upper_camel_case}}Result<()> {
    wb.set_client_name(format!("{}/{}", config.app.name, config.app.instance.name)).await?;
    wb.set_grave_goods(vec![topic!(config.app.name, config.app.instance.name, "#").as_ref()].as_ref())
        .await?;
    wb.set_last_will(&vec![KeyValuePair::of(
        topic!(config.app.name, config.app.instance.name, "running"),
        false,
    )])
    .await?;
    wb.set(topic!(config.app.name, config.app.instance.name, "running"), true)
        .await?;
    select! {
        _ = on_disconnect => {
            subsys.request_shutdown();
        },
        _ = subsys.on_shutdown_requested() => (),
    }
    Ok(())
}
