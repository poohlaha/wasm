pub mod http;
mod error;

use wasm_bindgen::prelude::*;
use log::log;
use crate::error::WasmError;
use crate::http::{HttpClient, Options};

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn client_send(opts: JsValue, is_form_submit: Option<bool>) -> Result<JsValue, JsValue> {
    if !opts.is_object() {
        return Err(JsValue::from_str(&WasmError::Error("`opts` is not a object !".to_string()).to_string()));
    }

    let options = serde_wasm_bindgen::from_value::<Options>(opts).map_err(|err| JsValue::from_str(&WasmError::Error(err.to_string()).to_string()))?;
    return HttpClient::send(options, is_form_submit).await;
}
