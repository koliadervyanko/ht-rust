use std::process::exit;

use reqwest::Response;

use crate::converters::response_converter::ResponseConverter;
use crate::dto::response_dto::ResponseDto;

pub struct ResponseBuilder;

impl ResponseBuilder {
    pub fn new() -> ResponseBuilder {
        ResponseBuilder
    }
    pub async fn build(&self, response: Response) -> ResponseDto {
        match self.get_response(response).await {
            Ok(response_dto) => response_dto,
            Err(e) => {
                eprintln!("Error building response: {}", e);
                exit(0)
            }
        }
    }
    async fn get_response(&self, response: Response) -> Result<ResponseDto, Box<dyn std::error::Error>> {
        let status = response.status();
        let headers = response.headers().clone();
        let body_text = response.text().await?;
        let response_converter = ResponseConverter::new();


        let handled_res = match response_converter.convert_body(body_text) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error handling response body: {}", e);
                exit(0);
            }
        };
        let dto = ResponseDto { response: handled_res, status, headers };
        Ok(dto)
    }
}