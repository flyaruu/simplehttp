use std::{error::Error, fmt::{Display, Debug}};

#[derive(Debug)]
pub struct SimpleHttpError {
    message: String,
    parent: Option<Box<dyn Error>>,
}

impl SimpleHttpError {
    pub fn new(message: &str)->Self {
        SimpleHttpError { message: message.to_owned(), parent: None }
    }
}
pub trait SimpleHttpClient {
    // fn post(&mut self,  url: &str, headers: &mut Vec<(&str, &str)>, data: Vec<u8>)->Result<Vec<u8>,RedPandaError>;
    fn post(&mut self, url: &str, headers: &Vec<(String, String)>, data: Vec<u8>)->Result<Vec<u8>,SimpleHttpError>;

    fn get(&mut self, url: &str, headers: &Vec<(String, String)>)->Result<Vec<u8>, SimpleHttpError>;
}

impl Display for SimpleHttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Http Error {}",self.message))?;
        match &self.parent {
            Some(parent) => {
                std::fmt::Display::fmt(&parent, f)?;

            }
            None => {
                ();
            }
        }
        Ok(())
    }
}
impl Error for SimpleHttpError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // self.parent.map(|f|(f.as_ref()))
        self.parent.as_deref()
    }



    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

    // fn provide<'a>(&'a self, demand: &mut std::any::Demand<'a>) {}
}