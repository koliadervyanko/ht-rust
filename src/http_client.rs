extern crate colored;

use std::process::exit;

use reqwest;

use crate::builders::response_builder::ResponseBuilder;
use crate::dto::request_dto::RequestDto;
use crate::dto::response_dto::ResponseDto;
use crate::request_type::RequestType;

pub struct HttpClient<'a> {
    pub request: &'a RequestDto,
}

impl HttpClient<'_> {
    pub fn new(request: &RequestDto) -> HttpClient {
        HttpClient { request }
    }
    pub async fn req(&self) -> ResponseDto {
        match self.request.req_type {
            RequestType::Get => match self.get().await {
                Ok(dto) => dto,
                Err(e) => {
                    eprintln!("Error {}", e);
                    exit(0)
                }
            },
            RequestType::Post => match self.post().await {
                Ok(dto) => dto,
                Err(e) => {
                    eprintln!("Error {}", e);
                    exit(0)
                }
            },
        }
    }
    // Move printer to main.rs
    async fn get(&self) -> Result<ResponseDto, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client.get(&self.request.link).headers(self.request.headers.clone()).send().await?;
        let response_builder = ResponseBuilder::new();
        let res = response_builder.build(response).await;
        Ok(res)
    }
    async fn post(&self) -> Result<ResponseDto, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client.post(&self.request.link) // Changed from .get to .post
            .json(&self.request.body)
            .headers(self.request.headers.clone())
            .send()
            .await?;
        let response_builder = ResponseBuilder::new();
        let res = response_builder.build(response).await;
        Ok(res)
    }
}