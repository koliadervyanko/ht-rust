use clap::{App, Arg};

use crate::argument_parser::ArgumentParser;
use crate::builders::request_builder::RequestBuilder;
use crate::http_client::HttpClient;
use crate::validator::Validator;

mod printer;

mod converters {
    pub mod request_converter;
    pub mod response_converter;
}

mod dto {
    pub mod request_dto;
    pub mod response_dto;
}

mod builders {
    pub mod request_builder;
    pub mod response_builder;
}

mod http_client;

mod argument_parser;


mod validator;
pub mod request_type;


#[tokio::main]
async fn main() {
    let matches = App::new("ht")
        .version("1.0")
        .author("Author Kolia Der")
        .about("Does http requests")
        .arg(Arg::with_name("method").short("m").takes_value(true).help("Request type, GET, POST, ..."))
        .arg(Arg::with_name("url").help("URL for the HTTP request").index(1))
        .arg(Arg::with_name("body").help("If you use post method set up the body").number_of_values(1).short("b").takes_value(true))
        .arg(Arg::with_name("headers").short("h").takes_value(true).multiple(true).number_of_values(1).help("Info for authorization"))
        .get_matches_safe();


    match matches {
        Ok(matches) => {
            let argument_parser = ArgumentParser::new(&matches);
            let url = argument_parser.get_arg(&"url".to_string());
            let header = argument_parser.get_headers();
            let req_type = argument_parser.get_req_type();
            let body = argument_parser.get_body();


            let request_builder = RequestBuilder::new(req_type, url, body, header);
            let request = request_builder.build();
            let validator = Validator::new(&request);
            validator.validate_url();
            let http_client = HttpClient::new(&request);
            http_client.req().await;
        }
        Err(e) => {
            eprintln!("Error parsing command line arguments: {}", e);
        }
    }
}

