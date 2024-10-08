use log::info;
use tokio::sync::mpsc::channel;
use ftswarm::prelude::*;


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
        .filter_level(log::LevelFilter::Debug)
        .init();

    let (send_color, mut recv_color) = channel(1);

    tokio::spawn(async move {
        // Coroutine that generates a rainbow when requested
        let mut hue = 0;
        loop {
            send_color.send(hue).await.unwrap();
            hue = (hue + 5) % 360;
        }
    });

    // Automatically connects to the first available ftSwarm
    let swarm = FtSwarm::default();

    let response = swarm.whoami().await?;
    info!("WhoAmI: {}", response);

    info!("Halting motors");
    swarm.halt().await;

    info!("Uptime: {:?}", swarm.uptime().await?);

    let switch = Digital::create(&swarm, Aliases::SWITCH, NormallyOpen::Open).await;
    let led1 = Led::create(&swarm, Aliases::LED1, ()).await;
    let led2 = Led::create(&swarm, Aliases::LED2, ()).await;

    led1.lock().await.set_color(LedColor::blue()).await?;
    led2.lock().await.set_color(LedColor::cyan()).await?;

    let mut switch_state = switch.lock().await.value;
    loop {
        let value = switch.lock().await.value;

        if switch_state != value {
            switch_state = value;
            info!("Switch state: {}", switch_state);

            let new_led_color = recv_color.recv().await.unwrap();
            let color = LedColor::hsl(new_led_color, 100, 50);
            led1.lock().await.set_color(color.clone()).await?;
            led2.lock().await.set_color(color).await?;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    }
}