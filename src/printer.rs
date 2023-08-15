extern crate colored;

use std::process::exit;

use colored::Colorize;
use http::HeaderMap;

use crate::dto::request_dto::RequestDto;
use crate::dto::response_dto::ResponseDto;
use crate::request_type::RequestType;

pub struct Printer {
    pub res: ResponseDto,
    pub req: RequestDto,
}

impl Printer {
    pub fn new(res: ResponseDto, req: RequestDto) -> Printer {
        Printer { res, req }
    }

    /// This function print the response
    ///
    /// # Example
    ///
    /// ```rust
    /// let printer = Printer::new(res, request);
    /// printer.output();
    /// ```
    ///
    /// # Returns
    /// > **Print response to the console**
    ///
    /// This method print response to the console
    pub fn output(&self) {
        if self.res.status.is_success() {
            println!("Status {}", self.res.status.to_string().green());
        } else {
            println!("Status {}", self.res.status.to_string().red());
        }
        let date = self.get_header(&self.res.headers, "date".to_string());
        let connection = self.get_header(&self.res.headers, "connection".to_string());
        let content_type = self.get_header(&self.res.headers, "content-type".to_string());
        let method = &self.get_method();
        println!("Method: {}", method.yellow());
        println!("Date: {}", date.yellow());
        println!("Connection: {}", connection.yellow());
        println!("Content-type: {}", content_type.yellow());
        println!("{}", self.res.response);
    }
    fn get_method(&self) -> String {
        let method = match self.req.req_type {
            RequestType::Get => "GET".to_string(),
            RequestType::Post => "POST".to_string()
        };
        method
    }
    fn get_header(&self, headers: &HeaderMap, value: String) -> String {
        let header = if let Some(date) = headers.get(&value) {
            date
        } else {
            eprintln!("Error: Key named {} not found", &value);
            exit(0)
        };
        header.to_str().unwrap().to_string()
    }
}