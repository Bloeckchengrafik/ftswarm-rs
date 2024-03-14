use log::info;
use ftswarm::{aliases, FtSwarm};
use ftswarm::swarm_object::{Digital, NormallyOpen, SwarmObject};


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

    let switch = Digital::create(&swarm, Aliases::SWITCH, NormallyOpen::Open).await;

    let mut switch_state = false;
    loop {
        {
            let switch = switch.lock().unwrap();
            if switch_state != switch.value {
                switch_state = switch.value;
                info!("Switch state: {}", switch_state);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
}