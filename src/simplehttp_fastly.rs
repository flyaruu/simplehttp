use std::str::from_utf8;

use crate::simplehttp::{SimpleHttpClient, SimpleHttpError, HttpResponse};
use fastly::{backend::Backend, Request};
use log::info;
pub struct SimpleHttpClientFastly {
    backend_name: String,
}

impl SimpleHttpClientFastly {
    pub fn new(backend_name: &str)->SimpleHttpClientFastly {
        SimpleHttpClientFastly { backend_name: backend_name.to_owned() }
    }
}
impl SimpleHttpClient for SimpleHttpClientFastly {
    fn custom(&mut self, method: crate::simplehttp::Method, url: &str, headers: &[(&str, &str)], body: Option<&[u8]>)->Result<crate::simplehttp::HttpResponse,crate::simplehttp::SimpleHttpError> {
            // Create a new request.
        println!("Coming!");
        let mut request = match method {
            crate::simplehttp::Method::Options =>  Request::options(url),
            crate::simplehttp::Method::Get => Request::get(url),
            crate::simplehttp::Method::Post =>  Request::post(url),
            crate::simplehttp::Method::Put =>  Request::put(url),
            crate::simplehttp::Method::Delete =>  Request::delete(url),
            crate::simplehttp::Method::Head =>  Request::head(url),
            crate::simplehttp::Method::Trace =>  Request::trace(url),
            crate::simplehttp::Method::Connect =>  Request::connect(url),
            crate::simplehttp::Method::Patch =>  Request::patch(url),
        };
        for (header_name, header_value) in headers {
            request = request.with_header(*header_name, *header_value)
        }
        if let Some(body) = body {
            println!("With body. size: {}",body.len());
            let body_utf = from_utf8(body).unwrap();
            println!("indeed: >>{}",body_utf);
            request = request.with_body(body)
        }
        if request.has_body() {
            println!("I can boogie!");
        } else {
            println!("no!");
        }

        
        let backend = Backend::from_name(&self.backend_name)
            .map_err(|e| SimpleHttpError::Nested("Error creating backend".to_owned(), Box::new(e)))?;

        let response = request.send(backend)
            .map_err(|x| SimpleHttpError::Nested("Error sending request to Fastly backend".to_owned(), Box::new(x)))?;
        Ok(HttpResponse {
            status_code: response.get_status().as_u16(),
            response_headers: response.get_headers().map(|(name,v)| (name.to_string(),v.to_str().unwrap().to_owned())).collect(),
            body: response.into_body().into_bytes(),
        })
    }
}