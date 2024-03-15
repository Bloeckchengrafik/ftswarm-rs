use ftswarm::prelude::*;
use ftswarm_emulator::EmulatedSerialPort;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .parse_default_env()
        .filter_module("ftswarm_emulator", log::LevelFilter::Debug)
        .init();
    let swarm = FtSwarm::new(EmulatedSerialPort::new());

    println!("WhoAmI: {:?}", swarm.whoami().await);

    let motor = XMMotor::create(&swarm, "hello", ()).await;
    motor.lock().unwrap().set(100).await.unwrap();

    let switch = Switch::create(&swarm, "hello", NormallyOpen::Open).await;
    println!("Switch: {:?}", switch.lock().unwrap().value);
}