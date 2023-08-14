use url::Url;

use crate::dto::request_dto::Request;

pub struct Validator<'a> {
    pub request: &'a Request,
}

impl Validator<'_> {
    pub fn new(request: &Request) -> Validator {
        Validator { request: &request }
    }
    pub fn validate_url(&self) {
        match Url::parse(&self.request.link) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Invalid URL: {}", e);
                std::process::exit(0);
            }
        }
    }
}
