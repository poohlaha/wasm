mod error;
pub mod http;

use crate::error::WasmError;
use crate::http::{HttpClient, Options};
use js_sys::{Array, JsString, Object};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn get_options(opts: JsValue, is_form_data: bool) -> Result<Options, JsValue> {
    if !opts.is_object() {
        return Err(JsValue::from_str(&WasmError::Error("`opts` is not a object !".to_string()).to_string()));
    }

    let get_str = |field_value: JsValue| -> String {
        let value = JsString::from(field_value);
        let value = String::from(value);
        value
    };

    if let Some(obj) = opts.dyn_ref::<Object>() {
        let mut options = Options::default();
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
        if let Some(data) = data {
            options.data = Some(from_value(data).ok().unwrap())
        }

        // form
        if is_form_data {
            let form = js_sys::Reflect::get(&obj, &JsValue::from_str("form")).ok();
            if let Some(form) = form {
                if !form.is_null() {
                    if let Some(obj) = form.dyn_ref::<web_sys::FormData>() {
                        let mut form = reqwest::multipart::Form::new();
                        let iterator = js_sys::try_iter(obj)?.ok_or_else(|| "get params error !")?;
                        for item in iterator {
                            let entry = item?;
                            let entry = Array::from(&entry);
                            let key = entry.get(0).as_string().unwrap();
                            let value = entry.get(1).as_string().unwrap();

                            form = form.text(key, value);
                        }

                        options.form = Some(form);
                    }
                }
            }
        }

        // headers
        let headers = js_sys::Reflect::get(&obj, &JsValue::from_str("headers")).ok();
        if let Some(headers) = headers {
            options.headers = Some(from_value(headers).ok().unwrap())
        }

        return Ok(options);
    }

    return Err(JsValue::from_str(&WasmError::Error("`opts` is not a object !".to_string()).to_string()));
}

/**
  发送普通请求, 包括 `form` 表单提交
*/
#[wasm_bindgen]
pub async fn send(opts: JsValue, is_form_submit: Option<bool>) -> Result<JsValue, JsValue> {
    if !opts.is_object() {
        return Err(JsValue::from_str(&WasmError::Error("`opts` is not a object !".to_string()).to_string()));
    }

    let options = get_options(opts, false)?;
    return HttpClient::send(options, is_form_submit).await;
}

/**
 发送 `FormData` 请求, 包括文件上传
*/
#[wasm_bindgen]
pub async fn send_form_data(opts: JsValue) -> Result<JsValue, JsValue> {
    if !opts.is_object() {
        return Err(JsValue::from_str(&WasmError::Error("`opts` is not a object !".to_string()).to_string()));
    }

    let options = get_options(opts, true)?;
    return HttpClient::send_form_data(options).await;
}
