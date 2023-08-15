use url::Url;

use crate::dto::request_dto::RequestDto;

pub struct Validator<'a> {
    pub request: &'a RequestDto,
}

impl Validator<'_> {
    pub fn new(request: &RequestDto) -> Validator {
        Validator { request: &request }
    }
    /// This function validate url
    ///
    /// # Example
    ///
    /// ```rust
    /// let validator = Validator::new(&request);
    /// validator.validate_url();
    /// ```
    ///
    /// # Returns
    /// > *Stop or continue program*
    ///
    /// This method can continue or stop program
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
