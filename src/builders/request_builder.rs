use std::error::Error;
use std::process::exit;

use crate::converters::request_converter::RequestConverter;
use crate::dto::request_dto::Request;
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
    pub fn build(&self) -> Request {
        match self.handle() {
            Ok(built) => built,
            Err(e) => {
                eprintln!("{}", e);
                exit(0);
            }
        }
    }
    pub fn handle(&self) -> Result<Request, Box<dyn Error>> {
        let converter = RequestConverter::new();
        let headers = converter.convert_headers(&self.headers);
        let body = converter.convert_body(&self.body)?;
        let request = Request::new(self.req_type.clone(), self.link.clone(), body, headers);
        Ok(request)
    }
}
