use std::{error::Error, fmt::{Display, Debug}};

#[derive(Debug)]
pub struct SimpleHttpError {
    message: String,
    parent: Option<Box<dyn Error + 'static>>,
}

impl SimpleHttpError {
    pub fn new(message: &str)->Self {
        SimpleHttpError { message: message.to_owned(), parent: None }
    }

    pub fn new_with_cause(message: &str, cause: Box<(dyn Error + 'static)>)->Self {
        SimpleHttpError { message: message.to_owned(), parent: Some(cause) }
    }
}

impl Display for SimpleHttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message.as_str())
    }
}

impl Error for SimpleHttpError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.parent.as_deref()
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

}
pub trait SimpleHttpClient: Send {
    fn post(&mut self, url: &str, headers: &[(&str, &str)], data: &[u8])->Result<Vec<u8>,SimpleHttpError>;
    fn put(&mut self, url: &str, headers: &[(&str, &str)], data: &[u8])->Result<Vec<u8>,SimpleHttpError>;
    fn patch(&mut self, url: &str, headers: &[(&str, &str)], data: &[u8])->Result<Vec<u8>,SimpleHttpError>;
    fn get(&mut self, url: &str, headers: &[(&str, &str)])->Result<Vec<u8>, SimpleHttpError>;
    fn head(&mut self, url: &str, headers: &[(&str, &str)])->Result<Vec<u8>, SimpleHttpError>;
    fn delete(&mut self, url: &str, headers: &[(&str, &str)])->Result<Vec<u8>, SimpleHttpError>;
    fn get_with_body(&mut self, url: &str, headers: &[(&str, &str)], data: &[u8])->Result<Vec<u8>,SimpleHttpError>;

}

