use std::error::Error;

use http::{ HeaderMap, HeaderName, HeaderValue };
use serde_json::Value;

pub struct RequestConverter;

impl RequestConverter {
    pub fn new() -> RequestConverter {
        RequestConverter {}
    }
    pub fn convert_body(&self, json: &str) -> Result<Value, Box<dyn Error>> {
        let valid_json = json
            .replace("{ ", "{ \"")
            .replace(": ", "\": \"")
            .replace(", ", "\", \"")
            .replace(" }", "\" }");

        let parsed_json: Value = serde_json::from_str(&valid_json)?;
        Ok(parsed_json)
    }
    pub fn convert_headers(&self, headers: &str) -> HeaderMap {
        let mut map = HeaderMap::new();

        for line in headers.lines() {
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() == 2 {
                if
                    let (Ok(header_name), Ok(header_value)) = (
                        HeaderName::try_from(parts[0].trim()),
                        HeaderValue::try_from(parts[1].trim()),
                    )
                {
                    map.insert(header_name, header_value);
                }
            }
        }

        map
    }
}
