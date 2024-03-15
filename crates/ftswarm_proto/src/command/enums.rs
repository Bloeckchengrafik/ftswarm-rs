use crate::IdOf;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum ToggleType {
    None,
    Up,
    Down
}

impl IdOf for ToggleType {
    fn id(&self) -> u32 {
        match self {
            ToggleType::None => 0,
            ToggleType::Up => 1,
            ToggleType::Down => 2,
        }
    }
}

impl From<i32> for ToggleType {
    fn from(value: i32) -> Self {
        match value {
            1 => ToggleType::Up,
            2 => ToggleType::Down,
            _ => ToggleType::None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MicroStepMode {
    FullStep,
    HalfStep,
    QuarterStep,
    EighthStep,
    SixteenthStep,
}

impl IdOf for MicroStepMode {
    fn id(&self) -> u32 {
        match self {
            MicroStepMode::FullStep => 0,
            MicroStepMode::HalfStep => 1,
            MicroStepMode::QuarterStep => 2,
            MicroStepMode::EighthStep => 3,
            MicroStepMode::SixteenthStep => 4,
        }
    }
}