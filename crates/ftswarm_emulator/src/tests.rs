use ftswarm::prelude::*;
use crate::EmulatedSerialPort;

#[tokio::test]
pub async fn test_controller() {
    let ftswarm = FtSwarm::new(EmulatedSerialPort::new());
    let controller = Controller::create(&ftswarm, "controller", ()).await;

    controller.lock().unwrap().set_register(0, 1).await.unwrap();
}
