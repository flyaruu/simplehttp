use std::str::from_utf8;
use embedded_svc::http::{client::*, Headers};
use embedded_svc::io::Write;
use esp_idf_svc::http::client::*;

use crate::simplehttp::{SimpleHttpClient, SimpleHttpError};

pub fn new_esp_http()->Box<dyn SimpleHttpClient> {
    Box::new(EspSimpleHttpClient::new().unwrap())
}

pub struct EspSimpleHttpClient {
    client: Client<EspHttpConnection>
}
impl EspSimpleHttpClient {
    pub fn new()->Result<EspSimpleHttpClient,SimpleHttpError> {
        let client = Client::wrap(EspHttpConnection::new(&Configuration {
            crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
            ..Default::default()
        }).map_err(|_| SimpleHttpError::new("Error creating http client"))?);
        Ok(EspSimpleHttpClient{client})
    }

    pub fn read_response(mut response: Response<&mut EspHttpConnection>)->Result<Vec<u8>,SimpleHttpError> {
        let size = response.content_len().ok_or(SimpleHttpError::new("Error reading content"))? as usize;
        let mut body = [0_u8; 3048];
        let mut output_buffer: Vec<u8> = Vec::with_capacity(size);
        loop {
            match response.read(&mut body) {
                Ok(bytes_read) => {
                    // println!("Bytes read: {}",bytes_read);
                    if bytes_read>0 {
                        output_buffer.extend_from_slice(&body[0..bytes_read]);
                    } else {
                        // println!("Result:\n{}",from_utf8(&output_buffer).unwrap());
                        return Ok(output_buffer);
                    }
                },
                Err(_) => return Err(SimpleHttpError::new("Error reading content")),
            };
        }
    }
}

impl SimpleHttpClient for EspSimpleHttpClient {
    fn get(&mut self, url: &str, input_headers: &Vec<(String, String)>)->Result<Vec<u8>, SimpleHttpError> {
        // println!("Getting url: {}",url);
        let mut headers = input_headers.clone();
        headers.push(("Accept".to_owned(), "application/vnd.kafka.binary.v2+json".to_owned()));        
        let collected_headers: Vec<(&str,&str)> = headers.iter().map(|(k,v)|(k.as_ref(),v.as_ref())).collect();
        let response = self.client
            .request(Method::Get,&url,&collected_headers)
            .map_err(|e| SimpleHttpError::new(&format!("Error createing  get: {}",e)))?
            .submit()
            .map_err(|e| SimpleHttpError::new(&format!("Error connecting: {}",e)))?;
        Self::read_response(response)
    }

    fn post<'a>(&'a mut self, url: &str, input_headers: &Vec<(String, String)>, data: Vec<u8>)->Result<Vec<u8>,SimpleHttpError> {
        // println!("Posting url: {}",url);
        if url.contains("localhost") {
            println!("\n\n!!!! Do you really want to use localhost from esp? I doubt that'n'n")
        }
        // let a = from_utf8(&data)
        //     .map_err(|_| SimpleHttpError("Error parsing body".to_owned()))?;
        // println!("Body: {}",a);
        let length_string = format!("{}",data.len());
        let mut headers = input_headers.clone();
        headers.push(("Content-Length".to_owned(),length_string));        
        let collected: Vec<(&str,&str)> = headers.iter().map(|(k,v)|(k.as_ref(),v.as_ref())).collect();
        // println!("Headers: {:?}",collected);
        let mut post_request = self.client
            .post(url,&collected)
            .map_err(|e| SimpleHttpError::new(&format!("Error posting url: {:?}",e)))?;
        // post_request.flush()
        //     .map_err(|_| SimpleHttpError("Error flushing url".to_owned()))?;
        post_request.write_all(&data).map_err(|e| SimpleHttpError::new(&format!("Error posting url: {:?}",e)))?;

        let post_response = post_request.submit()
                .map_err(|_| SimpleHttpError::new("Error sending data"))?;
        Self::read_response(post_response)     
    }
}