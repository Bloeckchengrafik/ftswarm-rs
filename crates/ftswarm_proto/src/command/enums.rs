use crate::IdOf;

pub enum SensorType {
    Digital,
    Analog,
    Switch,
    ReedSwitch,
    LightBarrier,
    Voltmeter,
    Ohmmeter,
    Thermometer,
    Ldr,
    TrailSensor,
    ColorSensor,
    Ultrasonic,
    CamSensor,
    Counter,
    RotaryEncoder,
    FrequencyMeter,
}

impl IdOf for SensorType {
    fn id(&self) -> u32 {
        match self {
            SensorType::Digital => 0,
            SensorType::Analog => 1,
            SensorType::Switch => 2,
            SensorType::ReedSwitch => 3,
            SensorType::LightBarrier => 4,
            SensorType::Voltmeter => 5,
            SensorType::Ohmmeter => 6,
            SensorType::Thermometer => 7,
            SensorType::Ldr => 8,
            SensorType::TrailSensor => 9,
            SensorType::ColorSensor => 10,
            SensorType::Ultrasonic => 11,
            SensorType::CamSensor => 12,
            SensorType::Counter => 13,
            SensorType::RotaryEncoder => 14,
            SensorType::FrequencyMeter => 15,
        }
    }
}

pub enum ActorType {
    Motor,
    XMMotor,
    Tractor,
    Encoder,
    Lamp,
    Valve,
    Compressor,
    Buzzer,
    Stepper,
}

impl IdOf for ActorType {
    fn id(&self) -> u32 {
        match self {
            ActorType::Motor => 0,
            ActorType::XMMotor => 1,
            ActorType::Tractor => 2,
            ActorType::Encoder => 3,
            ActorType::Lamp => 4,
            ActorType::Valve => 5,
            ActorType::Compressor => 6,
            ActorType::Buzzer => 7,
            ActorType::Stepper => 8,
        }
    }
}

pub enum MotionType {
    Coast,
    Brake,
    On,
}

impl IdOf for MotionType {
    fn id(&self) -> u32 {
        match self {
            MotionType::Coast => 0,
            MotionType::Brake => 1,
            MotionType::On => 2,
        }
    }
}