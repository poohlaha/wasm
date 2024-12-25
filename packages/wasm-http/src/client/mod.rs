mod call;
mod fetch;

use crate::client::call::Call;
use crate::error::Error;
use crate::request::HttpRequest;
use crate::{HttpRequestOptions, HttpResponseOptions};
use http::{Request, Response};
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower_service::Service;

pub struct Client {
    request: Option<HttpRequest>,
    options: HttpRequestOptions,
}

impl Client {
    #[allow(dead_code)]
    pub fn new(options: HttpRequestOptions) -> Self {
        Self { request: None, options }
    }

    #[allow(dead_code)]
    pub fn new_with_request(options: HttpRequestOptions, request: HttpRequest) -> Self {
        Self { request: Some(request), options }
    }

    #[allow(dead_code)]
    pub fn with_request(&mut self, options: HttpRequestOptions, request: HttpRequest) -> &mut Client {
        self.request = Some(request);
        self.options = options;
        self
    }
}

impl Service<Request<Value>> for Client {
    type Response = Response<HttpResponseOptions>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: Request<Value>) -> Self::Future {
        Box::pin(Call::exec(self.request.clone(), self.options.clone()))
    }
}
