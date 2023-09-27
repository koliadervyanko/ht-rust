use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum RequestType {
    Get,
    Post,
    Delete,
    Put,
}

impl FromStr for RequestType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "get" => Ok(RequestType::Get),
            "post" => Ok(RequestType::Post),
            "del" => Ok(RequestType::Delete),
            "put" => Ok(RequestType::Put),
            _ => Err("Unsupported request type.".to_string()),
        }
    }
}
