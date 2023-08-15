use serde_json::Value;

pub struct ResponseConverter;

impl ResponseConverter {
    pub fn new() -> ResponseConverter {
        ResponseConverter {}
    }

    pub fn convert_body(&self, body: String) -> Result<String, Box<dyn std::error::Error>> {
        let v: Value = match serde_json::from_str(&body) {
            Ok(val) => val,
            Err(err) => return Err(Box::new(err))
        };
        Ok(serde_json::to_string_pretty(&v)?)
    }
}