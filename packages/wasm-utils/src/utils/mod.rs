/*!
    公共 Utils 类
*/
use crate::error::WasmError;
use js_sys::{Array, Object, JSON};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct UtilsHandler;

#[wasm_bindgen]
impl UtilsHandler {
    /**
      生成 UUID
    */
    pub fn generate_uuid() -> String {
        Uuid::new_v4().to_string()
    }

    /**
      字符串是否为空
    */
    pub fn is_blank(str: &str) -> bool {
        str.trim().to_string().is_empty()
    }

    fn format(part: String) -> String {
        let formatted = part.chars().enumerate().fold(String::new(), |acc, (i, c)| {
            if i > 0 && i % 3 == 0 {
                format!("{},{}", acc, c)
            } else {
                format!("{}{}", acc, c)
            }
        });

        formatted.chars().rev().collect::<String>()
    }

    /**
      格式化 整数 为 字符串, 通过三位一个 `,` 连接
    */
    pub fn format_integer(num: i64) -> String {
        let str = format!("{}", num);
        let part = str.chars().rev().collect::<String>();
        Self::format(part)
    }

    /**
      格式化 小数数字 为 字符串, 通过三位一个 `,` 连接, digit 为四舍五入
    */
    pub fn format_float(num: f64, digit: Option<u32>) -> String {
        let str;
        if digit.is_none() {
            str = format!("{}", num);
        } else {
            let digit = digit.unwrap();
            str = format!("{:.*}", digit as usize, num)
        }

        let parts: Vec<&str> = str.split(".").collect();
        let integer_part = parts[0].chars().rev().collect::<String>();
        let formatted = Self::format(integer_part);
        let result = if parts.len() > 1 {
            format!("{}.{}", formatted, parts[1])
        } else {
            formatted
        };

        result
    }

    /**
     深拷贝
    */
    pub fn deep_copy(value: JsValue) -> Result<JsValue, JsValue> {
        // array
        if let Some(arr) = value.dyn_ref::<Array>() {
            let value = JSON::stringify(arr).map_err(|_| {
                JsValue::from_str(&WasmError::Error("deep copy error !".to_string()).to_string())
            })?;

            let value = String::from(value);
            let array = JSON::parse(&value).map_err(|_| {
                JsValue::from_str(&WasmError::Error("deep copy error !".to_string()).to_string())
            })?;

            return Ok(array);
        }

        // object
        if let Some(obj) = value.dyn_ref::<Object>() {
            let object = Object::new();
            let keys = Object::keys(&obj);
            for key in keys.iter() {
                let value = js_sys::Reflect::get(&obj, &key).map_err(|_| {
                    JsValue::from_str(
                        &WasmError::Error("deep copy error !".to_string()).to_string(),
                    )
                })?;

                js_sys::Reflect::set(&object, &key, &value).map_err(|_| {
                    JsValue::from_str(
                        &WasmError::Error("deep copy error !".to_string()).to_string(),
                    )
                })?;
            }

            return Ok(JsValue::from(object));
        }

        Ok(value.clone())
    }

    /**
     首字母转大写
    */
    pub fn capitalize_first_char(str: &str) -> String {
        if str.is_empty() {
            return String::new();
        }

        let chars = str.chars().enumerate().fold(String::new(), |acc, (i, c)| {
            if i == 0 {
                format!("{}{}", acc, c.to_uppercase())
            } else {
                format!("{}{}", acc, c)
            }
        });

        return chars;
    }

    /**
      驼峰转换下划线
      str: 要转换的字符串
      spec: 字符, 默认为 `_`
    */
    pub fn hump_with_line(str: &str, spec: Option<char>) -> String {
        let mut underline = '_';
        if spec.is_some() {
            let spec = spec.unwrap();
            underline = spec;
        }

        let mut chars = String::new();
        for (_, c) in str.chars().enumerate() {
            if c.is_uppercase() {
                chars.push(underline);
                chars.push(c.to_lowercase().next().unwrap());
            } else {
                chars.push(c)
            }
        }

        chars
    }

    /**
     格式化手机号码
    */
    pub fn format_phone(phone: &str, spec: Option<char>) -> Result<String, JsValue> {
        if phone.len() < 11 {
            return Err(JsValue::from_str(
                &WasmError::Error("`phone` is invalid !".to_string()).to_string(),
            ));
        }

        let mut reg_spec = ' ';
        if spec.is_some() {
            let spec = spec.unwrap();
            reg_spec = spec;
        }

        let format = format!(
            "{}{}{}{}{}",
            &phone[0..3],
            reg_spec,
            &phone[3..7],
            reg_spec,
            &phone[7..]
        );
        Ok(format)
    }
}
