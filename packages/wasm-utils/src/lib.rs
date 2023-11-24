pub mod date;
mod error;
pub mod signature;
mod storage;
pub mod utils;

use crate::error::WasmError;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlDocument, Window};

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/**
  浏览器是否支持 `wasm`
*/
#[wasm_bindgen]
pub fn is_support_wasm() -> Result<bool, JsValue> {
    let window = web_sys::window();
    if window.is_none() {
        return Ok(false);
    }

    let name = "WebAssembly";
    let window = window.unwrap();
    if !window.has_own_property(&JsValue::from_str(name)) {
        return Ok(false);
    }

    let web_assembly = window.get(name);
    if web_assembly.is_none() {
        return Ok(false);
    }

    let web_assembly = web_assembly.unwrap();
    if !web_assembly.is_object() {
        return Ok(false);
    }

    let instance_name = JsValue::from_str("instantiate");
    if !web_assembly.has_own_property(&instance_name) {
        return Ok(false);
    }

    let web_assembly_instantiate = js_sys::Reflect::get(&web_assembly, &instance_name).map_err(|err| err)?;

    if web_assembly_instantiate.is_function() {
        log("WebAssembly is supported");
        return Ok(true);
    }

    Ok(false)
}

/// 获取浏览器 `Window` 对象
fn get_window() -> Result<Window, JsValue> {
    let window = web_sys::window();
    if window.is_none() {
        return Err(JsValue::from_str(&WasmError::Error("get `window` error !".to_string()).to_string()));
    }

    let window = window.ok_or_else(|| JsValue::from_str(&WasmError::Error("get `window` error !".to_string()).to_string()))?;

    Ok(window)
}

/// 获取浏览器 `Document` 对象
fn get_document() -> Result<Document, JsValue> {
    let window = get_window()?;
    let document = window.document();
    if document.is_none() {
        return Err(JsValue::from_str(&WasmError::Error("get `document` error !".to_string()).to_string()));
    }

    let document = document.ok_or_else(|| JsValue::from_str(&WasmError::Error("get `document` error !".to_string()).to_string()))?;

    Ok(document)
}

/// 获取浏览器 `HtmlDocument` 对象
fn get_html_document() -> Result<HtmlDocument, JsValue> {
    let document = get_document()?;
    let html_document = document.dyn_into::<HtmlDocument>()?;
    Ok(html_document)
}
