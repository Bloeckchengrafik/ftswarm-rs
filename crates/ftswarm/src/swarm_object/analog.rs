use std::future::Future;
use ftswarm_macros::{default_new_swarm_object_impls, impl_swarm_object, impl_int_updateable};
use ftswarm_proto::command::argument::Argument;
use ftswarm_proto::command::enums::SensorType;
use ftswarm_proto::command::rpc::RpcFunction;
use ftswarm_proto::message_parser::rpc::RPCReturnParam;
use crate::FtSwarm;
use crate::swarm_object::{Hysteresis, NewSwarmObject, NormallyOpen, SwarmObject, Updateable};
use ftswarm_macros::analog_swarm_object;

analog_swarm_object!(Analog);
analog_swarm_object!(ColorSensor);
analog_swarm_object!(Ldr);
analog_swarm_object!(Thermometer);
analog_swarm_object!(Ohmmeter);
analog_swarm_object!(TrailSensor);
analog_swarm_object!(Ultrasonic);
analog_swarm_object!(Voltmeter);

impl Thermometer {
    pub async fn get_kelvin(&self) -> Result<f32, String> {
        return self.run_command(RpcFunction::GetKelvin, vec![])
            .await
            .and_then(|param| param.as_float().ok_or("Failed to get kelvin".to_string()));
    }

    pub async fn get_celsius(&self) -> Result<f32, String> {
        return self.run_command(RpcFunction::GetCelsius, vec![])
            .await
            .and_then(|param| param.as_float().ok_or("Failed to get celsius".to_string()));
    }

    pub async fn get_fahrenheit(&self) -> Result<f32, String> {
        return self.run_command(RpcFunction::GetFahrenheit, vec![])
            .await
            .and_then(|param| param.as_float().ok_or("Failed to get fahrenheit".to_string()));
    }
}

impl Ohmmeter {
    pub async fn get_resistance(&self) -> Result<f32, String> {
        return self.run_command(RpcFunction::GetResistance, vec![])
            .await
            .and_then(|param| param.as_float().ok_or("Failed to get resistance".to_string()))
    }
}

impl Voltmeter {
    pub async fn get_voltage(&self) -> Result<f32, String> {
        return self.run_command(RpcFunction::GetVoltage, vec![])
            .await
            .and_then(|param| param.as_float().ok_or("Failed to get voltage".to_string()))
    }
}