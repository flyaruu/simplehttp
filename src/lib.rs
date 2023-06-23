pub mod simplehttp;

#[cfg(feature = "reqwest")]
pub mod simplehttp_reqwest;

#[cfg(feature = "esp32")]
pub mod simplehttp_esp32;

#[cfg(feature = "spin")]
pub mod simplehttp_spin;

