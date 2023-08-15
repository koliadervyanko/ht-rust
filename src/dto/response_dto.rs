use reqwest::header::HeaderMap;
use reqwest::StatusCode;

pub struct ResponseDto {
    pub response: String,
    pub headers: HeaderMap,
    pub status: StatusCode,
}