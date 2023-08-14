extern crate colored;

use std::process::exit;

use colored::Colorize;
use http::HeaderMap;

use crate::dto::response_dto::Res;
use crate::request_type::RequestType;


pub struct Printer {
    pub res: Res,
}

impl Printer {
    pub fn new(res: Res) -> Printer {
        Printer { res }
    }
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
        let method = match self.res.method {
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