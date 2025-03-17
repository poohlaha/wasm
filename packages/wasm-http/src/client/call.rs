use crate::client::fetch::fetch;
use crate::request::HttpRequest;
use crate::{Error, HttpRequestOptions, HttpRequestType, HttpResponseOptions, TIMEOUT};
use http::header::CONTENT_TYPE;
use http::response::Builder;
use http::Response;
use js_sys::{Array, Object, Uint8Array, JSON};
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, BlobPropertyBag, FormData, Headers, RequestCredentials, RequestInit, UrlSearchParams};

pub struct Call;

impl Call {
    pub async fn exec(request: Option<HttpRequest>, options: HttpRequestOptions) -> Result<Response<HttpResponseOptions>, Error> {
        let headers = Self::prepare_headers(&options)?;

        let js_request = Self::prepare_js_request(&options, headers)?;
        let response = fetch(options.timeout, &js_request, request).await?;

        // http response
        let result = Response::builder().status(response.status());

        // response headers
        let response_headers = response.headers().clone();
        let (result, response_headers) = Self::prepare_response_headers(result, response_headers)?;

        // response body
        let status_code = response.status();
        let response_type = options.response_type.clone().unwrap_or(HttpRequestType::Text);

        let method = js_request.method().to_string();
        let mut body = serde_json::Value::Null;

        match response_type {
            HttpRequestType::Blob => {
                let response_body = response.blob().map_err(Error::js_error)?;
                let response_body: JsValue = JsFuture::from(response_body).await.map_err(Error::js_error)?;
                let blob = response_body.dyn_ref::<Blob>();
                if let Some(blob) = blob {
                    let array_buffer = JsFuture::from(blob.array_buffer()).await.map_err(Error::js_error)?;
                    let uint8_array = Uint8Array::new(&array_buffer);
                    let binary_array: Vec<u8> = uint8_array.to_vec();
                    body = serde_json::Value::Object(serde_json::Map::from_iter(vec![("binary".to_string(), serde_json::Value::Array(binary_array.into_iter().map(serde_json::Value::from).collect()))]))
                }
            }
            HttpRequestType::FormData => {
                let response_body = response.form_data().map_err(Error::js_error)?;
                let response_body: JsValue = JsFuture::from(response_body).await.map_err(Error::js_error)?;
                let form_data = response_body.dyn_ref::<FormData>();
                if let Some(form_data) = form_data {
                    let iterator = js_sys::try_iter(form_data).map_err(Error::js_error)?;
                    let mut form_data_map = serde_json::Map::new();
                    if let Some(iterator) = iterator {
                        for item in iterator {
                            let entry = item.map_err(Error::js_error)?;
                            let entry = Array::from(&entry);
                            let key = entry.get(0).as_string().unwrap();
                            let value = entry.get(1).as_string().unwrap();
                            form_data_map.insert(key, serde_json::Value::String(value));
                        }

                        body = serde_json::Value::Object(form_data_map);
                    }
                }
            }
            _ => {
                let response_body = response.text().map_err(Error::js_error)?;
                let response_body: JsValue = JsFuture::from(response_body).await.map_err(Error::js_error)?;
                if let Some(response_body) = response_body.as_string() {
                    if method.to_lowercase() == "post" || method.to_lowercase() == "get" {
                        body = serde_json::from_slice(response_body.as_bytes()).map_err(|_| Error::MissingResponseBody)?;
                    }
                }
            }
        }

        result
            .body(HttpResponseOptions {
                status_code,
                headers: response_headers,
                body,
                error: "".to_string(),
            })
            .map_err(Error::HttpError)
    }

    /// request headers
    fn prepare_headers(options: &HttpRequestOptions) -> Result<Headers, Error> {
        let headers = &options.headers;
        let new_headers = Headers::new().map_err(Error::js_error)?;
        let mut print_headers: HashMap<String, String> = HashMap::new();

        let request_type = &options.request_type;
        if let Some(request_type) = request_type {
            new_headers.append(CONTENT_TYPE.as_str(), request_type.get_content_type().as_str()).map_err(Error::js_error)?;
        }

        if let Some(headers) = headers {
            if !headers.is_null() {
                for (key, value) in headers.as_object().unwrap() {
                    let header_value = value.as_str().unwrap_or("");
                    new_headers.append(key.as_str(), header_value).map_err(Error::js_error)?;
                    print_headers.insert(key.to_string(), header_value.to_string());
                }
            }
        }

        // log(&format!("wasm request headers: {:#?}", print_headers));
        Ok(new_headers)
    }

    /// js request
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)"]
    fn prepare_js_request(options: &HttpRequestOptions, headers: Headers) -> Result<web_sys::Request, Error> {
        let request = RequestInit::new();
        let mut is_method_get: bool = false;
        let method = match &options.method {
            None => "POST",
            Some(method) => {
                if method.to_lowercase() == "get" {
                    is_method_get = true;
                    "GET"
                } else if method.to_lowercase() == "put" {
                    "PUT"
                } else if method.to_lowercase() == "delete" {
                    "DELETE"
                } else {
                    "POST"
                }
            }
        };

        let mut data: JsValue = JsValue::from_str("");

        let request_type = &options.request_type;
        if let Some(request_type) = request_type {
            match request_type {
                // Blob
                HttpRequestType::Blob => {
                    if let Some(value) = options.data.clone() {
                        let uint8_array = Uint8Array::new(&value);
                        let blob = Blob::new_with_u8_array_sequence_and_options(&Array::of1(&uint8_array), &BlobPropertyBag::new()).map_err(Error::js_error)?;
                        data = JsValue::from(blob);
                    }
                }
                // form 表单提交
                HttpRequestType::FormSubmit => {
                    let mut str: String = String::new();
                    if let Some(value) = options.data.clone() {
                        if let Ok(params) = value.dyn_into::<UrlSearchParams>() {
                            let entries = params.entries();
                            let mut pairs = vec![];

                            for entry in entries {
                                let entry = entry.map_err(Error::js_error)?;
                                if let Ok(array) = entry.dyn_into::<Array>() {
                                    let key = array.get(0).as_string().unwrap_or_default();
                                    let value = array.get(1).as_string().unwrap_or_default();
                                    pairs.push(format!("{}={}", key, value));
                                }
                            }

                            str = pairs.join("&");
                        }
                    }

                    data = JsValue::from_str(&str)
                }
                // formData 提交
                HttpRequestType::FormData => {
                    data = JsValue::from(options.data.clone());
                }
                _ => {
                    if let Some(value) = options.data.clone() {
                        if let Some(value) = value.dyn_ref::<Object>() {
                            let value = JSON::stringify(value).map_err(Error::js_error)?;
                            data = JsValue::from(value);
                        }
                    }
                }
            }
        }

        request.set_method(method);
        request.set_headers(headers.as_ref());
        request.set_credentials(RequestCredentials::SameOrigin);
        if !is_method_get {
            request.set_body(&data);
        }

        web_sys::Request::new_with_str_and_init(&options.url.as_str(), &request).map_err(Error::js_error)
    }

    pub(crate) fn prepare_request_timeout(timeout: Option<i32>) -> i32 {
        let mut request_timeout = TIMEOUT;
        // timeout, -1 表示不超时
        if let Some(timeout) = timeout {
            if timeout != -1 {
                if timeout > 0 {
                    request_timeout = timeout;
                }
            }
        }

        request_timeout
    }

    /// response headers
    fn prepare_response_headers(mut result: Builder, headers: Headers) -> Result<(Builder, HashMap<String, String>), Error> {
        let headers_iter = js_sys::try_iter(headers.as_ref()).map_err(Error::js_error)?;
        let mut response_headers: HashMap<String, String> = HashMap::new();
        if let Some(header_iter) = headers_iter {
            for header in header_iter {
                let header = header.map_err(Error::js_error)?;
                let pair: js_sys::Array = header.into();

                let header_name = pair.get(0).as_string();
                let header_value = pair.get(1).as_string();

                match (header_name, header_value) {
                    (Some(header_name), Some(header_value)) => {
                        response_headers.insert(header_name.clone(), header_value.clone());
                        result = result.header(header_name.clone(), header_value.clone());
                    }
                    _ => continue,
                }
            }
        }

        Ok((result, response_headers))
    }
}
