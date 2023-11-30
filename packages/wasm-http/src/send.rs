use http::Request;
use js_sys::{JsString, Object, Number};
use serde_json::Value;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use crate::client::Client;
use crate::error::Error;
use crate::{HttpRequestOptions};
use tower::{ServiceBuilder, ServiceExt};

/// 服务的缓冲区大小, 这个值影响在调用服务时可以排队等待处理的请求数量。如果设置为1，表示同一时刻只能有一个请求在等待处理。如果设置为更大的值，表示可以同时处理多个等待的请求。
#[allow(dead_code)]
const SERVICE_BUFFER: usize = 10;

/// 表示服务的并发限制。这个值限制了同时可以处理的请求数。如果设置为1，表示同一时刻只能处理一个请求。如果设置为更大的值，表示可以同时处理多个请求。
#[allow(dead_code)]
const CONCURRENCY_LIMIT: usize = 10;

/// 限制请求速率的方法。它允许你设置一个请求的速率限制，确保服务不会受到过多请求的影响。具体而言，`rate_limit` 会限制服务在指定的时间窗口内处理的请求数。
#[allow(dead_code)]
const RATE_LIMIT: u64 = 5;

#[wasm_bindgen]
pub struct HttpClient;

#[wasm_bindgen]
impl HttpClient {

    fn get_request_options(opts: JsValue) -> Result<HttpRequestOptions, JsValue> {
        if !opts.is_object() {
            return Err(JsValue::from_str(&Error::Error("`opts` is not a object !".to_string()).to_string()));
        }

        let get_str = |field_value: JsValue| -> String {
            let value = JsString::from(field_value);
            let value = String::from(value);
            value
        };

        if let Some(obj) = opts.dyn_ref::<Object>() {
            let mut options = HttpRequestOptions::default();
            // url
            let url = js_sys::Reflect::get(&obj, &JsValue::from_str("url")).ok();
            if let Some(url) = url {
                options.url = get_str(url);
            }

            // method
            let method = js_sys::Reflect::get(&obj, &JsValue::from_str("method")).ok();
            if let Some(method) = method {
                options.method = Some(get_str(method));
            }

            // data
            let data = js_sys::Reflect::get(&obj, &JsValue::from_str("data")).ok();
            options.data = data;

            // form
            let form = js_sys::Reflect::get(&obj, &JsValue::from_str("form")).ok();
            if let Some(form) = form {
                if !form.is_null() {
                    if let Some(form) = form.dyn_ref::<web_sys::FormData>() {
                        options.form = Some(form.clone());
                    }
                }
            }

            // headers
            let headers = js_sys::Reflect::get(&obj, &JsValue::from_str("headers")).ok();
            if let Some(headers) = headers {
                options.headers = Some(from_value(headers).ok().unwrap())
            }

            // timeout
            let timeout = js_sys::Reflect::get(&obj, &JsValue::from_str("timeout")).ok();
            if let Some(timeout) = timeout {
                if timeout.is_null() {
                    options.timeout = None;
                } else {
                    if timeout.is_string() {
                        let timeout = get_str(timeout).trim().to_string();
                        let timeout = timeout.parse::<i32>().unwrap_or(0);
                        options.timeout = Some(timeout);
                    } else {
                        if let Some(timeout) = timeout.dyn_ref::<Number>() {
                            let timeout = timeout.as_f64().map(|f| f as i32);
                            options.timeout = timeout;
                        }
                    }
                }
            }

            return Ok(options);
        }

        return Err(JsValue::from_str(&Error::Error("`opts` is not a object !".to_string()).to_string()));
    }

    pub async fn send(opts: JsValue) -> Result<JsValue, JsValue> {
        if !opts.is_object() {
            return Err(JsValue::from_str(&Error::Error("`opts` is not a object !".to_string()).to_string()));
        }

        let options = Self::get_request_options(opts)?;
        let client = Client::new(options);
        let service = ServiceBuilder::new()
            // .buffer(SERVICE_BUFFER)
            // .concurrency_limit(CONCURRENCY_LIMIT)
            // .rate_limit(RATE_LIMIT, Duration::from_secs(1))
            .service(client);

        let request = Request::builder().body(Value::Null).map_err(|err| JsValue::from_str(&err.to_string()))?;
        let response = service.oneshot(request).await.map_err(|err| JsValue::from_str(&err.to_string()))?;
        let (_, body) = response.into_parts();
        let result = serde_wasm_bindgen::to_value(&body).map_err(|err| JsValue::from_str(&err.to_string()))?;
        Ok(result)
    }

}


