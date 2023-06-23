use std::str::from_utf8;

use reqwest::{blocking::Client, header::{HeaderMap, HeaderValue, HeaderName}, Method};

use crate::simplehttp::{SimpleHttpError, SimpleHttpClient};
pub struct SimpleHttpClientReqwest {
    client: Client
}

impl SimpleHttpClientReqwest {
    pub fn new_reqwest()->Result<Box<dyn SimpleHttpClient>,SimpleHttpError> {
        let http_client = Client::builder().build().map_err(|e| SimpleHttpError(format!("Error initializing: {}",e)))?;
        Ok(Box::new(SimpleHttpClientReqwest { client: http_client}))
    }

    pub fn prepare_request(&self, url: &str, headers: &Vec<(String, String)>, body: Option<Vec<u8>>, method: reqwest::Method)->Result<reqwest::blocking::Request, SimpleHttpError> {
        let mut header_map: HeaderMap = HeaderMap::new();
        for (key,value) in headers {
            header_map.append(HeaderName::from_bytes(key.as_bytes()).unwrap(), HeaderValue::from_bytes(value.as_bytes()).unwrap());
        }
        let builder = self.client
            .request(method, url)
            .headers(header_map);

        let builder = match body {
            Some(b) => builder.body(b),
            None => builder,
        };
        builder.build().map_err(|_| SimpleHttpError("Error creating request".to_owned()))
    }

}
impl SimpleHttpClient for SimpleHttpClientReqwest {
    fn post(&mut self, url: &str, headers: &Vec<(String, String)>, body: Vec<u8>)->Result<Vec<u8>,SimpleHttpError> {
        let request = self.prepare_request(url, &headers, Some(body), Method::POST)?;
        let response = self.client.execute(request)
            .map_err(|_| SimpleHttpError("Error sending post".to_owned()))?;
        
        let response_status = response.status();
        let response_body = response.bytes()
            .map_err(|_| SimpleHttpError("Error decoding post response".to_owned()))?
            .to_vec();
        if !response_status.is_success() {
            return Err(SimpleHttpError(format!("Error status code: {}\n body: {}",response_status.as_u16(), from_utf8(&response_body).unwrap())))
        }            
        Ok(response_body)        

    }

    fn get(&mut self, url: &str, headers: &Vec<(String, String)>)->Result<Vec<u8>, SimpleHttpError> {
        let request = self.prepare_request(url, &headers, None, Method::GET)?;
        let result = self.client.execute(request)
            .map_err(|_| SimpleHttpError("Error sending get".to_owned()))?
            .bytes()
            .map_err(|_| SimpleHttpError("Error decoding get response".to_owned()))?
            .to_vec();
        Ok(result)
    }
}



#[cfg(test)]
mod test {
    use super::SimpleHttpClientReqwest;

    #[test]
    fn test1() {
        let mut client = SimpleHttpClientReqwest::new_reqwest();
        let res = client.get("http://localhost:8500", &vec![("Accept".to_owned(),"something".to_owned())]);
        assert!(res.is_err());
        println!("ok: {:?}",res);
    }
}