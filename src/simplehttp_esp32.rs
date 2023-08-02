use embedded_svc::http::{client::*, Headers};
use embedded_svc::io::Write;
use esp_idf_svc::http::client::*;
use log::{warn, info};

use crate::simplehttp::{SimpleHttpClient, SimpleHttpError};

pub fn new_esp_http()->Box<dyn SimpleHttpClient> {
    Box::new(EspSimpleHttpClient::new().unwrap())
}

pub fn new_esp_http_debug()->Box<dyn SimpleHttpClient> {
    Box::new(EspSimpleHttpClient::new_debug().unwrap())
}


pub struct EspSimpleHttpClient {
    client: Client<EspHttpConnection>,
    debug: bool,
}
impl EspSimpleHttpClient {
    pub fn new_debug()->Result<EspSimpleHttpClient,SimpleHttpError> {
        let mut instance = Self::new()?;
        instance.debug = true;
        Ok(instance)
    }
    pub fn new()->Result<EspSimpleHttpClient,SimpleHttpError> {
        let client = Client::wrap(EspHttpConnection::new(&Configuration {
            crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
            ..Default::default()
        }).map_err(|_| SimpleHttpError::new("Error creating http client"))?);
        Ok(EspSimpleHttpClient{client, debug: false})
    }

    fn read_response(mut response: Response<&mut EspHttpConnection>)->Result<Vec<u8>,SimpleHttpError> {
        let size = response.content_len()
            .ok_or(SimpleHttpError::new("Error reading content length"))? as usize;
        let mut body = [0_u8; 3048];
        let mut output_buffer: Vec<u8> = Vec::with_capacity(size);
        loop {
            match response.read(&mut body) {
                Ok(bytes_read) => {
                    if bytes_read>0 {
                        output_buffer.extend_from_slice(&body[0..bytes_read]);
                    } else {
                        return Ok(output_buffer);
                    }
                },
                Err(e) => return Err(SimpleHttpError::new_with_cause("Error reading content",Box::new(e))),
            };
        }
    }

    fn check_debug_request(&self, method: Method, url: &str, input_headers: &[(&str,&str)], _body: Option<&[u8]>) {
        if url.contains("localhost") {
            warn!("\n\n!!!! Do you really want to use localhost from esp? I doubt that.")
        }
        if self.debug {
            info!("Calling {:?} {} {:?}",method, &url, input_headers);
            // todo: debug request body
        }        
    }
}


unsafe impl Send for EspSimpleHttpClient {}

impl SimpleHttpClient for EspSimpleHttpClient {
    
    fn get(&mut self, url: &str, input_headers: &[(&str, &str)])->Result<Vec<u8>, SimpleHttpError> {
        self.check_debug_request(Method::Get, url, input_headers, None);
        let response = self.client
            .request(Method::Get,&url,&input_headers)
            .map_err(|e| SimpleHttpError::new_with_cause("Error creating  get: {}",Box::new(e)))?
            .submit()
            .map_err(|e| SimpleHttpError::new_with_cause("Error connecting",Box::new(e)))?;
        Self::read_response(response)
    }

    fn get_with_body<'a>(&'a mut self, url: &str, input_headers: &[(&str, &str)], data: &[u8])->Result<Vec<u8>,SimpleHttpError> {
        self.check_debug_request(Method::Post, url, input_headers, Some(data));

        let length_string = format!("{}",data.len());
        let mut headers = input_headers.to_vec();
        headers.push(("Content-Length",&length_string));        
        let collected: Vec<(&str,&str)> = headers.iter().map(|(k,v)|(k.as_ref(),v.as_ref())).collect();
        let mut get_request: Request<&mut EspHttpConnection> = self.client
            .request(Method::Get,url,&collected)
            .map_err(|e| SimpleHttpError::new_with_cause("Error posting url",Box::new(e)))?;
        get_request.write_all(&data).map_err(|e| SimpleHttpError::new_with_cause(&format!("Error getting(with body) url: {:?}",url),Box::new(e)))?;
        let get_request = get_request.submit()
                .map_err(|e| SimpleHttpError::new_with_cause("Error sending data",Box::new(e)))?;
        Self::read_response(get_request)     
    }

    fn head(&mut self, url: &str, input_headers: &[(&str, &str)])->Result<Vec<u8>, SimpleHttpError> {
        self.check_debug_request(Method::Head, url, input_headers, None);
        let response = self.client
            .request(Method::Head,&url,&input_headers)
            .map_err(|e| SimpleHttpError::new_with_cause("Error creating head: {}",Box::new(e)))?
            .submit()
            .map_err(|e| SimpleHttpError::new_with_cause("Error connecting",Box::new(e)))?;
        Self::read_response(response)
    }    
    fn delete(&mut self, url: &str, input_headers: &[(&str, &str)])->Result<Vec<u8>, SimpleHttpError> {
        self.check_debug_request(Method::Delete, url, input_headers, None);
        let response = self.client
            .request(Method::Delete,&url,&input_headers)
            .map_err(|e| SimpleHttpError::new_with_cause("Error createing  get: {}",Box::new(e)))?
            .submit()
            .map_err(|e| SimpleHttpError::new_with_cause("Error connecting",Box::new(e)))?;
        Self::read_response(response)
    }

    fn put<'a>(&'a mut self, url: &str, input_headers: &[(&str, &str)], data: &[u8])->Result<Vec<u8>,SimpleHttpError> {
        self.check_debug_request(Method::Put, url, input_headers, Some(data));
        let length_string = format!("{}",data.len());
        let mut headers = input_headers.to_vec();
        headers.push(("Content-Length",&length_string));        
        let collected: Vec<(&str,&str)> = headers.iter().map(|(k,v)|(k.as_ref(),v.as_ref())).collect();
        let mut put_request = self.client
            .put(url,&collected)
            .map_err(|e| SimpleHttpError::new_with_cause("Error posting url",Box::new(e)))?;
        put_request.write_all(&data).map_err(|e| SimpleHttpError::new_with_cause(&format!("Error posting url: {:?}",url),Box::new(e)))?;
        let post_response = put_request.submit()
                .map_err(|e| SimpleHttpError::new_with_cause("Error sending data",Box::new(e)))?;
        Self::read_response(post_response)     
    }

    fn post<'a>(&'a mut self, url: &str, input_headers: &[(&str, &str)], data: &[u8])->Result<Vec<u8>,SimpleHttpError> {
        self.check_debug_request(Method::Post, url, input_headers, Some(data));

        let length_string = format!("{}",data.len());
        let mut headers = input_headers.to_vec();
        headers.push(("Content-Length",&length_string));        
        let collected: Vec<(&str,&str)> = headers.iter().map(|(k,v)|(k.as_ref(),v.as_ref())).collect();
        let mut post_request = self.client
            .post(url,&collected)
            .map_err(|e| SimpleHttpError::new_with_cause("Error posting url",Box::new(e)))?;
        post_request.write_all(&data).map_err(|e| SimpleHttpError::new_with_cause(&format!("Error posting url: {:?}",url),Box::new(e)))?;
        let post_response = post_request.submit()
                .map_err(|e| SimpleHttpError::new_with_cause("Error sending data",Box::new(e)))?;
        Self::read_response(post_response)     
    }

    fn patch<'a>(&'a mut self, url: &str, input_headers: &[(&str, &str)], data: &[u8])->Result<Vec<u8>,SimpleHttpError> {
        if url.contains("localhost") {
            warn!("\n\n!!!! Do you really want to use localhost from esp? I doubt that.")
        }
        if self.debug {
            info!("Calling PATCH {} {:?}",&url, input_headers);
            // todo: write body as well
        }

        let length_string = format!("{}",data.len());
        let mut headers = input_headers.to_vec();
        headers.push(("Content-Length",&length_string));        
        let collected: Vec<(&str,&str)> = headers.iter().map(|(k,v)|(k.as_ref(),v.as_ref())).collect();
        let mut post_request = self.client.request(Method::Patch, url,&collected)
            .map_err(|e| SimpleHttpError::new_with_cause("Error patching url",Box::new(e)))?;
        post_request.write_all(&data).map_err(|e| SimpleHttpError::new_with_cause(&format!("Error posting url: {:?}",url),Box::new(e)))?;
        let post_response = post_request.submit()
                .map_err(|e| SimpleHttpError::new_with_cause("Error sending data",Box::new(e)))?;
        Self::read_response(post_response)     
    }



}