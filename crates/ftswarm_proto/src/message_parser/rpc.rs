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