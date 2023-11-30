mod fetch;
mod call;

use crate::error::Error;
use crate::request::{HttpRequest};
use crate::{HttpRequestOptions, HttpResponseOptions};
use http::{Request, Response};
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower_service::Service;
use crate::client::call::Call;

pub struct Client {
    request: Option<HttpRequest>,
    options: HttpRequestOptions,
}

impl Client {
    pub fn new(options: HttpRequestOptions) -> Self {
        Self { request: None, options }
    }

    pub fn new_with_options(request: HttpRequest, options: HttpRequestOptions) -> Self {
        Self {
            request: Some(request),
            options,
        }
    }

    pub fn with_options(&mut self, request: HttpRequest, options: HttpRequestOptions) -> &mut Client {
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
