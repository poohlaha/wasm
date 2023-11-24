use crate::error::WasmError;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, Utc};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DateHandler;

#[wasm_bindgen]
impl DateHandler {
    /// 获取默认的格式
    fn get_default_format(format: Option<String>, default_format: &str) -> String {
        let mut date_format = default_format.to_string();
        if format.is_some() {
            date_format = format.unwrap().to_string();
        }

        date_format
    }

    /**
      格式化日期:
        - %Y: 表示四位数的年份，例如 2022。
        - %y: 表示两位数的年份，范围是 00 到 99。
        - %m: 表示两位数的月份，范围是 01 到 12。
        - %_m: 表示不补零的月份，范围是 1 到 12
        - %d: 表示两位数的日期，范围是 01 到 31。
        - %e: 表示两位数的日期，范围是 1 到 31。
        - %H: 表示两位数的小时，范围是 00 到 23。
        - %I: 表示两位数的小时，范围是 00 到 12。
        - %k: 表示小时，不补零，范围是 0 到 23。
        - %M: 表示两位数的分钟，范围是 00 到 59。
        - %S: 表示两位数的秒数，范围是 00 到 59。
        - %S: 表示两位数的秒数，范围是 00 到 59。

        - %a: 缩写的星期几名称, 如：Sun、Mon、Tue
        - %b: 缩写的月份名称, 如：Jan、Feb、Mar
        - %e: 日期(1 到 31), 不补零
        - %T: 时间的 24 小时制表示，格式为 HH:MM:SS
        - %A: 完整的星期几名称
        - %B: 完整的月份名称
        - %E: 日期(1 到 31), 不补零
        - %p: 表示上午或下午(AM 或 PM)
        - %Z: 表示时区缩写，如 CST 表示中国标准时间
        - %z: 表示时区偏移，如 +0800 表示东八区，也就是相对于 UTC 的偏移时间

        例:
          - %Y-%m-%d %H:%M:%S => 2014-11-28 12:00:09
          - %a %b %e %T %Y => Fri Nov 28 12:00:09 2014
          - %a %b %e %I:%M:%S %Y => Fri Nov 28 00:00:09 2014
          - %A %e %B %Y, %T => Tuesday 14 February 2023, 17:23:35

        date: 需要输入的日期字符串
        old_format: 原来的格式, 默认为 `%Y-%m-%d %H:%M:%S`
        format: 需要的格式, 默认为 `%Y-%m-%d`
    */
    pub fn format(
        date: &str,
        format: Option<String>,
        old_format: Option<String>,
    ) -> Result<String, JsValue> {
        if date.is_empty() {
            return Err(JsValue::from_str(
                &WasmError::Empty("date".to_string()).to_string(),
            ));
        }

        let old_date_format = Self::get_default_format(old_format.clone(), "%Y-%m-%d %H:%M:%S");
        let date_format = Self::get_default_format(format.clone(), "%Y-%m-%d");
        Self::format_date(date, old_date_format, date_format)
    }

    /**
      格式化 UTC 时间
    */
    fn format_date(
        date: &str,
        old_date_format: String,
        date_format: String,
    ) -> Result<String, JsValue> {
        // Utc
        let date_time = date.parse::<DateTime<Utc>>().ok();
        if date_time.is_some() {
            let date_time = date_time.unwrap();
            return Ok(date_time.format(&date_format).to_string());
        }

        // FixedOffset
        let date_time = date.parse::<DateTime<FixedOffset>>().ok();
        if date_time.is_some() {
            let date_time = date_time.unwrap();
            return Ok(date_time.format(&date_format).to_string());
        }

        let date_time = DateTime::parse_from_rfc2822(date).ok();
        if date_time.is_some() {
            let date_time = date_time.unwrap();
            return Ok(date_time.format(&date_format).to_string());
        }

        let date_time = DateTime::parse_from_rfc3339(date).ok();
        if date_time.is_some() {
            let date_time = date_time.unwrap();
            return Ok(date_time.format(&date_format).to_string());
        }

        // 带旧格式的
        if !old_date_format.is_empty() {
            let date_time = DateTime::parse_from_str(date, &old_date_format).ok();
            if date_time.is_some() {
                let date_time = date_time.unwrap();
                return Ok(date_time.format(&date_format).to_string());
            }

            let date_time = NaiveDateTime::parse_from_str(date, &old_date_format).ok();
            if date_time.is_some() {
                let date_time = date_time.unwrap();
                return Ok(date_time.format(&date_format).to_string());
            }
        }

        Err(JsValue::from_str(
            &WasmError::Error(
                "invalid date, please input correct field `old_date_format` !".to_string(),
            )
            .to_string(),
        ))
    }

    /**
     根据时间戳获取时间
      date: 时间戳
      format: 需要的格式, 默认为 `%Y-%m-%d`
    */
    pub fn get_date_by_timestamp(
        timestamp: u64,
        format: Option<String>,
    ) -> Result<String, JsValue> {
        let seconds = timestamp / 1000;
        let time = NaiveDateTime::from_timestamp_opt(seconds as i64, 0);
        if time.is_none() {
            return Err(JsValue::from_str(
                &WasmError::Error("`timestamp` is invalid !".to_string()).to_string(),
            ));
        }

        let time = time.unwrap();
        let date_time = DateTime::<Local>::from_naive_utc_and_offset(time, *Local::now().offset());
        let date_format = Self::get_default_format(format, "%Y-%m-%d");
        Ok(date_time.format(&date_format).to_string())
    }

    /**
      获取当前时间
      format: 需要的格式, 默认为 `%Y-%m-%d`
    */
    pub fn get_current_date(format: Option<String>) -> String {
        let current_date = Local::now();
        if format.is_none() {
            return current_date.format("%Y-%m-%d").to_string();
        }

        let format = format.unwrap();
        current_date.format(format.as_str()).to_string()
    }
}
