use http::HeaderMap;
use serde_json::Value;

use crate::request_type::RequestType;

#[derive(Debug, Clone)]
pub struct RequestDto {
    pub req_type: RequestType,
    pub link: String,
    pub body: Value,
    pub headers: HeaderMap,
}

impl RequestDto {
    pub fn new(req_type: RequestType,
               link: String,
               body: Value,
               headers: HeaderMap) -> RequestDto {
        RequestDto { headers, body, link, req_type }
    }
}