use std::process::exit;
use std::str::FromStr;

use clap::ArgMatches;

use crate::request_type::RequestType;

pub struct ArgumentParser<'a> {
    pub matches: &'a ArgMatches<'a>,
}

impl ArgumentParser<'_> {
    pub fn new<'a>(matches: &'a ArgMatches<'a>) -> ArgumentParser<'a> {
        ArgumentParser { matches }
    }
    pub fn find_arg(&self, arg: &String) -> Result<String, &'static str> {
        if let Some(url) = self.matches.value_of(arg) {
            Ok(url.to_string())
        } else {
            println!("Give the {} argument", arg);
            Err("Argument was don't provided")
        }
    }
    pub fn get_arg(&self, arg: &String) -> String {
        let arg = match self.find_arg(arg) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(0)
            }
        };

        return arg;
    }
    pub fn get_headers(&self) -> String {
        let mut header = String::new();

        // Get all headers passed with -h
        if let Some(headers_iter) = self.matches.values_of("headers") {
            for value in headers_iter {
                header.push_str(value);
                header.push('\n');
            }
        }

        header
    }
    pub fn get_req_type(&self) -> RequestType {
        if let Some(rtype) = self.matches.value_of("method") {
            match RequestType::from_str(rtype) {
                Ok(RequestType::Get) => RequestType::Get,
                Ok(RequestType::Post) => RequestType::Post,
                Err(e) => {
                    eprintln!("Error {}", e);
                    exit(0)
                }
            }
        } else {
            return RequestType::Get;
        }
    }
    pub fn get_body(&self) -> String {
        let rtype = self.get_req_type();
        match rtype {
            RequestType::Get => "{}".to_string(),
            RequestType::Post => self.get_arg(&"body".to_string())
        }
    }
}