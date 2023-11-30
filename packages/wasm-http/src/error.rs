//! custom error

use js_sys::Object;
use thiserror::Error;
use wasm_bindgen::{JsCast, JsValue};

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Error(String),

    /// JS API error
    #[error("js api error: {0}")]
    JsError(String),

    #[error("http error")]
    HttpError(#[from] http::Error),

    /// Missing response body in HTTP call
    #[error("missing response body in HTTP call")]
    MissingResponseBody,
}

impl Error {
    pub(crate) fn js_error(value: JsValue) -> Self {
        let message = js_object_display(&value);
        Self::JsError(message)
    }
}

fn js_object_display(option: &JsValue) -> String {
    let object: &Object = option.unchecked_ref();
    ToString::to_string(&object.to_string())
}
