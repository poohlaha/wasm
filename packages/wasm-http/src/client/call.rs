use std::collections::HashMap;
use http::header::CONTENT_TYPE;
use crate::request::{HttpRequest};
use crate::{Error, HttpRequestOptions, HttpResponseOptions, log, TIMEOUT};
use http::{Response};
use http::response::Builder;
use js_sys::{Object, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{AbortController, AbortSignal, Blob, BlobPropertyBag, Headers, RequestCredentials, RequestInit};
use crate::client::fetch::fetch;

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
        let response_body = response.text().map_err(Error::js_error)?;
        let response_body = JsFuture::from(response_body).await.map_err(Error::js_error)?;
        let response_body = response_body.as_string().unwrap_or(String::new());
        let response_body = serde_json::from_slice(response_body.as_bytes()).map_err(|_| Error::MissingResponseBody)?;

        result.body(HttpResponseOptions {
            status_code: response.status(),
            headers: response_headers,
            body: response_body,
            error: "".to_string(),
        }).map_err(Error::HttpError)
    }

    /// request headers
    fn prepare_headers(options: &HttpRequestOptions) -> Result<Headers, Error> {
        let headers = &options.headers;
        let new_headers = Headers::new().map_err(Error::js_error)?;
        let mut print_headers: HashMap<String, String> = HashMap::new();
        let is_form_submit = if options.is_form_submit.is_some() { true } else { false };
        let is_file_submit = if options.form.is_some() { true } else { false };

        // 文件上传下载
        if is_file_submit {

        } else if is_form_submit { // form 表单提交
            new_headers.append(CONTENT_TYPE.as_str(), "application/x-www-form-urlencoded").map_err(Error::js_error)?;
            print_headers.insert(CONTENT_TYPE.to_string(), "application/x-www-form-urlencoded".to_string());
        } else {
            new_headers.append(CONTENT_TYPE.as_str(), "application/json").map_err(Error::js_error)?;
            print_headers.insert(CONTENT_TYPE.to_string(), "application/json".to_string());
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

        log(&format!("request headers: {:#?}", print_headers));

        Ok(new_headers)
    }

    /// js request
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)"]
    fn prepare_js_request(options: &HttpRequestOptions, headers: Headers) -> Result<web_sys::Request, Error> {
        let mut request = RequestInit::new();
        let mut is_method_get: bool = false;
        let method = match &options.method {
            None => "POST",
            Some(method) => {
                if method.to_lowercase() == "get" {
                    is_method_get = true;
                    "GET"
                } else {
                    "POST"
                }
            }
        };

        let mut data: JsValue = JsValue::from_str("");
        if options.form.is_some() { // FormData 提交
            if let Some(form) = options.form.clone() {
                data = JsValue::from(form);
            }
        } else if options.is_form_submit.is_some() { // form 表单提交
            if let Some(is_form_submit) = options.is_form_submit {
                if is_form_submit {
                    if let Some(value) = options.data.clone() {
                        let uint8_array = Uint8Array::new(&value);
                        let blob = Blob::new_with_u8_array_sequence_and_options(
                            &js_sys::Array::of1(&uint8_array),
                            &BlobPropertyBag::new()
                        ).map_err(Error::js_error)?;
                        data = JsValue::from(blob);
                    }
                }
            }
        }

        let mut request = request.method(method).headers(headers.as_ref()).credentials(RequestCredentials::SameOrigin);
        if !is_method_get {
            request = request.body(Some(&data));
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
    fn prepare_response_headers(mut result: Builder, headers: Headers) -> Result<(Builder, HashMap<String, String>), Error>{
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


