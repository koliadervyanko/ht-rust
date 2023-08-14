use reqwest::header::HeaderMap;
use reqwest::StatusCode;

use crate::request_type::RequestType;

pub struct Res {
    pub response: String,
    pub headers: HeaderMap,
    pub status: StatusCode,
    pub method: RequestType,
}