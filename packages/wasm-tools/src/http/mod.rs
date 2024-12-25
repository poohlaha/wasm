use reqwest::header::{HeaderMap, HeaderName};
use reqwest::{multipart, Client, Method, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;
use wasm_bindgen::JsValue;

use crate::error::WasmError;
use crate::log;

pub type HttpFormData = multipart::Form;

#[derive(Default, Debug)]
pub struct Options {
    pub url: String,                // url
    pub method: Option<String>,     // method: post、get
    pub data: Option<Value>,        // data
    pub form: Option<HttpFormData>, // form
    pub headers: Option<Value>,     // headers
                                    // pub timeout: Option<u64>,   // timeout
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Value,
    pub error: String,
}

pub struct HttpClient;

const DEFAULT_TIMEOUT: u64 = 30;

impl HttpClient {
    /// 获取超时时间
    #[allow(dead_code)]
    fn get_timeout(timeout: Option<u64>) -> u64 {
        let mut send_timeout = DEFAULT_TIMEOUT;
        if !timeout.is_none() {
            send_timeout = timeout.unwrap();
        }

        return send_timeout;
    }

    /// get headers
    fn get_headers(headers: Option<Value>, is_form_submit: bool, is_file_submit: bool) -> HeaderMap {
        let mut new_headers: Vec<(String, String)> = Vec::new();

        let mut has_content_type: bool = false;
        if let Some(header) = headers {
            // 判断 value 不为 null
            if !header.is_null() {
                for (key, value) in header.as_object().unwrap() {
                    if key.to_lowercase() == "content-type" {
                        has_content_type = true;
                        // 文件上传不需要 `content-type`
                        if is_file_submit {
                            continue;
                        }
                    }

                    let header_value = value.as_str().unwrap_or("");
                    new_headers.push((key.clone(), String::from(header_value)));
                }
            }
        }

        if !has_content_type {
            if is_form_submit {
                new_headers.push((String::from("content-type"), String::from("application/x-www-form-urlencoded")));
            } else if is_file_submit {
                // if need will be error
                // new_headers.push((String::from("content-type"), String::from("multipart/form-data")));
            } else {
                new_headers.push((String::from("content-type"), String::from("application/json")));
            }
        }

        let mut request_headers = HeaderMap::new();
        for (name, value) in new_headers.iter() {
            request_headers.insert(&HeaderName::from_bytes(name.as_bytes()).unwrap(), value.as_str().parse().unwrap());
        }

        let header_msg = format!("send headers: {:#?}", request_headers);
        log(&header_msg);
        return request_headers;
    }

    fn get_error_response<T: Debug + ToString>(code: u16, error: &T) -> HttpResponse {
        return HttpResponse {
            status_code: code,
            headers: HashMap::new(),
            body: Value::default(),
            error: format!("send request error: {:?}", error),
        };
    }

    /// get http response
    fn get_response(status: StatusCode, response_headers: HeaderMap, body: String) -> HttpResponse {
        let status_code = status.as_u16();
        if status.is_success() {
            let headers: HashMap<String, String> = response_headers.iter().map(|(name, value)| (name.to_string(), value.to_str().unwrap_or("").to_string())).collect();
            return HttpResponse {
                status_code: 200,
                headers,
                body: serde_json::from_slice(body.as_bytes()).unwrap(),
                error: String::new(),
            };
        } else {
            return Self::get_error_response(status_code, &status_code);
        }
    }

    fn get_method(method: Option<String>) -> (Method, bool) {
        let mut is_method_get: bool = false;
        let method = match method {
            None => Method::POST,
            Some(method) => {
                if method.to_lowercase() == "get" {
                    is_method_get = true;
                    Method::GET
                } else {
                    Method::POST
                }
            }
        };

        (method, is_method_get)
    }

    /// 发送普通请求
    pub async fn client_send(opts: Options, is_form_submit: Option<bool>, is_form_data: bool) -> Result<JsValue, JsValue> {
        if opts.url.is_empty() {
            return Err(JsValue::from_str(&WasmError::Empty("url".to_string()).to_string()));
        }

        let mut form_submit = false;
        if let Some(is_form_submit) = is_form_submit {
            form_submit = is_form_submit
        }

        // method
        let (method, is_method_get) = HttpClient::get_method(opts.method);

        // headers
        let headers = Self::get_headers(opts.headers, form_submit, is_form_data);

        let client_builder = Client::builder()
            // .danger_accept_invalid_certs(true)
            .build()
            .map_err(|err| JsValue::from_str(&WasmError::Error(err.to_string()).to_string()))?;
        // request
        let mut request = client_builder.request(method, opts.url);
        // let request = request.timeout(Duration::from_secs(HttpClient::get_timeout(options.timeout)));

        // body
        if !is_method_get {
            if is_form_data {
                // FormData
                if let Some(form) = opts.form {
                    request = request.multipart(form);
                }
            } else {
                if let Some(data) = opts.data {
                    if !data.is_null() {
                        if form_submit {
                            request = request.form(data.as_object().unwrap());
                        } else {
                            request = request.body(data.to_string());
                        }
                    }
                }
            }
        }

        // response
        let response = request.headers(headers).send().await.map_err(|err| JsValue::from_str(&WasmError::Error(err.to_string()).to_string()))?;
        let status = response.status();

        // response headers
        let response_headers = response.headers().clone();

        // response body
        let body = response.text().await.map_err(|err| JsValue::from_str(&WasmError::Error(err.to_string()).to_string()))?;
        let result = HttpClient::get_response(status, response_headers, body);
        let result = serde_wasm_bindgen::to_value(&result).map_err(|err| JsValue::from_str(&WasmError::Error(err.to_string()).to_string()))?;
        Ok(result)
    }

    pub async fn send(opts: Options, is_form_submit: Option<bool>) -> Result<JsValue, JsValue> {
        HttpClient::client_send(opts, is_form_submit, false).await
    }

    /// 发送 `FormData` 请求
    pub async fn send_form_data(opts: Options) -> Result<JsValue, JsValue> {
        HttpClient::client_send(opts, None, true).await
    }
}
