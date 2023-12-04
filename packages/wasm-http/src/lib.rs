mod client;
mod error;
mod request;
mod send;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tower::{ServiceExt};
use wasm_bindgen::prelude::*;
use crate::error::Error;
use crate::send::HttpClient;

#[derive(Default, Debug, Clone)]
pub struct HttpRequestOptions {
    pub url: String,            // url
    pub method: Option<String>, // method: post、get
    pub data: Option<JsValue>,    // data
    pub form: Option<web_sys::FormData>,     // form
    pub headers: Option<Value>, // headers
    pub is_form_submit: Option<bool>, // 是否 form 表单提交
    pub timeout: Option<i32>,   // timeout
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
