extern crate colored;

use std::process::exit;

use reqwest;

use crate::builders::response_builder::ResponseBuilder;
use crate::dto::request_dto::Request;
use crate::printer::Printer;
use crate::request_type::RequestType;

pub struct HttpClient<'a> {
    pub request: &'a Request,
}

impl HttpClient<'_> {
    pub fn new(request: &Request) -> HttpClient {
        HttpClient { request }
    }
    pub async fn req(&self) {
        match self.request.req_type {
            RequestType::Get => match self.get().await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error {}", e);
                    exit(0)
                }
            },
            RequestType::Post => match self.post().await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error {}", e);
                    exit(0)
                }
            },
        }
    }
    async fn get(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client.get(&self.request.link).headers(self.request.headers.clone()).send().await?;
        let response_builder = ResponseBuilder::new();
        let res = response_builder.build(RequestType::Get, response).await?;
        let printer = Printer::new(res);
        printer.output();
        Ok(())
    }
    async fn post(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client.post(&self.request.link) // Changed from .get to .post
            .json(&self.request.body)
            .headers(self.request.headers.clone())
            .send()
            .await?;
        let response_builder = ResponseBuilder::new();
        let res = response_builder.build(RequestType::Post, response).await?;
        let printer = Printer::new(res);
        printer.output();
        Ok(())
    }
}