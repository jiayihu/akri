#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate k8s_openapi;
extern crate serde_yaml;
extern crate tokio_core;

pub mod akri;
pub mod coap_core;
pub mod k8s;
pub mod onvif;
pub mod os;
