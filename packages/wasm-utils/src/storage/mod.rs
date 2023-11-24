//! 数据存储

use crate::error::WasmError;
use crate::signature::SignatureHandler;
use crate::{get_html_document, get_window};
use js_sys::{JsString, JSON};
use wasm_bindgen::prelude::*;
use web_sys::Storage;

#[wasm_bindgen]
pub struct StorageHandler;

#[wasm_bindgen]
impl StorageHandler {
    /// 获取 `window` 下的 `LocalStorage`
    fn get_storage(name: &str) -> Result<Storage, JsValue> {
        let window = get_window()?;
        if name == "sessionStorage" {
            let storage = window.session_storage()?.ok_or_else(|| {
                let msg = format!("get {} error !", name);
                JsValue::from_str(&WasmError::Error(msg).to_string())
            })?;

            return Ok(storage);
        }

        let storage = window.local_storage()?.ok_or_else(|| {
            let msg = format!("get {} error !", name);
            JsValue::from_str(&WasmError::Error(msg).to_string())
        })?;
        return Ok(storage);
    }

    /// 加密数据
    fn encode_value(item: JsValue) -> Result<String, JsValue> {
        if item.is_null() {
            return Ok(String::new());
        }

        let value;
        if item.is_object() {
            let str = JSON::stringify(&item).map_err(|err| err)?;
            value = String::from(str);
        } else {
            value = String::from(JsString::from(item));
        }

        if value.is_empty() {
            return Ok(String::new());
        }

        Ok(SignatureHandler::encode(&value))
    }

    fn decode_value(item: JsValue) -> Result<JsValue, JsValue> {
        if item.is_null() {
            return Ok(JsValue::from_str(""));
        }

        let item = JsString::from(item);
        let item = String::from(item);
        let item = SignatureHandler::decode(&item)?;
        let str = JSON::parse(&item).ok();
        if str.is_none() {
            return Ok(JsValue::from_str(""));
        }

        let str = str.ok_or_else(|| JsValue::from_str(""))?;
        Ok(str)
    }

    fn set(name: &str, item: JsValue, storage_name: &str) -> Result<bool, JsValue> {
        let storage = Self::get_storage(storage_name)?;
        let value = Self::encode_value(item)?; // 加密 value
        let result = js_sys::Reflect::set(
            &storage,
            &JsValue::from_str(name),
            &JsValue::from_str(&value),
        )
        .map_err(|err| err)?;
        Ok(result)
    }

    fn get(name: &str, storage_name: &str) -> Result<JsValue, JsValue> {
        let storage = Self::get_storage(storage_name)?;
        let value = js_sys::Reflect::get(&storage, &JsValue::from_str(name)).map_err(|err| err)?;
        let value = Self::decode_value(value)?; // 解密 value
        Ok(value)
    }

    fn clear(storage_name: &str) -> Result<bool, JsValue> {
        let storage = Self::get_storage(storage_name)?;
        storage.clear()?;
        Ok(true)
    }

    /**
     存储到 `LocalStorage`
    */
    pub fn set_local(name: &str, item: JsValue) -> Result<bool, JsValue> {
        Self::set(name, item, "localStorage")
    }

    /**
     从 `LocalStorage` 中取值
    */
    pub fn get_local(name: &str) -> Result<JsValue, JsValue> {
        Self::get(name, "localStorage")
    }

    /**
     清空 `LocalStorage` 中取值
    */
    pub fn clear_local() -> Result<bool, JsValue> {
        Self::clear("localStorage")
    }

    /**
    存储到 `SessionStorage`
     */
    pub fn set_session(name: &str, item: JsValue) -> Result<bool, JsValue> {
        Self::set(name, item, "sessionStorage")
    }

    /**
    从 `SessionStorage` 中取值
     */
    pub fn get_session(name: &str) -> Result<JsValue, JsValue> {
        Self::get(name, "sessionStorage")
    }

    /**
    清空 `SessionStorage` 中取值
     */
    pub fn clear_session() -> Result<bool, JsValue> {
        Self::clear("sessionStorage")
    }

    /**
    存储到 `Cookie`
     */
    pub fn set_cookie(name: &str, item: JsValue, expires: Option<u64>) -> Result<bool, JsValue> {
        let html_document = get_html_document()?;
        let value = Self::encode_value(item)?; // 加密 value
        let mut value = format!("{}={}", name, &value);

        if let Some(expires) = expires {
            let expiration_date = js_sys::Date::new(&JsValue::from_f64(expires as f64 * 1000.0));
            value.push_str(&format!("; expires={}", expiration_date.to_utc_string()));
        }

        html_document.set_cookie(&value)?;
        Ok(true)
    }

    /**
    从 `Cookie` 中取值
     */
    pub fn get_cookie(name: &str) -> Result<JsValue, JsValue> {
        let html_document = get_html_document()?;
        let cookie = html_document.cookie()?;
        let cookie = cookie.split(";").find(|part| {
            let mut iter = part.trim().split('=');
            if let Some(cookie_name) = iter.next() {
                cookie_name == name
            } else {
                false
            }
        });

        let value = cookie.map(|c| {
            c.trim_start_matches(name)
                .trim_start_matches('=')
                .to_string()
        });
        if let Some(value) = value {
            let value = value.replace(&format!("{}=", name), "");
            let value = Self::decode_value(JsValue::from_str(&value.trim()))?;
            return Ok(value);
        }

        return Ok(JsValue::from_str(""));
    }

    /**
    清空 `Cookie` 中取值
     */
    pub fn clear_cookie() -> Result<bool, JsValue> {
        let html_document = get_html_document()?;
        let cookie = html_document.cookie()?;
        let cookies: Vec<&str> = cookie.split(";").collect();

        // 遍历 cookie 数组，将每个 cookie 的过期时间设置为过去的日期
        for cookie in cookies {
            let cookie = cookie.trim();
            let parts: Vec<&str> = cookie.split('=').collect();
            if parts.len() > 1 {
                let name = parts[0];
                let expired_date = "Thu, 01 Jan 1970 00:00:00 GMT";
                let value = format!("{}=; expires={}", name, expired_date);
                html_document.set_cookie(&value)?;
            }
        }

        Ok(true)
    }
}
