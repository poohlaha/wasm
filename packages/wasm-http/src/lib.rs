mod client;
mod error;
mod request;
mod send;

use crate::error::Error;
use crate::send::HttpClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub enum HttpRequestType {
    Json = 0,
    FormSubmit,
    FormData,
    Blob,
    Text,
    Html,
}

impl HttpRequestType {
    fn get_content_type(&self) -> String {
        match self {
            HttpRequestType::Json => String::from("application/json;charset=UTF-8"),
            HttpRequestType::FormSubmit => String::from("application/x-www-form-urlencoded"),
            HttpRequestType::FormData => String::from("multipart/form-data"),
            HttpRequestType::Blob => String::from("application/octet-stream"),
            HttpRequestType::Text => String::from("text/plain;charset=UTF-8"),
            HttpRequestType::Html => String::from("text/html;charset=UTF-8"),
        }
    }

    fn get_type(data: i32) -> HttpRequestType {
        match data {
            1 => HttpRequestType::FormSubmit,
            2 => HttpRequestType::FormData,
            3 => HttpRequestType::Blob,
            4 => HttpRequestType::Text,
            5 => HttpRequestType::Html,
            _ => HttpRequestType::Json,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct HttpRequestOptions {
    pub url: String,                            // url
    pub method: Option<String>,                 // method: post、get
    pub data: Option<JsValue>,                  // data
    pub headers: Option<Value>,                 // headers
    pub timeout: Option<i32>,                   // timeout
    pub request_type: Option<HttpRequestType>,  // request type
    pub response_type: Option<HttpRequestType>, // response type
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponseOptions {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Value,
    pub error: String,
}

const TIMEOUT: i32 = 30;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn send(opts: JsValue, request: JsValue) -> Result<JsValue, JsValue> {
    HttpClient::send(opts, request).await
}
