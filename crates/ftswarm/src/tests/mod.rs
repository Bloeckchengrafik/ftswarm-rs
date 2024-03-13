use ftswarm_serial::FixedSerialPort;

use crate::{aliases, FtSwarm};

aliases! {
    Outputs {
        LED1 = "led1",
        LED2 = "led2",
    }
}

aliases! {
    Inputs {
        SWITCH = "switch",
    }
}

#[test]
fn test_aliases() {
    assert_eq!(Outputs::LED1, "led1");
    assert_eq!(Outputs::LED2, "led2");
    assert_eq!(Inputs::SWITCH, "switch");
}

#[tokio::test]
async fn test_lowlevel() {
    let static_serial = FixedSerialPort::new();
    static_serial.add_response("ftSwarm100/example");

    let swarm = FtSwarm::new(static_serial);
    let whoami = swarm.whoami().await.unwrap();
    assert_eq!(whoami.hostname, "example");
    assert_eq!(whoami.id, "ftSwarm100");
    assert_eq!(whoami.serial, Some(100));
}