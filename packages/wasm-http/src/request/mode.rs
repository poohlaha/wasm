/*!
    `Request` 接口的 `mode` 只读属性包含请求的模式（例如：`cors` 、 `no-cors` 、 `cors-with-forced-preflight` 、 `same-origin` 或 `navigate` 。）
    这用于确定跨域请求是否能得到有效的响应，以及响应的哪些属性是可读的。
*/

use web_sys::RequestMode;

#[doc = "https://developer.mozilla.org/en-US/docs/Web/API/Request/mode"]
#[derive(Debug, Clone, Copy, Default)]
pub enum Mode {

    /// 如果使用此模式向另外一个源发送请求，显而易见，结果会是一个错误。你可以设置该模式以确保请求总是向当前的源发起的
    SameOrigin,

    /**
        允许跨域请求，例如访问第三方供应商提供的各种 `API`。
        预期将会遵守 `CORS protocol` 。
        仅有有限部分的头部暴露在 `Response` ，但是 `body` 部分是可读的。
    */
    Cors,

    /**
        保证请求对应的 `method` 只有 `HEAD`，`GET` 或 `POST` 方法，并且请求的 `headers` 只能有简单请求头 (simple headers)。
        如果 `ServiceWorker` 劫持了此类请求，除了 `simple header` 之外，不能添加或修改其他 `header`。
        另外 `JavaScript` 不会读取 `Response` 的任何属性。这样将会确保 `ServiceWorker` 不会影响 Web 语义 (semantics of the Web)，同时保证了在跨域时不会发生安全和隐私泄露的问题。
     */
    #[default]
    NoCors,

    /**
        表示这是一个浏览器的页面切换请求 (`request`)。
        `navigate` 请求仅在浏览器切换页面时创建，该请求应该返回 `HTML`。
    */
    Navigate,
}

impl From<Mode> for RequestMode {
    fn from(value: Mode) -> Self {
        match value {
            Mode::SameOrigin => RequestMode::SameOrigin,
            Mode::Cors => RequestMode::Cors,
            Mode::NoCors => RequestMode::NoCors,
            Mode::Navigate => RequestMode::Navigate,
        }
    }
}

impl Mode {
    pub fn get_mode(mode: String) -> Mode {
        let mode = mode.trim();
        if mode.is_empty() {
            return Mode::NoCors;
        }

        return match mode.to_lowercase().as_str() {
            "same-origin" => Mode::SameOrigin,
            "cors" => Mode::Cors,
            "navigate" => Mode::Navigate,
            _ => Mode::NoCors,
        };
    }
}