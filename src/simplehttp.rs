use std::fmt::{Display, Debug};

use thiserror::Error;

#[derive(Debug,Error)]
pub enum SimpleHttpError {
    #[error("HTTP error `{0}`")]
    Simple(String),
    // #[source]
    #[error("HTTP response error `{0}`")]
    Nested(String, Box<dyn std::error::Error>),
    #[error("HTTP response error `{0}`")]
    ResponseError(HttpResponse),
}

impl SimpleHttpError {
    pub fn new(message: &str)->Self {
        SimpleHttpError::Simple(message.to_owned())
    }

    pub fn new_with_cause(message: &str, cause: Box<(dyn std::error::Error + 'static)>)->Self {
        SimpleHttpError::Nested (message.to_owned(), cause)
    }
}


#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: u16,
    pub response_headers: Vec<(String,String)>,
    pub body: Vec<u8>,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Response code: {} header count: {} body size {}",self.status_code,self.response_headers.len(),self.body.len())?;
        Ok(())
    }
}
#[derive(Debug)]
pub enum Method {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
}

impl Into<String> for Method {
    fn into(self) -> String {
        todo!("Derive I think")
    }
}

pub trait SimpleHttpClient: Send {
    fn custom(&mut self, method: Method, url: &str, headers: &[(&str, &str)], body: Option<&[u8]>)->Result<HttpResponse,SimpleHttpError>;
    fn post(&mut self, url: &str, headers: &[(&str, &str)], body: &[u8])->Result<Vec<u8>,SimpleHttpError> {
        let result = self.custom(crate::simplehttp::Method::Post, url, headers, Some(body))?;
        Ok(result.body)
    }

    fn patch(&mut self, url: &str, headers: &[(&str, &str)], body: &[u8])->Result<Vec<u8>,SimpleHttpError> {
        let result = self.custom(crate::simplehttp::Method::Patch, url, headers, Some(body))?;
        Ok(result.body)
    }

    fn put(&mut self, url: &str, headers: &[(&str, &str)], body: &[u8])->Result<Vec<u8>,SimpleHttpError> {
        let result = self.custom(crate::simplehttp::Method::Put, url, headers, Some(body))?;
        Ok(result.body)
    }

    fn get(&mut self, url: &str, headers: &[(&str, &str)])->Result<Vec<u8>, SimpleHttpError> {
        let result = self.custom(crate::simplehttp::Method::Get, url, headers, None)?;
        Ok(result.body)
    }

    fn delete(&mut self, url: &str, headers: &[(&str, &str)])->Result<Vec<u8>, SimpleHttpError> {
        let result = self.custom(crate::simplehttp::Method::Delete, url, headers, None)?;
        Ok(result.body)
    }

    fn head(&mut self, url: &str, headers: &[(&str, &str)])->Result<Vec<u8>, SimpleHttpError> {
        let result = self.custom(crate::simplehttp::Method::Head, url, headers, None)?;
        Ok(result.body)
    }
}

