use crate::client::Client;
use crate::error::Error;
use crate::request::cache::Cache;
use crate::request::credentials::Credentials;
use crate::request::mode::Mode;
use crate::request::redirect::Redirect;
use crate::request::referrer_policy::ReferrerPolicy;
use crate::request::HttpRequest;
use crate::{log, HttpRequestOptions};
use http::Request;
use js_sys::{JsString, Number, Object};
use serde_json::Value;
use serde_wasm_bindgen::from_value;
use tower::{ServiceBuilder, ServiceExt};
use wasm_bindgen::prelude::*;

/// 服务的缓冲区大小, 这个值影响在调用服务时可以排队等待处理的请求数量。如果设置为1，表示同一时刻只能有一个请求在等待处理。如果设置为更大的值，表示可以同时处理多个等待的请求。
#[allow(dead_code)]
const SERVICE_BUFFER: usize = 10;

/// 表示服务的并发限制。这个值限制了同时可以处理的请求数。如果设置为1，表示同一时刻只能处理一个请求。如果设置为更大的值，表示可以同时处理多个请求。
#[allow(dead_code)]
const CONCURRENCY_LIMIT: usize = 10;

/// 限制请求速率的方法。它允许你设置一个请求的速率限制，确保服务不会受到过多请求的影响。具体而言，`rate_limit` 会限制服务在指定的时间窗口内处理的请求数。
#[allow(dead_code)]
const RATE_LIMIT: u64 = 5;

pub struct HttpClient;

impl HttpClient {
    fn get_str(field_value: JsValue) -> String {
        let value = JsString::from(field_value);
        let value = String::from(value);
        value
    }

    /// 获取 `http` `options`
    fn get_http_options(opts: JsValue) -> Result<HttpRequestOptions, JsValue> {
        if !opts.is_object() {
            return Err(JsValue::from_str(&Error::Error("`opts` is not a object !".to_string()).to_string()));
        }

        if let Some(obj) = opts.dyn_ref::<Object>() {
            if obj.is_null() {
                return Err(JsValue::from_str(&Error::Error("`opts` is null !".to_string()).to_string()));
            }

            let mut options = HttpRequestOptions::default();

            // url
            let url = js_sys::Reflect::get(&obj, &JsValue::from_str("url")).ok();
            if let Some(url) = url {
                options.url = Self::get_str(url);
            }

            if options.url.is_empty() {
                return Err(JsValue::from_str(&Error::Error("`url` is empty !".to_string()).to_string()));
            }

            // method
            let method = js_sys::Reflect::get(&obj, &JsValue::from_str("method")).ok();
            if let Some(method) = method {
                if !method.is_null() {
                    options.method = Some(Self::get_str(method));
                }
            }

            // data
            let data = js_sys::Reflect::get(&obj, &JsValue::from_str("data")).ok();
            options.data = data;

            // headers
            let headers = js_sys::Reflect::get(&obj, &JsValue::from_str("headers")).ok();
            if let Some(headers) = headers {
                if !headers.is_object() {
                    return Err(JsValue::from_str(&Error::Error("`headers` is not a object !".to_string()).to_string()));
                }

                options.headers = Some(from_value(headers).ok().unwrap())
            }

            // timeout
            let timeout = js_sys::Reflect::get(&obj, &JsValue::from_str("timeout")).ok();
            if let Some(timeout) = timeout {
                if timeout.is_null() {
                    options.timeout = None;
                } else {
                    if timeout.is_string() {
                        let timeout = Self::get_str(timeout).trim().to_string();
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

            // type
            let _type = js_sys::Reflect::get(&obj, &JsValue::from_str("type")).ok();
            if let Some(_type) = _type {
                let request_type: String = Self::get_str(_type);
                let request_type = request_type.parse::<i32>().unwrap_or(0);
                options.request_type = Some(crate::HttpRequestType::get_type(request_type));
            }

            // responseType
            let response_type = js_sys::Reflect::get(&obj, &JsValue::from_str("responseType")).ok();
            if let Some(response_type) = response_type {
                let response_type: String = Self::get_str(response_type);
                let response_type = response_type.parse::<i32>().unwrap_or(0);
                options.response_type = Some(crate::HttpRequestType::get_type(response_type));
            }

            return Ok(options);
        }

        Err(JsValue::from_str(&Error::Error("`opts` is not a object !".to_string()).to_string()))
    }

    /// 获取 `request` `options`
    fn get_request_options(request: JsValue) -> Result<HttpRequest, JsValue> {
        let mut http_request = HttpRequest::default();
        if request.is_null() {
            return Ok(http_request);
        }

        if !request.is_object() {
            return Err(JsValue::from_str(&Error::Error("`request` is not a object !".to_string()).to_string()));
        }

        if let Some(obj) = request.dyn_ref::<Object>() {
            if obj.is_null() {
                return Ok(http_request);
            }

            // cache
            let cache = js_sys::Reflect::get(&obj, &JsValue::from_str("cache")).ok();
            if let Some(cache) = cache {
                let cache = Self::get_str(cache);
                http_request.cache = Some(Cache::get_cache(cache));
            }

            // credentials
            let credentials = js_sys::Reflect::get(&obj, &JsValue::from_str("credentials")).ok();
            if let Some(credentials) = credentials {
                let credentials = Self::get_str(credentials);
                http_request.credentials = Some(Credentials::get_credentials(credentials));
            }

            // integrity
            let integrity = js_sys::Reflect::get(&obj, &JsValue::from_str("integrity")).ok();
            if let Some(integrity) = integrity {
                let integrity = Self::get_str(integrity);
                let integrity = integrity.trim();
                if !integrity.is_empty() {
                    http_request.integrity = Some(integrity.to_string());
                }
            }

            // mode
            let mode = js_sys::Reflect::get(&obj, &JsValue::from_str("mode")).ok();
            if let Some(mode) = mode {
                let mode = Self::get_str(mode);
                if !mode.is_empty() {
                    http_request.mode = Some(Mode::get_mode(mode));
                }
            }

            // redirect
            let redirect = js_sys::Reflect::get(&obj, &JsValue::from_str("redirect")).ok();
            if let Some(redirect) = redirect {
                let redirect = Self::get_str(redirect);
                if !redirect.is_empty() {
                    http_request.redirect = Some(Redirect::get_redirect(redirect));
                }
            }

            // referrer
            let referrer = js_sys::Reflect::get(&obj, &JsValue::from_str("referrer")).ok();
            if let Some(referrer) = referrer {
                let referrer = Self::get_str(referrer);
                let referrer = referrer.trim();
                if !referrer.is_empty() {
                    http_request.referrer = Some(referrer.to_string());
                }
            }

            // referrer policy
            let referrer_policy = js_sys::Reflect::get(&obj, &JsValue::from_str("referrerPolicy")).ok();
            if let Some(referrer_policy) = referrer_policy {
                let referrer_policy = Self::get_str(referrer_policy);
                if !referrer_policy.is_empty() {
                    http_request.referrer_policy = Some(ReferrerPolicy::get_referrer_policy(referrer_policy));
                }
            }
        }

        Ok(http_request)
    }

    pub async fn send(opts: JsValue, request: JsValue) -> Result<JsValue, JsValue> {
        if opts.is_null() {
            return Err(JsValue::from_str(&Error::Error("`opts` is null !".to_string()).to_string()));
        }

        if !opts.is_object() {
            return Err(JsValue::from_str(&Error::Error("`opts` is not a object !".to_string()).to_string()));
        }

        let options = Self::get_http_options(opts)?;
        let request = Self::get_request_options(request)?;

        // client
        let client = Client::new_with_request(options, request);

        // service
        let service = ServiceBuilder::new()
            // .buffer(SERVICE_BUFFER)
            // .concurrency_limit(CONCURRENCY_LIMIT)
            // .rate_limit(RATE_LIMIT, Duration::from_secs(1))
            .service(client);

        let request = Request::builder().body(Value::Null).map_err(|err| JsValue::from_str(&err.to_string()))?;
        let response = service.oneshot(request).await.map_err(|err| JsValue::from_str(&err.to_string()))?;
        let (_, mut http_response) = response.into_parts();
        let body = http_response.body.clone();
        // 查看 body 中有没有大数字

        http_response.body = Self::convert_numbers(body);
        let result = serde_wasm_bindgen::to_value(&http_response).map_err(|err| JsValue::from_str(&err.to_string()))?;
        Ok(result)
    }

    fn convert_numbers(value: Value) -> Value {
        match value {
            Value::Array(vec) => {
                let vec = vec.iter().map(|v| Self::convert_numbers(v.clone())).collect();
                Value::Array(vec)
            }
            Value::Object(map) => {
                let obj = map.iter().map(|(key, v)| (key.clone(), Self::convert_numbers(v.clone()))).collect();
                Value::Object(obj)
            }
            Value::Number(num) => {
                if let Some(val) = num.as_f64() {
                    // 超出范围
                    if val.is_infinite() || val.is_nan() || val.abs() > Number::MAX_SAFE_INTEGER {
                        Value::String(num.to_string())
                    } else {
                        Value::Number(num)
                    }
                } else {
                    Value::Number(num)
                }
            }

            _ => value.clone(),
        }
    }
}
