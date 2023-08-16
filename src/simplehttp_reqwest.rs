use std::str::from_utf8;

use reqwest::{blocking::Client, header::{HeaderMap, HeaderValue, HeaderName}, Method};

use crate::simplehttp::{SimpleHttpError, SimpleHttpClient, HttpResponse};
pub struct SimpleHttpClientReqwest {
    client: Client
}

impl SimpleHttpClientReqwest {
    pub fn new_reqwest()->Result<Box<dyn SimpleHttpClient>,SimpleHttpError> {
        let http_client = Client::builder().build().map_err(|e| SimpleHttpError::new_with_cause("Error initializing",Box::new(e)))?;
        Ok(Box::new(SimpleHttpClientReqwest { client: http_client}))
    }

    pub fn prepare_request(&self, url: &str, headers: &[(&str, &str)], body: Option<&[u8]>, method: reqwest::Method)->Result<reqwest::blocking::Request, SimpleHttpError> {
        let mut header_map: HeaderMap = HeaderMap::new();
        for (key,value) in headers {
            header_map.append(HeaderName::from_bytes(key.as_bytes()).unwrap(), HeaderValue::from_bytes(value.as_bytes()).unwrap());
        }
        let builder = self.client
            .request(method, url)
            .headers(header_map);

        let builder = match body {
            Some(b) => builder.body(b.to_vec()),
            None => builder,
        };
        builder.build().map_err(|e| SimpleHttpError::new_with_cause("Error creating request",Box::new(e)))
    }
}

impl SimpleHttpClient for SimpleHttpClientReqwest {

    fn custom(&mut self, method: crate::simplehttp::Method, url: &str, headers: &[(&str, &str)], body: Option<&[u8]>)->Result<HttpResponse,SimpleHttpError> {

        let request = self.prepare_request(url, headers, body, method.into())?;
        let response = self.client.execute(request)
            .map_err(|e| SimpleHttpError::new_with_cause("Error sending custom",Box::new(e)))?;
        
        let response_status = response.status();
        let response_headers: Vec<(String,String)> = response.headers().iter().filter_map(|(header_name,header_value)| header_value.to_str().ok().map(|v| (header_name.as_str().to_owned(),v.to_owned())) ).collect();
        let response_body = response.bytes()
            .map_err(|e| SimpleHttpError::new_with_cause("Error decoding custom response",Box::new(e)))?
            .to_vec();
        let response = HttpResponse {
            status_code: response_status.as_u16(),
            response_headers: response_headers,
            body: response_body,
        };
        if !response_status.is_success() {
            return Err(SimpleHttpError::ResponseError(response));
        }
        Ok(response)
    }

}

impl Into<Method> for crate::simplehttp::Method {
    fn into(self) -> Method {
        match self {
            crate::simplehttp::Method::Options => Method::OPTIONS,
            crate::simplehttp::Method::Get => Method::GET,
            crate::simplehttp::Method::Post => Method::POST,
            crate::simplehttp::Method::Put => Method::PUT,
            crate::simplehttp::Method::Delete => Method::DELETE,
            crate::simplehttp::Method::Head => Method::HEAD,
            crate::simplehttp::Method::Trace => Method::TRACE,
            crate::simplehttp::Method::Connect => Method::CONNECT,
            crate::simplehttp::Method::Patch => Method::PATCH,
        }
    }
}


#[cfg(test)]
mod test {
    use super::SimpleHttpClientReqwest;

    #[test]
    fn test1() {
        let mut client = SimpleHttpClientReqwest::new_reqwest().unwrap();
        let res = client.get("http://localhost:8500", &vec![("Accept","something")]);
        assert!(res.is_err());
        println!("ok: {:?}",res);
    }
}