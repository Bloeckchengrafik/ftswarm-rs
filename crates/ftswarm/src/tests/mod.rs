use ftswarm_serial::FixedSerialPort;

use crate::prelude::*;

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
async fn test_whoami() {
    let static_serial = FixedSerialPort::new();
    static_serial.add_response("ftSwarm100/example");

    let swarm = FtSwarm::new(static_serial);
    let whoami = swarm.whoami().await.unwrap();
    assert_eq!(whoami.hostname, "example");
    assert_eq!(whoami.id, "ftSwarm100");
    assert_eq!(whoami.serial, Some(100));
}

#[tokio::test]
async fn test_servo() {
    let static_serial = FixedSerialPort::new();
    static_serial.add_response("R: 0");
    static_serial.add_response("R: 10");
    static_serial.add_response("R: Ok");
    static_serial.add_response(" ^ Port not found");

    let swarm = FtSwarm::new(static_serial);
    let servo: Io<Servo> = Servo::create(&swarm, "example", ()).await;
    
    {
        let servo = servo.lock().unwrap();
        assert_eq!(servo.get_position().await, Ok(0));
        assert_eq!(servo.get_offset().await, Ok(10));

        servo.set_offset(32).await.unwrap();
        servo.set_position(32).await.unwrap_err();
    }
}

#[tokio::test]
async fn test_ntc() {
    let static_serial = FixedSerialPort::new();
    static_serial.add_response("R: Ok");
    static_serial.add_response("R: 0");
    static_serial.add_response("R: 10");
    static_serial.add_response("R: Ok");
    static_serial.add_response(" ^ Port not found");

    let swarm = FtSwarm::new(static_serial);
    let ntc: Io<Thermometer> = Thermometer::create(&swarm, "example", Hysteresis(0)).await;
    
    {
        let ntc = ntc.lock().unwrap();
        assert_eq!(ntc.value, 0);
    }
}