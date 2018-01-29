use std::collections::HashMap;
use std::io::Read;

// Use abstract HTTP request with reqwest default adapter.
// Default adapter is reqwest?
extern crate hyper;
use self::hyper::server::Request as HttpRequest;

#[derive(Default, Debug)]
pub struct Request {
    pub header: HashMap<String, String>,
    pub content: String,
}

impl Request {
    pub fn new(header: HashMap<String, String>, content: String) -> Request {
        Request {
            header: header,
            content: content,
        }
    }
}

// So just generic request here with all cipher stuff
// How to send all of the cipher stuff here???
impl<'a, 'b> From<HttpRequest<'a, 'b>> for Request {
    fn from(mut other: HttpRequest<'a, 'b>) -> Request {
        let mut request = Request::default();

        for h in other.headers.iter() {
            request
                .header
                .insert(h.name().to_string(), h.value_string());
        }

        request.content = String::new();
        let _ = other.read_to_string(&mut request.content);

        request
    }
}
