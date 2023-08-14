use http::HeaderMap;
use serde_json::Value;

use crate::request_type::RequestType;

#[derive(Debug, Clone)]
pub struct Request {
    pub req_type: RequestType,
    pub link: String,
    pub body: Value,
    pub headers: HeaderMap,
}

impl Request {
    pub fn new(req_type: RequestType,
               link: String,
               body: Value,
               headers: HeaderMap) -> Request {
        Request { headers, body, link, req_type }
    }
}