extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod client;
mod error;
mod proxy;
mod toxic;

pub use client::Client;
pub use error::Error;
pub use proxy::Proxy;
pub use toxic::{Toxic, ToxicType, Stream};

#[cfg(test)]
mod client_test;
