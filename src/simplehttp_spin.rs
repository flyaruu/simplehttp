use http::{HeaderName, HeaderValue, Method};
use log::debug;
use crate::simplehttp::{SimpleHttpClient, SimpleHttpError};
pub struct SimpleHttpClientSpin {
}

impl SimpleHttpClientSpin {
    pub fn new_spin()->Box<dyn SimpleHttpClient> {
        Box::new(SimpleHttpClientSpin{})
    }
    fn prepare_request( uri: &str, headers: &[(&str, &str)], body: Option<Vec<u8>>, method: Method)->Result<http::Request<Option<bytes::Bytes>>,SimpleHttpError> {
        let mut request_builder = http::Request::builder()
            .method(method)
            .uri(uri);
        for (header_key, header_value) in headers {
            request_builder = request_builder.header(HeaderName::from_bytes(header_key.as_bytes()).unwrap(), HeaderValue::from_bytes(header_value.as_bytes()).unwrap());
        }
        
        match body  {
            Some(b) => request_builder.body( Some(bytes::Bytes::from(b))),
            None => request_builder.body(None)
        }.map_err(|e| SimpleHttpError::new_with_cause("Error sending body", Box::new(e)))
    }
}
impl SimpleHttpClient for SimpleHttpClientSpin {

    fn post(&mut self, uri: &str, headers: &[(&str, &str)], body: &[u8])->Result<Vec<u8>,SimpleHttpError> {
        debug!("Posting to uri: {}",uri);
        let request = SimpleHttpClientSpin::prepare_request(uri,headers,Some(body.to_vec()),Method::POST)?;
        let mut res = spin_sdk::http::send(
            request
        ).map_err(|e|SimpleHttpError::new_with_cause("Error posting", Box::new(e)))?;
        let result = res.body_mut().take().unwrap();
        Ok(result.to_vec())
    }


    fn get_with_body(&mut self, url: &str, headers: &[(&str, &str)], body: &[u8])->Result<Vec<u8>,SimpleHttpError> {
        debug!("Posting to uri: {}",url);
        let request = SimpleHttpClientSpin::prepare_request(url,headers,Some(body.to_vec()),Method::GET)?;
        let mut res = spin_sdk::http::send(
            request
        ).map_err(|e|SimpleHttpError::new_with_cause("Error posting", Box::new(e)))?;
        let result = res.body_mut().take().unwrap();
        Ok(result.to_vec())
    }

    fn patch(&mut self, uri: &str, headers: &[(&str, &str)], body: &[u8])->Result<Vec<u8>,SimpleHttpError> {
        debug!("Patching uri: {}",uri);
        let request = SimpleHttpClientSpin::prepare_request(uri,headers,Some(body.to_vec()),Method::POST)?;
        let mut res = spin_sdk::http::send(
            request
        ).map_err(|e|SimpleHttpError::new_with_cause("Error patching", Box::new(e)))?;
        let result = res.body_mut().take().unwrap();
        Ok(result.to_vec())
    }

    fn put(&mut self, uri: &str, headers: &[(&str, &str)], body: &[u8])->Result<Vec<u8>,SimpleHttpError> {
        debug!("Putting to uri: {}",uri);
        let request = SimpleHttpClientSpin::prepare_request(uri,headers,Some(body.to_vec()),Method::PUT)?;
        let mut res = spin_sdk::http::send(
            request
        ).map_err(|e|SimpleHttpError::new_with_cause("Error putting", Box::new(e)))?;
        let result = res.body_mut().take().unwrap();
        Ok(result.to_vec())
    }


    fn get(&mut self, uri: &str, headers: &[(&str, &str)])->Result<Vec<u8>, SimpleHttpError> {
        let request = SimpleHttpClientSpin::prepare_request(uri, headers, None, Method::GET)?;
        let mut res = spin_sdk::http::send(request)
            .map_err(|e| SimpleHttpError::new_with_cause("Error calling get",Box::new(e)))?;
        let result = res.body_mut().take().unwrap();
        Ok(result.to_vec())
    }

    fn delete(&mut self, uri: &str, headers: &[(&str, &str)])->Result<Vec<u8>, SimpleHttpError> {
        let request = SimpleHttpClientSpin::prepare_request(uri, headers, None, Method::DELETE)?;
        let mut res = spin_sdk::http::send(request)
            .map_err(|e| SimpleHttpError::new_with_cause("Error calling delete",Box::new(e)))?;
        let result = res.body_mut().take().unwrap();
        Ok(result.to_vec())
    }

    fn head(&mut self, uri: &str, headers: &[(&str, &str)])->Result<Vec<u8>, SimpleHttpError> {
        let request = SimpleHttpClientSpin::prepare_request(uri, headers, None, Method::HEAD)?;
        let mut res = spin_sdk::http::send(request)
            .map_err(|e| SimpleHttpError::new_with_cause("Error calling head",Box::new(e)))?;
        let result = res.body_mut().take().unwrap();
        Ok(result.to_vec())
    }

}

