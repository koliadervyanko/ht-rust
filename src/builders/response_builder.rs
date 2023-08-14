use std::process::exit;

use reqwest::Response;

use crate::converters::response_converter::ResponseConverter;
use crate::dto::response_dto::Res;
use crate::request_type::RequestType;

pub struct ResponseBuilder;

impl ResponseBuilder {
    pub fn new() -> ResponseBuilder {
        ResponseBuilder {}
    }
    pub async fn build(&self, req_type: RequestType, response: Response) -> Result<Res, Box<dyn std::error::Error>> {
        let status = response.status();
        let headers = response.headers().clone();
        let body_text = response.text().await?;
        let response_converter = ResponseConverter::new();


        let handled_res = match response_converter.handle_body(body_text) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error handling response body: {}", e);
                exit(0);
            }
        };
        let server_res = Res { response: handled_res, status, headers, method: req_type };
        Ok(server_res)
    }
}