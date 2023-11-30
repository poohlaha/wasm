use js_sys::{Function, Promise};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AbortController, Request, RequestInit, Response, window};

use crate::{error::Error, request::HttpRequest};
use crate::client::call::Call;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = fetch)]
    fn fetch_with_request_and_init(input: &Request, init: &RequestInit) -> Promise;
}

#[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope)"]
#[doc = "[Github Example](https://github.com/mdn/dom-examples/tree/main/fetch)"]
fn js_fetch(timeout: Option<i32>, request: &Request, http_request: Option<HttpRequest>) -> Promise {
    let global = js_sys::global();
    let window = window();
    let init = http_request.clone().map(Into::into).unwrap_or_else(RequestInit::new);

    // timeout
    let mut timeout = Call::prepare_request_timeout(timeout);

    // abort controller
    let controller = AbortController::new();
    if let Err(err) = controller {
        return Promise::reject(&err);
    }

    let controller = controller.unwrap();
    if timeout != -1 {
        let signal = controller.signal();
        timeout = timeout * 1000;

        if let Some(http_request) = http_request.clone() {
            http_request.signal(Some(&signal));
        }
    }

    let controller_closure = Closure::wrap(Box::new(move || {
        // 在这里执行超时时的操作
        println!("Timeout reached!");

        // 在超时时调用 controller.abort() 方法
        controller.abort();
    }) as Box<dyn FnMut()>);

    let callback = Function::from(controller_closure.as_ref().clone());

    // 释放闭包，避免内存泄漏
    controller_closure.forget();

    if let Ok(true) = js_sys::Reflect::has(&global, &JsValue::from_str("ServiceWorkerGlobalScope")) {
        let service_worker_global_scope = global.unchecked_into::<web_sys::ServiceWorkerGlobalScope>();
        if timeout != -1 {
            let result = service_worker_global_scope.set_timeout_with_callback_and_timeout_and_arguments_0(&callback, timeout);
            if let Err(err) = result {
                return Promise::reject(&err);
            }
        }
        service_worker_global_scope.fetch_with_request_and_init(request, &init)
    } else {
        if let Some(window) = window {
            let result = window.set_timeout_with_callback_and_timeout_and_arguments_0(&callback, timeout);
            if let Err(err) = result {
                return Promise::reject(&err);
            }
        }
        fetch_with_request_and_init(request, &init)
    }
}


pub async fn fetch(timeout: Option<i32>, request: &Request, http_request: Option<HttpRequest>) -> Result<Response, Error> {
    let js_response = JsFuture::from(js_fetch(timeout, request, http_request)).await.map_err(Error::js_error)?;
    Ok(js_response.unchecked_into())
}
