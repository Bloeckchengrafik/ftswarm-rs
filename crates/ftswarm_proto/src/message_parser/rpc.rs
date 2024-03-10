fn parse_result(value: &str) -> Result<(), ()> {
    if value.to_lowercase() == "ok" {
        Ok(())
    } else {
        Err(())
    }
}

#[derive(Debug)]
pub enum ReturnParam {
    Ok,
    Int(i32),
    Float(f32),
    String(String),
}

impl From<String> for ReturnParam {
    fn from(value: String) -> Self {
        match value.parse::<i32>() {
            Ok(int) => ReturnParam::Int(int),
            Err(_) => match value.parse::<f32>() {
                Ok(float) => ReturnParam::Float(float),
                Err(_) => match parse_result(&value) {
                    Ok(_) => ReturnParam::Ok,
                    Err(_) => ReturnParam::String(value),
                },
            },
        }
    }
}