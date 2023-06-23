#[derive(Debug)]
pub struct SimpleHttpError(pub String);

pub trait SimpleHttpClient {
    // fn post(&mut self,  url: &str, headers: &mut Vec<(&str, &str)>, data: Vec<u8>)->Result<Vec<u8>,RedPandaError>;
    fn post(&mut self, url: &str, headers: &Vec<(String, String)>, data: Vec<u8>)->Result<Vec<u8>,SimpleHttpError>;

    fn get(&mut self, url: &str, headers: &Vec<(String, String)>)->Result<Vec<u8>, SimpleHttpError>;
}
