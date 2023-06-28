use http::{HeaderName, HeaderValue, Method};
use crate::simplehttp::{SimpleHttpClient, SimpleHttpError};
pub struct SimpleHttpClientSpin {

}

impl SimpleHttpClientSpin {
    pub fn new_spin()->Box<dyn SimpleHttpClient> {
        Box::new(SimpleHttpClientSpin{})
    }

    fn prepare_request( uri: &str, headers: &Vec<(String, String)>, body: Option<Vec<u8>>, method: Method)->Result<http::Request<Option<bytes::Bytes>>,SimpleHttpError> {
        let mut request_builder = http::Request::builder()
            .method(method)
            .uri(uri);
        for (header_key, header_value) in headers {
            request_builder = request_builder.header(HeaderName::from_bytes(header_key.as_bytes()).unwrap(), HeaderValue::from_bytes(header_value.as_bytes()).unwrap());
        }
        
        match body  {
            Some(b) => request_builder.body( Some(bytes::Bytes::from(b))),
            None => request_builder.body(None)
        }.map_err(|_| SimpleHttpError::new("Error sending body"))
    }
}
impl SimpleHttpClient for SimpleHttpClientSpin {

    fn post(&mut self, uri: &str, headers: &Vec<(String, String)>, body: Vec<u8>)->Result<Vec<u8>,SimpleHttpError> {
        println!("Posting to uri: {}",uri);
        let request = SimpleHttpClientSpin::prepare_request(uri,headers,Some(body),Method::POST)?;
        let mut res = spin_sdk::http::send(
            request
        ).expect("response error");
        let result = res.body_mut().take().unwrap();
        println!("Bytes: {}",result.len());
        Ok(result.to_vec())
    }

    fn get(&mut self, uri: &str, headers: &Vec<(String, String)>)->Result<Vec<u8>, SimpleHttpError> {
        let request = SimpleHttpClientSpin::prepare_request(uri, headers, None, Method::GET)?;
        let mut res = spin_sdk::http::send(request)
            .map_err(|_| SimpleHttpError::new("Error calling get"))
        .unwrap();
        let result = res.body_mut().take().unwrap();
        Ok(result.to_vec())
    }

}

