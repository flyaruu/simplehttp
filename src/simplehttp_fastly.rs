use crate::simplehttp::{SimpleHttpClient, SimpleHttpError, HttpResponse};
use fastly::{backend::Backend, Request};
pub struct SimpleHttpClientFastly {}

impl SimpleHttpClient for SimpleHttpClientFastly {
    fn custom(&mut self, method: crate::simplehttp::Method, url: &str, headers: &[(&str, &str)], body: Option<&[u8]>)->Result<crate::simplehttp::HttpResponse,crate::simplehttp::SimpleHttpError> {
            // Create a new request.
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
            request = request.with_body(body)
        }
        
        let backend = Backend::builder("target", url)
            .finish()
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