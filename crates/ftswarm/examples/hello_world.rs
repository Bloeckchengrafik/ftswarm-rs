use log::info;
use ftswarm_proto::command::direct::FtSwarmDirectCommand::Whoami;
use ftswarm_proto::command::FtSwarmCommand;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    // Automatically connects to the first available ftSwarm
    let swarm = ftswarm::FtSwarm::default();
    info!("Connected to the ftSwarm");

    swarm.send_command(FtSwarmCommand::Direct(Whoami));
    info!("Sent Whoami command");
    let response = swarm.read_response().await;
    info!("Received response: {:?}", response);
}