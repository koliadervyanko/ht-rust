use std::error::Error;
use std::process::exit;

use crate::converters::request_converter::RequestConverter;
use crate::dto::request_dto::RequestDto;
use crate::request_type::RequestType;

pub struct RequestBuilder {
    pub req_type: RequestType,
    pub link: String,
    pub body: String,
    pub headers: String,
}

impl RequestBuilder {
    pub fn new(req_type: RequestType,
               link: String,
               body: String,
               headers: String) -> RequestBuilder {
        RequestBuilder { req_type, link, body, headers }
    }
    pub fn build(&self) -> RequestDto {
        match self.get_request() {
            Ok(request) => request,
            Err(e) => {
                eprintln!("{}", e);
                exit(0);
            }
        }
    }
    fn get_request(&self) -> Result<RequestDto, Box<dyn Error>> {
        let converter = RequestConverter::new();
        let headers = converter.convert_headers(&self.headers);
        let body = converter.convert_body(&self.body)?;
        let request = RequestDto::new(self.req_type.clone(), self.link.clone(), body, headers);
        Ok(request)
    }
}
