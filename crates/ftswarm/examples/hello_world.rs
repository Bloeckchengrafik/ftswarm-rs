use log::info;
use ftswarm::{aliases, FtSwarm};


aliases! {
    Aliases {
        SWITCH = "switch",
        LED1 = "led1",
        LED2 = "led2",
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    // Automatically connects to the first available ftSwarm
    let swarm = FtSwarm::default();

    let response = swarm.whoami().await?;
    info!("WhoAmI: {}", response);

    swarm.halt();
    info!("Uptime: {:?}", swarm.uptime().await?);

    Ok(())
}