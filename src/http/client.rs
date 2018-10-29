use hyper::client::Client as HyperClient;
use hyper::client::HttpConnector;
use hyper::client::Request;
use hyper::Error as HyperError;
use hyper::Method;
use hyper::Uri;
use hyper::header::ContentLength;
use hyper::header::ContentType;
use hyper::client::Service;
use hyper::client::Response;
use hyper_tls::HttpsConnector;


use std::time::Duration;

use soap_request::SoapRequest;
use search_response::SoapResponse;

use futures::Future;
use futures::Stream;

use tokio_timer::Timer;
use tokio_core::reactor::Handle;
use tokio_timer::TimeoutError;

// use std::convert::TryFrom;

/// for working with errors
use failure::Error;
use failure::err_msg;

use formats_core::base::airport_code::AirportCode;
use std::collections::HashMap;
use std::sync::Arc;
use client::parser;

pub struct Client {
    url: Uri,
    inner: HyperClient<HttpsConnector<HttpConnector>>,
    timeout: Duration,
}

struct ClientError(HyperError);

impl<T> From<TimeoutError<T>> for ClientError {
    fn from(_err: TimeoutError<T>) -> Self {
        ClientError(HyperError::Timeout)
    }
}

impl From<HyperError> for ClientError {
    fn from(err: HyperError) -> Self {
        ClientError(err)
    }
}

impl Into<HyperError> for ClientError {
    fn into(self) -> HyperError {
        self.0
    }
}

struct SoapRequest {
    payload: String
}

impl SoapRequest {
    fn to_xml_string(&self) -> String {
        self.payload
    }
}

struct SoapResponse {
   payload: String
}

impl SoapResponse {
    // TODO: custom response transformer
    fn parse_payload() -> String {
        self.payload
    }
}

impl Client {
    pub fn new(url: Uri, timeout: Duration, handle: &Handle) -> Self {
        Client {
            url,
            timeout,
            // TODO: Custom authentication mechanizm
            inner: HyperClient::configure().connector(HttpsConnector::new(1, handle).expect("create https connector")).build(handle),
        }
    }

    // TODO: Deal with arcs or clones
    pub fn search(&self, soap_request: SoapRequest) -> impl Future<Item = SoapResponse, Error = Error> {
        let body = soap_request.to_xml_string();

        self.post(self.url.clone(), body)
            .and_then(|res| { res.body().concat2() })
            .map_err(|err| err_msg(format!("{:#?}", err)))
            .and_then(move |res| {
                let raw_response = String::from_utf8_lossy(&res).into_owned();
                let parsed_response = SoapResponse::new(&raw_response, soap_request);
                parsed_response.map(|item| (raw_response, item))
            })
    }

    fn post<B: Into<Vec<u8>>>(&self, uri: Uri, body: B) -> impl Future<Item = Response, Error = HyperError> {
        let mut req: Request<::hyper::Body> = Request::new(Method::Post, uri);

        let body: Vec<u8> = body.into();

        {
            let headers = req.headers_mut();
            headers.set(ContentLength(body.len() as u64));
            headers.set(ContentType::xml());
        }

        req.set_body(body);

        let timer = Timer::default();
        let future = self.inner.call(req).map_err(ClientError::from);
        let timeout = self.timeout;

        timer.timeout(future, timeout).map_err(|err| err.into())
    }
}
