use bytes::Bytes;
use http::{HeaderName, HeaderValue};
use log::debug;
use crate::simplehttp::{SimpleHttpClient, SimpleHttpError, Method, HttpResponse};
pub struct SimpleHttpClientSpin {
}


impl Into<http::Method> for crate::simplehttp::Method {
    fn into(self) -> http::Method {
        match self {
            Method::Options => todo!(),
            Method::Get => todo!(),
            Method::Post => todo!(),
            Method::Put => todo!(),
            Method::Delete => todo!(),
            Method::Head => todo!(),
            Method::Trace => todo!(),
            Method::Connect => todo!(),
            Method::Patch => todo!(),
        }
    }
}


impl SimpleHttpClientSpin {
    pub fn new_spin()->Box<dyn SimpleHttpClient> {
        Box::new(SimpleHttpClientSpin{})
    } 
    fn prepare_request( uri: &str, headers: &[(&str, &str)], body: Option<&[u8]>, method: crate::simplehttp::Method)->Result<http::Request<Option<bytes::Bytes>>,SimpleHttpError> {
        let http_method: crate::simplehttp::Method = method.into();
        let mut request_builder = http::Request::builder()
            .method::<Method>(http_method)
            .uri(uri);
        for (header_key, header_value) in headers {
            request_builder = request_builder.header(HeaderName::from_bytes(header_key.as_bytes()).unwrap(), HeaderValue::from_bytes(header_value.as_bytes()).unwrap());
        }
        
        match body  {
            Some(b) => request_builder.body( Some(bytes::Bytes::from(b.to_vec()))),
            None => request_builder.body(None)
        }.map_err(|e| SimpleHttpError::new_with_cause("Error sending body", Box::new(e)))
    }
}

impl SimpleHttpClient for SimpleHttpClientSpin {

    fn custom(&mut self, method: crate::simplehttp::Method, uri: &str, input_headers: &[(&str, &str)], body: Option<&[u8]>)->Result<HttpResponse,SimpleHttpError> {
        let request = SimpleHttpClientSpin::prepare_request(uri,input_headers,body,method)?;
        let response = spin_sdk::http::send(
            request
        ).map_err(|e|SimpleHttpError::new_with_cause("Error posting", Box::new(e)))?;

        let response_status = response.status();
        let response_headers: Vec<(String,String)> = response.headers().iter().filter_map(|(header_name,header_value)| header_value.to_str().ok().map(|v| (header_name.as_str().to_owned(),v.to_owned())) ).collect();
        let response_body = response.into_body()
            .unwrap_or(Bytes::new())
            .to_vec();
        let response = HttpResponse {
            status_code: response_status.as_u16(),
            response_headers: response_headers,
            body: response_body,
        };
        Ok(response)

    }

}

