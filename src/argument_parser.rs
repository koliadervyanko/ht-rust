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
    /// This function gives you argument from arguments
    ///
    /// # Example
    ///
    /// ```rust
    /// let argument_parser = ArgumentParser::new(&matches);
    /// let arg = argument_parser.get_arg(&"url".to_string())
    /// ```
    ///
    /// # Returns
    /// > *This method returns your String*
    ///
    /// Returns string
    pub fn get_arg(&self, arg: &String) -> String {
        match self.find_arg(arg) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(0)
            }
        }
    }
    /// This function gives you body from arguments
    ///
    /// # Example
    ///
    /// ```rust
    /// let argument_parser = ArgumentParser::new(&matches);
    /// let headers = argument_parser.get_headers()
    /// ```
    ///
    /// # Returns
    /// > *This method returns your String*
    ///
    /// Returns string with some format
    pub fn get_headers(&self) -> String {
        let mut header = String::new();

        if let Some(headers_iter) = self.matches.values_of("headers") {
            for value in headers_iter {
                header.push_str(value);
                header.push('\n');
            }
        }

        header
    }
    /// This function gives you body from arguments
    ///
    /// # Example
    ///
    /// ```rust
    /// let argument_parser = ArgumentParser::new(&matches);
    /// let req_type = argument_parser.get_req_type()
    /// ```
    ///
    /// # Returns
    /// > *This method returns your RequestType enum option*
    ///
    /// RequestType::Get or RequestType::Post

    pub fn get_req_type(&self) -> RequestType {
        if let Some(rtype) = self.matches.value_of("method") {
            match RequestType::from_str(rtype) {
                Ok(RequestType::Get) => RequestType::Get,
                Ok(RequestType::Post) => RequestType::Post,
                Ok(RequestType::Delete) => RequestType::Delete,
                Ok(RequestType::Put) => RequestType::Put,
                Err(e) => {
                    eprintln!("Error {}", e);
                    exit(0)
                }
            }
        } else {
            RequestType::Get
        }
    }
    /// This function gives you request type from arguments
    ///
    /// # Example
    ///
    /// ```rust
    /// let argument_parser = ArgumentParser::new(&matches);
    /// let body = argument_parser.get_body()
    /// ```
    ///
    /// # Returns
    /// > *This method returns your String*
    ///
    /// If method is **get** returns **{}**
    ///
    /// If method is **post** returns **body** from **arguments**

    pub fn get_body(&self) -> String {
        let rtype = self.get_req_type();
        match rtype {
            RequestType::Get => "{}".to_string(),
            RequestType::Post => self.get_arg(&"body".to_string()),
            RequestType::Delete => "{}".to_string(),
            RequestType::Put => self.get_arg(&"body".to_string()),
        }
    }
}
