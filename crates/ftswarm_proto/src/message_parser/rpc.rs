fn parse_result(value: &str) -> Result<(), ()> {
    if value.to_lowercase() == "ok" {
        Ok(())
    } else {
        Err(())
    }
}

#[derive(Debug)]
pub enum RPCReturnParam {
    Ok,
    Int(i32),
    Float(f32),
    String(String),
}

impl From<String> for RPCReturnParam {
    fn from(value: String) -> Self {
        match value.parse::<i32>() {
            Ok(int) => RPCReturnParam::Int(int),
            Err(_) => match value.parse::<f32>() {
                Ok(float) => RPCReturnParam::Float(float),
                Err(_) => match parse_result(&value) {
                    Ok(_) => RPCReturnParam::Ok,
                    Err(_) => RPCReturnParam::String(value),
                },
            },
        }
    }
}

impl RPCReturnParam {
    pub fn as_int(&self) -> Option<i32> {
        match self {
            RPCReturnParam::Int(int) => Some(*int),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        match self {
            RPCReturnParam::Float(float) => Some(*float),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            RPCReturnParam::String(string) => Some(string),
            _ => None,
        }
    }

    pub fn as_ok(&self) -> Option<()> {
        match self {
            RPCReturnParam::Ok => Some(()),
            _ => None,
        }
    }
}