use embedded_svc::{http::{client::*, Headers}, io::Write};
use esp_idf_svc::http::client::*;
use log::{warn, info};
use crate::simplehttp::{SimpleHttpClient, SimpleHttpError, HttpResponse};

pub fn new_esp_http()->Box<dyn SimpleHttpClient> {
    Box::new(EspSimpleHttpClient::new().unwrap())
}

pub fn new_esp_http_debug()->Box<dyn SimpleHttpClient> {
    Box::new(EspSimpleHttpClient::new_debug().unwrap())
}

// Supported headers are hard coded in the embedded_svc crate
const SUPPORTED_HEADERS: &[&'static str] = &["Content-Type","Content-Length","Content-Encoding","Transfer-Encoding","Host","Connection","Cache-Control","Location","Upgrade"];

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

    fn read_response(mut response: Response<&mut EspHttpConnection>)->Result<HttpResponse,SimpleHttpError> {
        let size = response.content_len()
            .ok_or(SimpleHttpError::new("Error reading content length"))? as usize;
        let status = response.status();
        // let released = response.release();
        let response_headers: Vec<(String,String)> =  SUPPORTED_HEADERS.iter()
            .filter_map(|k| response.header(k).map(|e| (*k,e.to_owned())))
            .map(|(k,v)|(k.to_owned(),v))
            .collect();

        // released.header(name)
        let mut body = [0_u8; 3048];
        let mut output_buffer: Vec<u8> = Vec::with_capacity(size);
        loop {
            match response.read(&mut body) {
                Ok(bytes_read) => {
                    if bytes_read>0 {
                        output_buffer.extend_from_slice(&body[0..bytes_read]);
                    } else {
                        break;
                    }
                },
                Err(e) => return Err(SimpleHttpError::new_with_cause("Error reading content",Box::new(e))),
            };
        };
        Ok(HttpResponse {
            status_code: status,
            response_headers: response_headers,
            body: output_buffer,
        })
    }

    fn check_debug_request(&self, method: &crate::simplehttp::Method, url: &str, input_headers: &[(&str,&str)], _body: Option<&[u8]>) {
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

impl Into<embedded_svc::http::Method> for crate::simplehttp::Method {
    fn into(self) -> embedded_svc::http::Method {
        match self {
            crate::simplehttp::Method::Options => embedded_svc::http::Method::Options,
            crate::simplehttp::Method::Get => embedded_svc::http::Method::Get,
            crate::simplehttp::Method::Post => embedded_svc::http::Method::Post,
            crate::simplehttp::Method::Put => embedded_svc::http::Method::Put,
            crate::simplehttp::Method::Delete => embedded_svc::http::Method::Delete,
            crate::simplehttp::Method::Head => embedded_svc::http::Method::Head,
            crate::simplehttp::Method::Trace => embedded_svc::http::Method::Trace,
            crate::simplehttp::Method::Connect => embedded_svc::http::Method::Connect,
            crate::simplehttp::Method::Patch => embedded_svc::http::Method::Patch,
        }
    }
}
impl SimpleHttpClient for EspSimpleHttpClient {

    fn custom(&mut self, method: crate::simplehttp::Method, url: &str, input_headers: &[(&str, &str)], body: Option<&[u8]>)->Result<HttpResponse,SimpleHttpError> {

        self.check_debug_request(&method, url, input_headers, None);
        // let length_string = format!("{}",body.len());

        let mut request = self.client
            .request(method.into(),&url,&input_headers)
            .map_err(|e| SimpleHttpError::new_with_cause("Error creating  get: {}",Box::new(e)))?;

        if let Some(b) = body {
            request.write_all(&b).map_err(|e| SimpleHttpError::new_with_cause(&format!("Error getting(with body) url: {:?}",url),Box::new(e)))?;
        }
        let response = request
            .submit()
            .map_err(|e| SimpleHttpError::new_with_cause("Error connecting",Box::new(e)))?;


        Self::read_response(response)


        // get_request.write_all(&data).map_err(|e| SimpleHttpError::new_with_cause(&format!("Error getting(with body) url: {:?}",url),Box::new(e)))?;
        // let get_request = get_request.submit()
        //         .map_err(|e| SimpleHttpError::new_with_cause("Error sending data",Box::new(e)))?;
        
        // todo!()
        // let request = self.prepare_request(url, input_headers, body, method.into())?;
        // let response = self.client.execute(request)
        //     .map_err(|e| SimpleHttpError::new_with_cause("Error sending custom",Box::new(e)))?;
        
        // let response_status = response.status();
        // let response_headers: Vec<(String,String)> = response.headers().iter().filter_map(|(header_name,header_value)| header_value.to_str().ok().map(|v| (header_name.as_str().to_owned(),v.to_owned())) ).collect();
        // let response_body = response.bytes()
        //     .map_err(|e| SimpleHttpError::new_with_cause("Error decoding custom response",Box::new(e)))?
        //     .to_vec();
        // let response = HttpResponse {
        //     status_code: response_status.as_u16(),
        //     response_headers: response_headers,
        //     body: response_body,
        // };
        // if !response_status.is_success() {
        //     return Err(SimpleHttpError::ResponseError(response));
        // }
        // Ok(response)
    }

    // fn post<'a>(&'a mut self, url: &str, input_headers: &[(&str, &str)], data: &[u8])->Result<Vec<u8>,SimpleHttpError> {
    //     self.check_debug_request(Method::Post, url, input_headers, Some(data));

    //     let length_string = format!("{}",data.len());
    //     let mut headers = input_headers.to_vec();
    //     headers.push(("Content-Length",&length_string));        
    //     let collected: Vec<(&str,&str)> = headers.iter().map(|(k,v)|(k.as_ref(),v.as_ref())).collect();
    //     let mut post_request = self.client
    //         .post(url,&collected)
    //         .map_err(|e| SimpleHttpError::new_with_cause("Error posting url",Box::new(e)))?;
    //     post_request.write_all(&data).map_err(|e| SimpleHttpError::new_with_cause(&format!("Error posting url: {:?}",url),Box::new(e)))?;
    //     let post_response = post_request.submit()
    //             .map_err(|e| SimpleHttpError::new_with_cause("Error sending data",Box::new(e)))?;
    //     Self::read_response(post_response)     
    // }
}