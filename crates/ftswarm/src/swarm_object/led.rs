use ftswarm_macros::{default_new_swarm_object_impls, impl_swarm_object};
use ftswarm_proto::command::argument::Argument;
use ftswarm_proto::command::rpc::RpcFunction;
use crate::FtSwarm;
use crate::swarm_object::{NewSwarmObject, SwarmObject, Updateable};

#[derive(Updateable, Clone)]
pub struct Led {
    pub name: String,
    swarm: FtSwarm
}

impl_swarm_object!(Led, ());

impl NewSwarmObject<()> for Led {
    fn new(name: &str, swarm: FtSwarm, _params: ()) -> Box<Self> {
        Box::new(Led {
            name: name.to_string(),
            swarm
        })
    }

    default_new_swarm_object_impls!();
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LedColor {
    pub red: i32,
    pub green: i32,
    pub blue: i32
}

impl LedColor {
    pub fn new(red: i32, green: i32, blue: i32) -> Self {
        LedColor {
            red,
            green,
            blue
        }
    }

    pub fn rgb(red: i32, green: i32, blue: i32) -> LedColor {
        LedColor::new(red, green, blue)
    }

    pub fn hsl(hue: i32, saturation: i32, lightness: i32) -> LedColor {
        let hue = hue as f64 / 360.0;
        let saturation = saturation as f64 / 100.0;
        let lightness = lightness as f64 / 100.0;

        let c = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
        let x = c * (1.0 - ((hue * 6.0) % 2.0 - 1.0).abs());
        let m = lightness - c / 2.0;

        let (red, green, blue) = if hue < 1.0 / 6.0 {
            (c, x, 0.0)
        } else if hue < 2.0 / 6.0 {
            (x, c, 0.0)
        } else if hue < 3.0 / 6.0 {
            (0.0, c, x)
        } else if hue < 4.0 / 6.0 {
            (0.0, x, c)
        } else if hue < 5.0 / 6.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        LedColor::new(
            ((red + m) * 255.0) as i32,
            ((green + m) * 255.0) as i32,
            ((blue + m) * 255.0) as i32
        )
    }

    pub fn red() -> LedColor {
        LedColor::new(255, 0, 0)
    }

    pub fn green() -> LedColor {
        LedColor::new(0, 255, 0)
    }

    pub fn blue() -> LedColor {
        LedColor::new(0, 0, 255)
    }

    pub fn yellow() -> LedColor {
        LedColor::new(255, 255, 0)
    }

    pub fn cyan() -> LedColor {
        LedColor::new(0, 255, 255)
    }

    pub fn magenta() -> LedColor {
        LedColor::new(255, 0, 255)
    }

    pub fn white() -> LedColor {
        LedColor::new(255, 255, 255)
    }

    pub fn off() -> LedColor {
        LedColor::new(0, 0, 0)
    }
}

impl From<String> for LedColor {
    fn from(value: String) -> Self {
        // try match hex

        if value.starts_with("#") {
            let hex = value.trim_start_matches("#");
            let red = i32::from_str_radix(&hex[0..2], 16).unwrap();
            let green = i32::from_str_radix(&hex[2..4], 16).unwrap();
            let blue = i32::from_str_radix(&hex[4..6], 16).unwrap();
            return LedColor::new(red, green, blue);
        }

        // try match rgb
        let mut parts = value.split(",");
        if let (Some(red), Some(green), Some(blue)) = (parts.next(), parts.next(), parts.next()) {
            return LedColor::new(red.parse().unwrap(), green.parse().unwrap(), blue.parse().unwrap());
        }

        // try match normal color
        match value.to_lowercase().as_str() {
            "red" => LedColor::new(255, 0, 0),
            "green" => LedColor::new(0, 255, 0),
            "blue" => LedColor::new(0, 0, 255),
            "yellow" => LedColor::new(255, 255, 0),
            "cyan" => LedColor::new(0, 255, 255),
            "magenta" => LedColor::new(255, 0, 255),
            "white" => LedColor::new(255, 255, 255),
            "black" => LedColor::new(0, 0, 0),
            _ => panic!("Invalid color")
        }
    }
}

impl Into<i64> for LedColor {
    fn into(self) -> i64 {
        ((self.red << 16) | (self.green << 8) | self.blue) as i64
    }
}

impl Led {
    pub async fn set_color(&self, color: LedColor) -> Result<(), String> {
        self.run_command(RpcFunction::SetColor, vec![Argument::Int(color.into())]).await
        .map(|_| ())
    }

    pub async fn set_brightness(&self, brightness: i32) -> Result<(), String> {
        let brightness = brightness.min(255).max(0);
        self.run_command(RpcFunction::SetBrightness, vec![Argument::Int(brightness as i64)]).await
        .map(|_| ())
    }
}
