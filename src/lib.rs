#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate hyper;
extern crate xml;
extern crate encoding;

#[macro_use]
extern crate error_chain;

mod wsdl;

pub use wsdl::schema::{
    Documented,
    NamedItem,
    Wsdl,
    WsdlBinding,
    WsdlOperationBinding,
    WsdlInputBinding,
    WsdlOutputBinding,
    WsdlFaultBinding,
    WsdlPort,
    WsdlService
};


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
