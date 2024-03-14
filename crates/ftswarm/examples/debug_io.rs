use log::info;
use ftswarm::prelude::*;

#[tokio::main]
async fn main() -> Result<(), String> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .filter_module("ftswarm_serial", log::LevelFilter::Trace)
        .init();

    // Automatically connects to the first available ftSwarm
    let swarm = FtSwarm::default();

    let response = swarm.whoami().await?;
    info!("WhoAmI: {}", response);

    Ok(())
}